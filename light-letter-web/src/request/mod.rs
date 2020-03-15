use std::rc::Rc;
use std::pin::Pin;
use std::fmt;
use serde::{Serialize, Deserialize};

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
            Box::pin(async {
                // TODO
                Ok(json)
            })
        })
    };
}

pub(crate) fn client_request_channel() -> RequestChannel {
    CLIENT_REQUEST_CHANNEL.with(|x| x.clone())
}
