use std::net::SocketAddr;
use std::path::{Path, PathBuf, Component};
use actix_web::{web, middleware, http, App, HttpRequest, HttpResponse, HttpServer, guard};
use actix_files::NamedFile;
use failure::Fail;
use regex::Regex;

pub(crate) struct Server {
    addrs: Vec<SocketAddr>,
    http_server: actix_web::dev::Server,
}

#[derive(Clone, Debug)]
pub(crate) struct SiteState {
    host: &'static str,
    host_aliases: Vec<&'static str>,
    dir: PathBuf,
    config: crate::SiteConfig,
}

#[derive(Fail, Debug)]
enum HttpError {
    #[fail(display = "Not Found")]
    NotFound,
    #[fail(display = "")]
    Forbidden,
}

impl actix_web::error::ResponseError for HttpError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            HttpError::NotFound => HttpResponse::new(http::StatusCode::NOT_FOUND),
            HttpError::Forbidden => HttpResponse::new(http::StatusCode::FORBIDDEN),
        }
    }
}

macro_rules! serve_dir {
    ($rule: ident, $path: expr) => {
        async fn $rule(site_state: web::Data<SiteState>, req: HttpRequest) -> actix_web::Result<NamedFile, HttpError> {
            let path: PathBuf = req.match_info().query("filename").parse().unwrap();
            for slice in path.components() {
                if let Component::Normal(_) = slice {
                    // empty
                } else {
                    return Err(HttpError::Forbidden)
                }
            }
            let path = site_state.dir.join($path).join(path);
            if path.is_dir() {
                let index_path = path.join("index.html");
                NamedFile::open(index_path.as_path()).map_err(|_| HttpError::NotFound)
            } else {
                NamedFile::open(path).map_err(|_| HttpError::NotFound)
            }
        }
    };
}
serve_dir!(serve_static, "static");
serve_dir!(serve_files, "files");

fn host_alias_redirect(site_state: web::Data<SiteState>, req: HttpRequest) -> HttpResponse {
    let ori_uri = req.uri();
    let uri = format!("//{}{}", site_state.host, ori_uri.path_and_query().map(|x| x.to_string()).unwrap_or("".into()));
    HttpResponse::Found()
        .header(http::header::LOCATION, uri)
        .finish()
        .into_body()
}

impl Server {
    pub(crate) fn new(sites_root: &Path, config: &crate::SitesConfig) -> Self {
        let sites_root = sites_root.to_owned();

        // check config and initialize dir structure for each site
        let site_states: Vec<SiteState> = config.site.iter().map(|site_config| {
            lazy_static! {
                static ref NAME_RE: Regex = Regex::new(r#"^[-_0-9a-zA-Z]+$"#).unwrap();
            }
            if !NAME_RE.is_match(site_config.name.as_str()) {
                panic!(format!(r#"Illegal site name "{}" (site name should only contains letters, numbers, dashes, and underlines)."#, site_config.name))
            }
            let dir = sites_root.join("sites").join(site_config.name.as_str());
            fs_extra::dir::create_all(dir.as_path(), false).unwrap();
            let site_type = site_config.r#type.clone();
            match site_type.as_str() {
                "blog" => {
                    fs_extra::dir::create_all(dir.join("files").as_path(), false).unwrap();
                },
                "static" => {
                    fs_extra::dir::create_all(dir.join("static").as_path(), false).unwrap();
                },
                _ => panic!(format!(r#"Unrecognized site type "{}"."#, &site_type))
            };
            debug!(r#"Serve site {} for host "{}", aliases {:?}"#, site_config.name, site_config.host, site_config.alias.as_ref().unwrap_or(&vec![]));
            let host = Box::leak(site_config.host.clone().into_boxed_str()); // NOTE here might cause memory leak
            let host_aliases = site_config.alias.as_ref().unwrap_or(&vec![]).iter().map(|x| { // NOTE here might cause memory leak
                let s: &'static str = Box::leak(x.clone().into_boxed_str());
                s
            }).collect();
            let site_state = SiteState {
                host,
                host_aliases,
                dir,
                config: site_config.clone(),
            };
            site_state
        }).collect();

        // create http server
        let mut http_server = HttpServer::new(move || {
            let mut app = App::new()
                .wrap(middleware::Compress::default());
            for site_state in site_states.clone() {
                let scope = web::scope("")
                    .guard(guard::Header("Host", site_state.host))
                    .data(site_state.clone());
                let site_type = site_state.config.r#type.clone();
                let routes = match site_type.as_str() {
                    "blog" => {
                        let scope = scope.route("/files/{filename:.*}", web::get().to(serve_files));
                        let scope = super::blog::route(scope);
                        scope
                    },
                    "static" => {
                        scope.route("/{filename:.*}", web::get().to(serve_static))
                    },
                    _ => unreachable!()
                };
                app = app.service(routes);
                for host_alias in site_state.host_aliases.iter() {
                    let scope = web::scope("")
                        .guard(guard::Header("Host", host_alias))
                        .data(site_state.clone());
                    let routes = scope.route("/{filename:.*}", web::to(host_alias_redirect));
                    app = app.service(routes);
                }
            }
            app
        });
        for port in config.net.port.iter() {
            http_server = http_server.bind((config.net.ip.as_str(), *port)).unwrap();
        }

        Self {
            addrs: http_server.addrs(),
            http_server: http_server.run(),
        }
    }

    pub(crate) fn addrs(&self) -> &Vec<SocketAddr> {
        &self.addrs
    }

    pub(crate) fn stop(&mut self, graceful: bool) {
        futures::executor::block_on(self.http_server.stop(graceful));
    }
}
