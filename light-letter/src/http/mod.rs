use std::sync::Arc;
use std::cell::RefCell;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::convert::Infallible;
use tokio::sync::oneshot;
use regex::Regex;
use hyper::{Body, Request, Response, Server as HttpServer};
use hyper::service::{make_service_fn, service_fn};
use http::header::*;
use light_letter_rpc::SiteState;

mod blog;
mod res_utils;
mod error;

pub(crate) struct Server {
    addrs: Vec<SocketAddr>,
    rx: Vec<oneshot::Receiver<()>>,
    site_states: Vec<SiteState>,
}

struct SiteStatesWrapper {
    s: &'static Vec<SiteState>,
}

impl Drop for SiteStatesWrapper {
    fn drop(&mut self) {
        unsafe { Box::from_raw(self.s as *const Vec<SiteState> as *mut Vec<SiteState>); }
    }
}

async fn serve_blog(req: Request<Body>, site_state: &'static SiteState) -> Result<Response<Body>, Infallible> {
    debug!("Requested {:?} {:?}, matched blog site {:?}", req.method(), req.uri(), site_state.host);
    let (req, body) = req.into_parts();
    let path = req.uri.path();
    let dir = &site_state.dir;
    if path == "/favicon.ico" {
        Ok(res_utils::file(&req, dir, "favicon.ico").await)
    } else {
        let mut p = path[1..].splitn(2, '/');
        let scope = p.next().unwrap_or("");
        let sub_path = p.next().unwrap_or("");
        let ret = match scope {
            "files" => res_utils::file(&req, &dir.join("files"), sub_path).await,
            "static" => blog::static_resource(&req, sub_path, &site_state.initialization_time),
            "rpc" => {
                let sub_path = format!("/{}", sub_path);
                blog::rpc(site_state, req, body, sub_path).await
            },
            "backstage" => blog::backstage_page(site_state, req).await,
            _ => blog::page(site_state, req).await,
        };
        Ok(ret)
    }
}

async fn serve_static(req: Request<Body>, site_state: &'static SiteState) -> Result<Response<Body>, Infallible> {
    debug!("Requested {:?} {:?}, matched static site {:?}", req.method(), req.uri(), site_state.host);
    let req = req.into_parts().0;
    let path = &req.uri.path()[1..];
    let dir = &site_state.dir;
    let ret = res_utils::file(&req, &dir.join("static"), path).await;
    Ok(ret)
}

async fn serve(req: Request<Body>, site_states: &'static Vec<SiteState>) -> Result<Response<Body>, Infallible> {
    let host = req.headers().get(HOST).map(|x| x.to_str().unwrap_or("")).unwrap_or("");

    if let Some(site_state) = site_states.iter().find(|x| {
        x.host == host
    }) {
        let site_type = site_state.config.r#type.clone();
        return match site_type.as_str() {
            "blog" => serve_blog(req, site_state).await,
            "static" => serve_static(req, site_state).await,
            _ => unreachable!()
        };
    }

    if let Some(site_state) = site_states.iter().find(|x| {
        x.host_aliases.iter().position(|x| {
            *x == host
        }).is_some()
    }) {
        let uri = req.uri();
        let query = uri.query().unwrap_or("");
        let location = format!("//{}{}{}{}", site_state.host, uri.path(), if query.len() > 0 { "?" } else { "" }, query);
        debug!("Requested {:?} {:?}, redirecting {:?}", req.method(), req.uri(), location);
        let response = res_utils::redirect(&req.into_parts().0, &location);
        return Ok(response);
    }

    warn!("Requested {:?} {:?}, no site matched", req.method(), req.uri());
    let response = res_utils::not_found(&req.into_parts().0);
    Ok(response)
}

impl Server {
    pub(crate) fn new_with_close_handler(sites_root: &Path, config: &light_letter_rpc::SitesConfig) -> (Self, CloseHandler) {
        // check config and initialize dir structure for each site
        let site_states: Vec<SiteState> = light_letter_rpc::SiteState::from_sites_config(config, sites_root);

        let ip = &config.net.ip;
        let addrs: Vec<_> = config.net.port.iter().map(|port| {
            SocketAddr::from((ip.clone(), *port))
        }).collect();

        let (tx, rx) = config.net.port.iter().map(|_| {
            tokio::sync::oneshot::channel::<()>()
        }).unzip();

        (Self {
            addrs,
            rx,
            site_states,
        }, CloseHandler {
            tx
        })
    }

    pub(crate) async fn run_async(self) {
        let Self {mut addrs, mut rx, site_states} = self;
        let site_states = Arc::new(site_states);

        thread_local! {
            static SITE_STATES_WRAPPER: RefCell<Option<SiteStatesWrapper>> = RefCell::new(None);
        }

        let graceful: Vec<_> = addrs.iter_mut().zip(rx.iter_mut()).map(|(addr, rx)| {
            let site_states = site_states.clone();
            let make_svc = make_service_fn(move |_conn| {
                let site_states = site_states.clone();
                async move {
                    Ok::<_, Infallible>(service_fn(move |req| {
                        let site_states: &'static Vec<SiteState> = SITE_STATES_WRAPPER.with(|ssw| {
                            let mut ssw = ssw.borrow_mut();
                            if let Some(ssw) = ssw.as_ref() {
                                let site_states: &'static Vec<SiteState> = ssw.s;
                                site_states
                            } else {
                                *ssw = Some(SiteStatesWrapper { s: Box::leak(Box::new(site_states.clone())) });
                                let site_states: &'static Vec<SiteState> = ssw.as_ref().unwrap().s;
                                site_states
                            }
                        });
                        serve(req, site_states)
                    }))
                }
            });

            let server = HttpServer::bind(&addr).serve(make_svc);
            let graceful = server
                .with_graceful_shutdown(async {
                    rx.await.ok();
                });
            graceful
        }).collect();

        let all_services = futures::future::join_all(graceful);
        for r in all_services.await {
            r.unwrap();
        }
    }

    pub(crate) fn addrs(&self) -> &Vec<SocketAddr> {
        &self.addrs
    }
}

pub(crate) struct CloseHandler {
    tx: Vec<oneshot::Sender<()>>,
}

impl CloseHandler {
    pub(crate) fn close(self) {
        for tx in self.tx {
            tx.send(()).unwrap();
        }
    }
}
