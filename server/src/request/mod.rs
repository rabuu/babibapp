use actix_web::{web, HttpResponse};

use crate::error::BabibappError;

mod student;
mod token;

type ActionResult = Result<HttpResponse, BabibappError>;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/student")
            .service(student::list_all)
            .service(student::get)
            .service(student::add)
            .service(student::reset)
            .service(student::delete),
    )
    .service(
        web::scope("/token")
            .service(token::generate)
            .service(token::validate),
    );
}
