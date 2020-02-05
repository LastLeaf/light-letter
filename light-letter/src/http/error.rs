use std::fmt;
use hyper::{Body, Response};
use http::header::*;

enum ErrorKind {
    BadRequest,
    Forbidden,
    InternalServerError,
}

pub(crate) struct Error {
    kind: ErrorKind,
    message: String,
}

impl Error {
    pub(crate) fn bad_request<T: fmt::Display>(message: T) -> Self {
        Self {
            kind: ErrorKind::BadRequest,
            message: message.to_string(),
        }
    }
    pub(crate) fn forbidden<T: fmt::Display>(message: T) -> Self {
        Self {
            kind: ErrorKind::Forbidden,
            message: message.to_string(),
        }
    }
    pub(crate) fn internal_server_error<T: fmt::Display>(message: T) -> Self {
        Self {
            kind: ErrorKind::BadRequest,
            message: message.to_string(),
        }
    }
    pub(crate) fn response(self) -> Response<Body> {
        let (st, msg) = match self.kind {
            ErrorKind::BadRequest => (400, "Bad Request"),
            ErrorKind::Forbidden => (403, "Forbidden"),
            ErrorKind::InternalServerError => (500, "Internal Server Error"),
        };
        error!("{}: {}", msg, self.message);
        Response::builder()
            .status(st)
            .header("X-Powered-By", "light-letter")
            .header(CACHE_CONTROL, "no-cache, no-store")
            .header(CONTENT_TYPE, "text/plain; charset=utf-8")
            .body(Body::from(msg))
            .unwrap()
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.message)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for Error { }
