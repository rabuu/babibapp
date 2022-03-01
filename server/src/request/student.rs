use actix_web::{delete, get, post, web, HttpRequest, HttpResponse};
use diesel::prelude::*;

use babibapp_models as models;
use babibapp_schema::schema;
use pwhash::bcrypt;

use super::ActionResult;
use crate::auth;
use crate::db;
use crate::error::BabibappError;
use crate::settings::Settings;
use crate::DbPool;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(list_all)
        .service(get)
        .service(add)
        .service(delete);
}

//
// GET
//

#[get("/list_all")]
async fn list_all(
    pool: web::Data<DbPool>,
    settings: web::Data<Settings>,
    req: HttpRequest,
) -> ActionResult {
    let token_settings = &settings.token;

    let token = auth::TokenWrapper::from_request(req.clone())?;
    let claims = token.validate(token_settings.secret.clone())?;

    if !claims.student.admin {
        return Ok(HttpResponse::Unauthorized().body("Access only for admins"));
    }

    let students = db::blocked_access(&pool, |conn| {
        use schema::students::table;
        let list = table.load::<models::student::Student>(conn)?;
        Ok(list) as Result<Vec<models::student::Student>, BabibappError>
    })
    .await??;

    log::debug!("Database response: {:?}", students);

    Ok(HttpResponse::Ok().json(students))
}

#[get("/{student_id}")]
async fn get(
    pool: web::Data<DbPool>,
    settings: web::Data<Settings>,
    student_id: web::Path<i32>,
    req: HttpRequest,
) -> ActionResult {
    let token_settings = &settings.token;

    let token = auth::TokenWrapper::from_request(req.clone())?;
    let claims = token.validate(token_settings.secret.clone())?;

    if !claims.student.admin {
        return Ok(HttpResponse::Unauthorized().body("Access only for admins"));
    }

    let student_id = student_id.into_inner();

    let student = db::blocked_access(&pool, move |conn| {
        use schema::students::dsl::*;

        students
            .filter(id.eq(student_id))
            .first::<models::student::Student>(conn)
            .optional()
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

//
// POST
//

#[post("/add")]
async fn add(
    pool: web::Data<DbPool>,
    settings: web::Data<Settings>,
    form: web::Json<models::student::RegisterStudent>,
    req: HttpRequest,
) -> ActionResult {
    let token_settings = &settings.token;

    let token = auth::TokenWrapper::from_request(req.clone())?;
    let claims = token.validate(token_settings.secret.clone())?;

    if !claims.student.admin {
        return Ok(HttpResponse::Unauthorized().body("Access only for admins"));
    }

    let hashed_password = bcrypt::hash(form.password.clone())?;

    let student = db::blocked_access(&pool, move |conn| {
        use schema::students::dsl::*;

        let new_student = models::student::NewStudent {
            email: form.email.clone(),
            first_name: form.first_name.clone(),
            last_name: form.last_name.clone(),
            password_hash: hashed_password,
            admin: form.admin,
        };

        diesel::insert_into(students)
            .values(&new_student)
            .get_result::<models::student::Student>(conn)
    })
    .await??;

    log::debug!("Database response: {:?}", student);

    Ok(HttpResponse::Ok().json(student))
}

//
// DELETE
//

#[delete("/{student_id}")]
async fn delete(
    pool: web::Data<DbPool>,
    settings: web::Data<Settings>,
    student_id: web::Path<i32>,
    req: HttpRequest,
) -> ActionResult {
    let token_settings = &settings.token;

    let token = auth::TokenWrapper::from_request(req.clone())?;
    let claims = token.validate(token_settings.secret.clone())?;

    if !claims.student.admin {
        return Ok(HttpResponse::Unauthorized().body("Access only for admins"));
    }

    let student_id = student_id.into_inner();

    let student = db::blocked_access(&pool, move |conn| {
        use schema::students::dsl::*;

        diesel::delete(students.filter(id.eq(student_id)))
            .get_result::<models::student::Student>(conn)
            .optional()
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
