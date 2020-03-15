use std::rc::Rc;
use std::cell::Cell;
use std::pin::Pin;
use std::fmt;
use serde::{Serialize, Deserialize};
use web_sys::XmlHttpRequest;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use futures::task::Poll;

#[derive(Debug)]
pub enum RequestError {
    InvalidRequest(String),
    InvalidResponse(String),
    Custom(String),
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for RequestError {}

#[derive(Clone)]
pub struct RequestChannel {
    f: Rc<dyn 'static + Fn(&str, String) -> Pin<Box<dyn futures::Future<Output = Result<String, RequestError>>>>>,
}

impl RequestChannel {
    pub fn new<F: 'static + Fn(&str, String) -> Pin<Box<dyn futures::Future<Output = Result<String, RequestError>>>>>(f: F) -> Self {
        Self {
            f: Rc::new(f)
        }
    }
    pub(crate) async fn request<R: Serialize, S: for<'a> Deserialize<'a>>(&self, path: &str, data: &R) -> Result<S, RequestError> {
        let q = serde_json::to_string(data).map_err(|x| RequestError::InvalidRequest(x.to_string()))?;
        let ret = (self.f)(path, q).await?;
        serde_json::from_reader(ret.as_bytes()).map_err(|x| RequestError::InvalidResponse(x.to_string()))
    }
}

thread_local! {
    pub(crate) static CLIENT_REQUEST_CHANNEL: RequestChannel = {
        RequestChannel::new(|path, json| {
            let path = format!("/rpc{}", path);
            Box::pin({
                let xhr_resp = Rc::new(Cell::new(None));
                let xhr_resp2 = xhr_resp.clone();
                let waker = Rc::new(Cell::new(None));
                let waker2 = waker.clone();
                let ret = futures::future::poll_fn(move |context| {
                    match xhr_resp2.take() {
                        Some(resp) => Poll::Ready(resp),
                        None => {
                            waker2.set(Some(context.waker().clone()));
                            Poll::Pending
                        }
                    }
                });
                let xhr = XmlHttpRequest::new().unwrap();
                let xhr2 = xhr.clone();
                let path2 = path.clone();
                let cb = Closure::once_into_js(move || {
                    debug!("Receive XHR response from {:?}", path2);
                    let resp: Result<String, RequestError> = if xhr2.status() != Ok(200) {
                        Err(RequestError::Custom(xhr2.response_text().unwrap().unwrap_or_default()))
                    } else {
                        Ok(xhr2.response_text().unwrap().unwrap_or_default())
                    };
                    xhr_resp.set(Some(resp));
                    if let Some(waker) = waker.take() {
                        waker.wake();
                    }
                });
                xhr.add_event_listener_with_callback("load", cb.as_ref().unchecked_ref()).unwrap();
                xhr.open("POST", &path).unwrap();
                xhr.send_with_opt_str(Some(&json)).unwrap();
                debug!("Send XHR request to {:?}", path);
                ret
            })
        })
    };
}

pub(crate) fn client_request_channel() -> RequestChannel {
    CLIENT_REQUEST_CHANNEL.with(|x| x.clone())
}
