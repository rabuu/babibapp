use actix_web::{delete, get, post, web, HttpRequest, HttpResponse};
use diesel::prelude::*;

use babibapp_models as models;
use babibapp_schema::schema;
use pwhash::bcrypt;

use crate::auth;
use crate::db;
use crate::error::BabibappError;
use crate::request::{RequestContext, RequestResult};

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
async fn list_all(context: web::Data<RequestContext>, req: HttpRequest) -> RequestResult {
    let token_settings = &context.settings.token;

    let token = auth::TokenWrapper::from_request(req.clone())?;
    let claims = token.validate(token_settings.secret.clone())?;

    if !claims.student.admin {
        return Ok(HttpResponse::Unauthorized().body("Access only for admins"));
    }

    let students = db::blocked_access(&context.pool, |conn| {
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
    context: web::Data<RequestContext>,
    req: HttpRequest,
    student_id: web::Path<i32>,
) -> RequestResult {
    let token_settings = &context.settings.token;

    let token = auth::TokenWrapper::from_request(req.clone())?;
    let claims = token.validate(token_settings.secret.clone())?;

    if !claims.student.admin {
        return Ok(HttpResponse::Unauthorized().body("Access only for admins"));
    }

    let student_id = student_id.into_inner();

    let student = db::blocked_access(&context.pool, move |conn| {
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
    context: web::Data<RequestContext>,
    req: HttpRequest,
    form: web::Json<models::student::RegisterStudent>,
) -> RequestResult {
    let token_settings = &context.settings.token;

    let token = auth::TokenWrapper::from_request(req.clone())?;
    let claims = token.validate(token_settings.secret.clone())?;

    if !claims.student.admin {
        return Ok(HttpResponse::Unauthorized().body("Access only for admins"));
    }

    let hashed_password = bcrypt::hash(form.password.clone())?;

    let student = db::blocked_access(&context.pool, move |conn| {
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
    context: web::Data<RequestContext>,
    req: HttpRequest,
    student_id: web::Path<i32>,
) -> RequestResult {
    let token_settings = &context.settings.token;

    let token = auth::TokenWrapper::from_request(req.clone())?;
    let claims = token.validate(token_settings.secret.clone())?;

    if !claims.student.admin {
        return Ok(HttpResponse::Unauthorized().body("Access only for admins"));
    }

    let student_id = student_id.into_inner();

    let student = db::blocked_access(&context.pool, move |conn| {
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
