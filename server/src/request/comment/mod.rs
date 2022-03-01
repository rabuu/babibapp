use actix_web::web;

mod student_comment;
mod teacher_comment;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/student").configure(student_comment::config))
        .service(web::scope("/teacher").configure(teacher_comment::config));
}
