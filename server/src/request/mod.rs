use actix_web::{web, HttpResponse};

use crate::error::BabibappError;

mod student;
mod teacher;
mod token;

type ActionResult = Result<HttpResponse, BabibappError>;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/student")
            .service(student::list_all)
            .service(student::get)
            .service(student::add)
            .service(student::delete),
    )
    .service(
        web::scope("/teacher")
            .service(teacher::list_all)
            .service(teacher::get)
            .service(teacher::add)
            .service(teacher::delete),
    )
    .service(
        web::scope("/token")
            .service(token::generate)
            .service(token::validate),
    );
}
