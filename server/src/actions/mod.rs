use actix_web::web;

mod student;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/student")
            .service(student::list_all_students)
            .service(student::get_student_by_id)
            .service(student::add_student),
    );
}
