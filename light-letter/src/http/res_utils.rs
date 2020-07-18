use std::pin::Pin;
use std::path::{Path, Component};
use std::borrow::Cow;
use std::io::Write;
use hyper::{Body, Response};
use bytes::{Bytes, BytesMut};
use http::request::Parts;
use http::response::Builder;
use http::header::*;
use flate2::write::GzEncoder;
use flate2::Compression;
use futures::task::{Poll, Context};
use tokio::io::AsyncRead;
use chrono::{DateTime, Utc, FixedOffset, Duration};

use super::error::Error;

fn is_unmodified_since(if_modified_since: &str, modified: DateTime<Utc>) -> bool {
    let since = DateTime::<FixedOffset>::parse_from_rfc2822(if_modified_since);
    let ret = match since {
        Ok(t) => t >= (modified - Duration::seconds(1)).into(),
        Err(_) => false
    };
    ret
}

fn parse_accept_encoding(s: &str) -> &str {
    for slice in s.split(',') {
        let slice = slice.trim();
        let mut method = slice.splitn(2, ';');
        if method.next().unwrap_or("") == "gzip" {
            return "gzip";
        }
    }
    ""
}

fn common_builder<F: FnOnce(Builder) -> Builder>(req: &Parts, status: u16, body: Cow<'static, [u8]>, f: F) -> Result<Response<Body>, Error> {
    let mut builder = Response::builder()
        .status(status)
        .header("X-Powered-By", "light-letter")
        .header(VARY, "Accept-Encoding,Cookie");
    builder = f(builder);
    let accept_encoding = req.headers.get(ACCEPT_ENCODING).map(|x| x.to_str().unwrap_or("")).unwrap_or("");
    let body: Body = match parse_accept_encoding(accept_encoding) {
        "gzip" => {
            builder = builder.header(CONTENT_ENCODING, "gzip");
            let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
            encoder.write_all(&body).map_err(|e| Error::internal_server_error(e))?;
            encoder.try_finish().map_err(|e| Error::internal_server_error(e))?;
            encoder.finish().map_err(|e| Error::internal_server_error(e))?.into()
        },
        _ => Body::from(body.to_owned())
    };
    builder.body(body).map_err(|e| Error::internal_server_error(e))
}

pub(crate) fn internal_server_error(req: &Parts, message: String) -> Response<Body> {
    debug!("Response with internal server error for {:?}", req.uri.path());
    common_builder(req, 500, Cow::Owned(message.into_bytes()), |builder| {
        builder
            .header(CACHE_CONTROL, "no-cache, no-store")
            .header(CONTENT_TYPE, "text/plain; charset=utf-8")
    }).unwrap_or_else(|e| e.response())
}

pub(crate) fn not_found(req: &Parts) -> Response<Body> {
    debug!("Response with not found for {:?}", req.uri.path());
    common_builder(req, 404, Cow::Borrowed(b"Not Found"), |builder| {
        builder
            .header(CACHE_CONTROL, "no-cache, no-store")
            .header(CONTENT_TYPE, "text/plain; charset=utf-8")
    }).unwrap_or_else(|e| e.response())
}

pub(crate) fn redirect(req: &Parts, location: &str) -> Response<Body> {
    debug!("Response with redirect location for {:?}", req.uri.path());
    common_builder(req, 302, Cow::Borrowed(b""), |builder| {
        builder
            .header(CACHE_CONTROL, "no-cache, no-store")
            .header(LOCATION, location)
    }).unwrap_or_else(|e| e.response())
}

pub(crate) fn html_ok(req: &Parts, body: Cow<'static, [u8]>) -> Response<Body> {
    debug!("Response with HTML content for {:?}", req.uri.path());
    common_builder(req, 200, body, |builder| {
        builder
            .header(CACHE_CONTROL, "no-cache, no-store")
            .header(CONTENT_TYPE, "text/html; charset=utf-8")
    }).unwrap_or_else(|e| e.response())
}

pub(crate) fn cache_ok(req: &Parts, modified: &DateTime<Utc>, content_type: &str, body: Cow<'static, [u8]>) -> Response<Body> {
    let status_code = {
        let if_modified_since = req.headers.get(IF_MODIFIED_SINCE).map(|x| x.to_str().unwrap_or("")).unwrap_or("");
        if is_unmodified_since(if_modified_since, modified.clone()) { 304 } else { 200 }
    };
    let head_only = match req.method.as_str() {
        "GET" => status_code == 304,
        "HEAD" => true,
        _ => return Error::forbidden("Invalid Method").response()
    };
    let real_body = match head_only {
        false => body,
        true => Cow::Borrowed(b"" as &[u8]),
    };
    debug!("Response with cached static content for {:?}", req.uri.path());
    common_builder(req, if head_only { 304 } else { 200 }, real_body, |builder| {
        builder
            .header(CACHE_CONTROL, "max-age=0")
            .header(LAST_MODIFIED, modified.to_rfc2822().replace("+0000", "GMT"))
            .header(CONTENT_TYPE, content_type)
    }).unwrap_or_else(|e| e.response())
}

