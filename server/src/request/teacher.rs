use actix_web::{delete, get, post, web, HttpRequest, HttpResponse};
use diesel::prelude::*;

use babibapp_models as models;
use babibapp_schema::schema;

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

    let teachers = db::blocked_access(&context.pool, |conn| {
        use schema::teachers::table;
        let list = table.load::<models::teacher::Teacher>(conn)?;
        Ok(list) as Result<Vec<models::teacher::Teacher>, BabibappError>
    })
    .await??;

    log::debug!("Database response: {:?}", teachers);

    Ok(HttpResponse::Ok().json(teachers))
}

#[get("/{teacher_id}")]
async fn get(
    context: web::Data<RequestContext>,
    req: HttpRequest,
    teacher_id: web::Path<i32>,
) -> RequestResult {
    let token_settings = &context.settings.token;

    let token = auth::TokenWrapper::from_request(req.clone())?;
    let claims = token.validate(token_settings.secret.clone())?;

    if !claims.student.admin {
        return Ok(HttpResponse::Unauthorized().body("Access only for admins"));
    }

    let teacher_id = teacher_id.into_inner();

    let teacher = db::blocked_access(&context.pool, move |conn| {
        use schema::teachers::dsl::*;

        teachers
            .filter(id.eq(teacher_id))
            .first::<models::teacher::Teacher>(conn)
            .optional()
    })
    .await??;

    log::debug!("Database response: {:?}", teacher);

    if let Some(teacher) = teacher {
        Ok(HttpResponse::Ok().json(teacher))
    } else {
        Ok(HttpResponse::NotFound()
            .body(format!("No teacher found with teacher_id: {}", teacher_id)))
    }
}

//
// POST
//

#[post("/add")]
async fn add(
    context: web::Data<RequestContext>,
    req: HttpRequest,
    form: web::Json<models::teacher::NewTeacher>,
) -> RequestResult {
    let token_settings = &context.settings.token;

    let token = auth::TokenWrapper::from_request(req.clone())?;
    let claims = token.validate(token_settings.secret.clone())?;

    if !claims.student.admin {
        return Ok(HttpResponse::Unauthorized().body("Access only for admins"));
    }

    let teacher = db::blocked_access(&context.pool, move |conn| {
        use schema::teachers::dsl::*;

        let new_teacher = models::teacher::NewTeacher {
            name: form.name.clone(),
            prefix: form.prefix.clone(),
        };

        diesel::insert_into(teachers)
            .values(&new_teacher)
            .get_result::<models::teacher::Teacher>(conn)
    })
    .await??;

    log::debug!("Database response: {:?}", teacher);

    Ok(HttpResponse::Ok().json(teacher))
}

//
// DELETE
//

#[delete("/{teacher_id}")]
async fn delete(
    context: web::Data<RequestContext>,
    req: HttpRequest,
    teacher_id: web::Path<i32>,
) -> RequestResult {
    let token_settings = &context.settings.token;

    let token = auth::TokenWrapper::from_request(req.clone())?;
    let claims = token.validate(token_settings.secret.clone())?;

    if !claims.student.admin {
        return Ok(HttpResponse::Unauthorized().body("Access only for admins"));
    }

    let teacher_id = teacher_id.into_inner();

    let teacher = db::blocked_access(&context.pool, move |conn| {
        use schema::teachers::dsl::*;

        diesel::delete(teachers.filter(id.eq(teacher_id)))
            .get_result::<models::teacher::Teacher>(conn)
            .optional()
    })
    .await??;

    log::debug!("Database response: {:?}", teacher);

    if let Some(teacher) = teacher {
        Ok(HttpResponse::Ok().json(teacher))
    } else {
        Ok(HttpResponse::NotFound()
            .body(format!("No teacher found with teacher_id: {}", teacher_id)))
    }
}
