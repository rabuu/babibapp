use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};

use babibapp_models as models;
use babibapp_schema::schema;

use super::ActionResult;
use crate::auth;
use crate::db;
use crate::error::BabibappError;
use crate::DbPool;

//
// GET
//

#[get("/list_all")]
async fn list_all(pool: web::Data<DbPool>, req: HttpRequest) -> ActionResult {
    let token = auth::TokenWrapper::from_request(req)?;
    let claims = token.validate()?;

    if !claims.student.is_admin {
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
    student_id: web::Path<i32>,
    req: HttpRequest,
) -> ActionResult {
    let token = auth::TokenWrapper::from_request(req)?;
    let claims = token.validate()?;

    if !claims.student.is_admin {
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

#[post("/register")]
async fn add(
    pool: web::Data<DbPool>,
    form: web::Json<models::student::RegisterStudent>,
    req: HttpRequest,
) -> ActionResult {
    let token = auth::TokenWrapper::from_request(req)?;
    let claims = token.validate()?;

    if !claims.student.is_admin {
        return Ok(HttpResponse::Unauthorized().body("Access only for admins"));
    }

    let student = db::blocked_access(&pool, move |conn| {
        use schema::students::dsl::*;

        let new_student = models::student::NewStudent {
            email: form.email.clone(),
            // TODO: Implement hashing
            first_name: form.first_name.clone(),
            last_name: form.last_name.clone(),
            password_hash: form.password.clone(),
            is_admin: form.is_admin,
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
// PUT
//

#[put("/reset/{student_id}")]
async fn reset(
    pool: web::Data<DbPool>,
    student_id: web::Path<i32>,
    form: web::Json<models::student::NewStudent>,
    req: HttpRequest,
) -> ActionResult {
    let token = auth::TokenWrapper::from_request(req)?;
    let claims = token.validate()?;

    if !claims.student.is_admin {
        return Ok(HttpResponse::Unauthorized().body("Access only for admins"));
    }

    let student_id = student_id.into_inner();

    let student = db::blocked_access(&pool, move |conn| {
        use schema::students::dsl::*;

        diesel::update(students.find(student_id))
            .set((
                first_name.eq(form.first_name.clone()),
                last_name.eq(form.last_name.clone()),
            ))
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

//
// DELETE
//

#[delete("/{student_id}")]
async fn delete(
    pool: web::Data<DbPool>,
    student_id: web::Path<i32>,
    req: HttpRequest,
) -> ActionResult {
    let token = auth::TokenWrapper::from_request(req)?;
    let claims = token.validate()?;

    if !claims.student.is_admin {
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
