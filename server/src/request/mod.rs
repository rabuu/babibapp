use actix_web::{web, HttpResponse};

use crate::error::BabibappError;
use crate::settings::Settings;
use crate::DbPool;

mod comment;
mod student;
mod teacher;
mod token;

type RequestResult = Result<HttpResponse, BabibappError>;

#[derive(Clone)]
pub struct RequestContext {
    pub pool: DbPool,
    pub settings: Settings,
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/token").configure(token::config))
        .service(web::scope("/student").configure(student::config))
        .service(web::scope("/teacher").configure(teacher::config))
        .service(web::scope("/comment").configure(comment::config));
}
