use actix_web::{get, post, web, HttpResponse};
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};

use crate::db;
use crate::error::BabibappError;
use crate::models;
use crate::DbPool;

type ActionResult = Result<HttpResponse, BabibappError>;

#[get("/list_all_students")]
pub async fn list_all_students(pool: web::Data<DbPool>) -> ActionResult {
    let students = db::blocked_access(&pool, |conn| {
        use crate::schema::students::table;
        let list = table.load::<models::Student>(conn)?;
        Ok(list) as Result<Vec<models::Student>, BabibappError>
    })
    .await??;

    log::debug!("Database response: {:?}", students);

    Ok(HttpResponse::Ok().json(students))
}

#[get("/get_student_by_id/{student_id}")]
pub async fn get_student_by_id(
    pool: web::Data<DbPool>,
    student_id: web::Path<i32>,
) -> ActionResult {
    let student_id = student_id.into_inner();

    let student = db::blocked_access(&pool, move |conn| {
        use crate::schema::students::dsl::*;

        let student = students
            .filter(id.eq(student_id))
            .first::<models::Student>(conn)
            .optional()?;

        Ok(student) as Result<Option<models::Student>, BabibappError>
    })
    .await??;

    log::debug!("Database response: {:?}", student);

    if let Some(student) = student {
        Ok(HttpResponse::Ok().json(student))
    } else {
        Ok(HttpResponse::NotFound()
            .body(format!("No student found with student_id: {}", student_id)))
    }
}

#[post("/add_student")]
pub async fn add_student(
    pool: web::Data<DbPool>,
    form: web::Json<models::NewStudent>,
) -> ActionResult {
    let student: models::Student = db::blocked_access(&pool, move |conn| {
        use crate::schema::students::dsl::*;

        let new_student = models::NewStudent {
            first_name: form.first_name.clone(),
            last_name: form.last_name.clone(),
        };

        diesel::insert_into(students)
            .values(&new_student)
            .get_result(conn)
            .unwrap()
    })
    .await?;

    log::debug!("Database response: {:?}", student);

    Ok(HttpResponse::Ok().json(student))
}
