use actix_web::{web, HttpResponse};
use serde::Deserialize;

use crate::auth;
use crate::error::BabibappError;

mod student;

type ActionResult = Result<HttpResponse, BabibappError>;

#[derive(Debug, Deserialize)]
struct ActionTokenQuery {
    token: String,
}

impl ActionTokenQuery {
    pub fn validate(&self) -> Result<auth::Claims, BabibappError> {
        auth::validate_token(&self.token)
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/student")
            .service(student::list_all)
            .service(student::get)
            .service(student::add)
            .service(student::reset)
            .service(student::delete),
    );
}