struct FileStream {
    file: Pin<Box<tokio::fs::File>>,
}

impl tokio::stream::Stream for FileStream {
    type Item = Result<Bytes, std::io::Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut bytes = BytesMut::with_capacity(65536);
        if let Poll::Ready(file_result) = self.file.as_mut().poll_read_buf(cx, &mut bytes) {
            match file_result {
                Ok(_) => {
                    if bytes.len() == 0 {
                        return Poll::Ready(None);
                    }
                    return Poll::Ready(Some(Ok(bytes.freeze())));
                }
                Err(err) => {
                    return Poll::Ready(Some(Err(err)));
                }
            }
        }
        Poll::Pending
    }
}

pub(crate) async fn file(req: &Parts, base: &Path, path: &str) -> Response<Body> {
    let path = Path::new(path);
    for slice in path.components() {
        if let Component::Normal(_) = slice {
            // empty
        } else {
            return not_found(req);
        }
    }
    let path = base.join(path);
    let metadata = match tokio::fs::metadata(&path).await {
        Ok(x) => x,
        Err(_) => return not_found(req),
    };
    let path = if metadata.is_dir() {
        path.join("index.html")
    } else {
        path
    };
    let modified = if let Ok(st) = metadata.modified() {
        let time_str: chrono::DateTime<chrono::Utc> = st.into();
        Some(time_str)
    } else {
        None
    };
    let status_code = {
        let if_modified_since = req.headers.get(IF_MODIFIED_SINCE).map(|x| x.to_str().unwrap_or("")).unwrap_or("");
        if let Some(modified) = modified {
            if is_unmodified_since(if_modified_since, modified.clone()) { 304 } else { 200 }
        } else {
            200
        }
    };
    let head_only = match req.method.as_str() {
        "GET" => status_code == 304,
        "HEAD" => true,
        _ => return Error::forbidden("Invalid Method").response()
    };
    debug!("Response with file {:?}", path);
    let mut builder = Response::builder()
        .status(status_code)
        .header("X-Powered-By", "light-letter")
        .header(VARY, "Accept-Encoding,Cookie")
        .header(CACHE_CONTROL, "max-age=0");
    if let Some(modified) = modified {
        builder = builder.header(LAST_MODIFIED, modified.to_rfc2822().replace("+0000", "GMT"));
    }
    let mime = mime_guess::from_path(&path).first();
    if let Some(mime) = &mime {
        builder = builder.header(CONTENT_TYPE, mime.to_string());
    }
    let use_gzip = if let Some(mime) = mime {
        // TODO impl real stream in gzip mode and remove size limit
        if mime.type_() == "text" && metadata.len() <= 4 * 1024 * 1024 {
            let accept_encoding = req.headers.get(ACCEPT_ENCODING).map(|x| x.to_str().unwrap_or("")).unwrap_or("");
            parse_accept_encoding(accept_encoding) == "gzip"
        } else {
            false
        }
    } else {
        builder = builder.header(CONTENT_LENGTH, metadata.len());
        false
    };
    if head_only {
        builder.body(Body::empty()).unwrap_or_else(|e| Error::internal_server_error(e).response())
    } else {
        let body = if use_gzip {
            builder = builder.header(CONTENT_ENCODING, "gzip");
            let file_content = match tokio::fs::read(&path).await {
                Ok(x) => x,
                Err(_) => return not_found(req),
            };
            let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
            if let Err(e) = encoder.write_all(&file_content) {
                return Error::internal_server_error(e).response();
            }
            if let Err(e) = encoder.try_finish() {
                return Error::internal_server_error(e).response();
            }
            match encoder.finish() {
                Ok(x) => x.into(),
                Err(e) => return Error::internal_server_error(e).response(),
            }
        } else {
            let file = match tokio::fs::File::open(&path).await {
                Ok(x) => x,
                Err(_) => return not_found(req),
            };
            let file_stream = FileStream {
                file: Box::pin(file),
            };
            Body::wrap_stream(file_stream)
        };
        builder.body(body).unwrap_or_else(|e| Error::internal_server_error(e).response())
    }
}
