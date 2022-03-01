use actix_web::{web, HttpResponse};

use crate::error::BabibappError;

mod comment;
mod student;
mod teacher;
mod token;

type ActionResult = Result<HttpResponse, BabibappError>;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/token").configure(token::config))
        .service(web::scope("/student").configure(student::config))
        .service(web::scope("/teacher").configure(teacher::config))
        .service(web::scope("/comment").configure(comment::config));
}
