use actix_web::web;

mod student;

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
