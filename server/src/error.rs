use std::fmt;

use actix_web::{http::StatusCode, HttpResponse};
use anyhow::anyhow;
use serde::Serialize;

#[derive(Serialize)]
struct ApiError {
    error: &'static str,
}

pub struct BabibappError {
    pub msg: Option<&'static str>,
    pub inner: anyhow::Error,
}

impl BabibappError {
    pub fn from_msg(msg: &'static str) -> Self {
        let inner = anyhow!("{}", msg);
        BabibappError {
            msg: Some(msg),
            inner,
        }
    }
}

impl<T> From<T> for BabibappError
where
    T: Into<anyhow::Error>,
{
    fn from(t: T) -> Self {
        BabibappError {
            msg: None,
            inner: t.into(),
        }
    }
}

impl fmt::Debug for BabibappError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BabibappError")
            .field("msg", &self.msg)
            .field("inner", &self.inner)
            .finish()
    }
}

impl fmt::Display for BabibappError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(msg) = self.msg {
            write!(f, "{}: ", msg)?;
        }
        writeln!(f, "{}", self.inner)
    }
}

impl actix_web::error::ResponseError for BabibappError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self.inner.downcast_ref::<diesel::result::Error>() {
            Some(diesel::result::Error::NotFound) => StatusCode::NOT_FOUND,
            _ => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        if let Some(msg) = &self.msg {
            HttpResponse::build(self.status_code()).json(ApiError { error: msg })
        } else {
            HttpResponse::build(self.status_code())
                .content_type("text/plain")
                .body(self.inner.to_string())
        }
    }
}
