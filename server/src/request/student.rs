use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
use diesel::prelude::*;
use pwhash::bcrypt;

use babibapp_models as models;
use babibapp_schema::schema;
use models::wrappers::*;

use crate::auth;
use crate::db;
use crate::error::BabibappError;
use crate::request::{RequestContext, RequestResult};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get)
        .service(get_self)
        .service(get_all)
        .service(register)
        .service(reset_email)
        .service(reset_password)
        .service(reset_name)
        .service(make_admin)
        .service(reset_full)
        .service(delete);
}

#[get("/get/{student_id}")]
async fn get(
    context: web::Data<RequestContext>,
    req: HttpRequest,
    student_id: web::Path<i32>,
) -> RequestResult {
    let token_settings = &context.settings.token;

    let token = auth::token_from_request(req.clone())?;
    let claims = auth::validate_token(&token.token, token_settings.secret.clone())?;

    let student_id = student_id.into_inner();

    let student = db::blocked_access(&context.pool, move |conn| {
        use schema::students::dsl::*;

        students
            .find(student_id)
            .first::<models::student::Student>(conn)
            .optional()
    })
    .await??;

    log::debug!("Database response: {:?}", student);

    if let Some(student) = student {
        let student_view = if claims.id == student.id || claims.admin {
            models::student::StudentView::Full(student)
        } else {
            let limited = models::student::LimitedViewStudent {
                id: student.id,
                first_name: student.first_name,
                last_name: student.last_name,
            };
            models::student::StudentView::Limited(limited)
        };

        return Ok(HttpResponse::Ok().json(student_view));
    }

    Ok(HttpResponse::NotFound().body(format!("No student found with student_id: {}", student_id)))
}

#[get("/get_self")]
async fn get_self(context: web::Data<RequestContext>, req: HttpRequest) -> RequestResult {
    let token_settings = &context.settings.token;

    let token = auth::token_from_request(req.clone())?;
    let claims = auth::validate_token(&token.token, token_settings.secret.clone())?;

    let student_id = claims.id;

    let student = db::blocked_access(&context.pool, move |conn| {
        use schema::students::dsl::*;

        students
            .find(student_id)
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

#[get("/get_all")]
async fn get_all(context: web::Data<RequestContext>, req: HttpRequest) -> RequestResult {
    let token_settings = &context.settings.token;

    let token = auth::token_from_request(req.clone())?;
    let claims = auth::validate_token(&token.token, token_settings.secret.clone())?;

    let students = db::blocked_access(&context.pool, |conn| {
        use schema::students::table;
        let list = table.load::<models::student::Student>(conn)?;
        Ok(list) as Result<Vec<models::student::Student>, BabibappError>
    })
    .await??;

    log::debug!("Database response: {:?}", students);

    let student_views: Vec<models::student::StudentView> = students
        .into_iter()
        .map(|s| {
            if claims.id == s.id || claims.admin {
                models::student::StudentView::Full(s)
            } else {
                let limited = models::student::LimitedViewStudent {
                    id: s.id,
                    first_name: s.first_name,
                    last_name: s.last_name,
                };
                models::student::StudentView::Limited(limited)
            }
        })
        .collect();

    Ok(HttpResponse::Ok().json(student_views))
}

#[post("/register")]
async fn register(
    context: web::Data<RequestContext>,
    req: HttpRequest,
    form: web::Json<models::student::RegisterStudent>,
) -> RequestResult {
    let token_settings = &context.settings.token;

    let token = auth::token_from_request(req.clone())?;
    let claims = auth::validate_token(&token.token, token_settings.secret.clone())?;

    if !claims.admin {
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

#[put("/reset_email/{student_id}")]
async fn reset_email(
    context: web::Data<RequestContext>,
    req: HttpRequest,
    student_id: web::Path<i32>,
    form: web::Json<EmailWrapper>,
) -> RequestResult {
    let token_settings = &context.settings.token;

    let token = auth::token_from_request(req.clone())?;
    let claims = auth::validate_token(&token.token, token_settings.secret.clone())?;

    let student_id = student_id.into_inner();

    if !claims.admin && student_id != claims.id {
        return Ok(HttpResponse::Unauthorized().body("Access only for admins"));
    }

    let new_email = form.email.clone();

    let student = db::blocked_access(&context.pool, move |conn| {
        use schema::students::dsl::*;

        diesel::update(students.find(student_id))
            .set(email.eq(new_email))
            .get_result::<models::student::Student>(conn)
            .optional()
    })
    .await??;

    if let Some(student) = student {
        Ok(HttpResponse::Ok().json(student))
    } else {
        Ok(HttpResponse::NotFound()
            .body(format!("No student found with student_id: {}", student_id)))
    }
}

#[put("/reset_password/{student_id}")]
async fn reset_password(
    context: web::Data<RequestContext>,
    req: HttpRequest,
    student_id: web::Path<i32>,
    form: web::Json<PasswordWrapper>,
) -> RequestResult {
    let token_settings = &context.settings.token;

    let token = auth::token_from_request(req.clone())?;
    let claims = auth::validate_token(&token.token, token_settings.secret.clone())?;

    let student_id = student_id.into_inner();

    if !claims.admin && student_id != claims.id {
        return Ok(HttpResponse::Unauthorized().body("Access only for admins"));
    }

    let hashed_password = bcrypt::hash(form.password.clone())?;

    let student = db::blocked_access(&context.pool, move |conn| {
        use schema::students::dsl::*;

        diesel::update(students.find(student_id))
            .set(password_hash.eq(hashed_password))
            .get_result::<models::student::Student>(conn)
            .optional()
    })
    .await??;

    if let Some(student) = student {
        Ok(HttpResponse::Ok().json(student))
    } else {
        Ok(HttpResponse::NotFound()
            .body(format!("No student found with student_id: {}", student_id)))
    }
}

#[put("/reset_name/{student_id}")]
async fn reset_name(
    context: web::Data<RequestContext>,
    req: HttpRequest,
    student_id: web::Path<i32>,
    form: web::Json<NameWrapper>,
) -> RequestResult {
    let token_settings = &context.settings.token;

    let token = auth::token_from_request(req.clone())?;
    let claims = auth::validate_token(&token.token, token_settings.secret.clone())?;

    let student_id = student_id.into_inner();

    if !claims.admin {
        return Ok(HttpResponse::Unauthorized().body("Access only for admins"));
    }

    let student = db::blocked_access(&context.pool, move |conn| {
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

    if let Some(student) = student {
        Ok(HttpResponse::Ok().json(student))
    } else {
        Ok(HttpResponse::NotFound()
            .body(format!("No student found with student_id: {}", student_id)))
    }
}

#[put("/make_admin/{student_id}")]
async fn make_admin(
    context: web::Data<RequestContext>,
    req: HttpRequest,
    student_id: web::Path<i32>,
) -> RequestResult {
    let token_settings = &context.settings.token;

    let token = auth::token_from_request(req.clone())?;
    let claims = auth::validate_token(&token.token, token_settings.secret.clone())?;

    let student_id = student_id.into_inner();

    if !claims.admin {
        return Ok(HttpResponse::Unauthorized().body("Access only for admins"));
    }

    let student = db::blocked_access(&context.pool, move |conn| {
        use schema::students::dsl::*;

        diesel::update(students.find(student_id))
            .set(admin.eq(true))
            .get_result::<models::student::Student>(conn)
            .optional()
    })
    .await??;

    if let Some(student) = student {
        Ok(HttpResponse::Ok().json(student))
    } else {
        Ok(HttpResponse::NotFound()
            .body(format!("No student found with student_id: {}", student_id)))
    }
}

#[put("/reset_full/{student_id}")]
async fn reset_full(
    context: web::Data<RequestContext>,
    req: HttpRequest,
    student_id: web::Path<i32>,
    form: web::Json<models::student::RegisterStudent>,
) -> RequestResult {
    let token_settings = &context.settings.token;

    let token = auth::token_from_request(req.clone())?;
    let claims = auth::validate_token(&token.token, token_settings.secret.clone())?;

    let student_id = student_id.into_inner();

    if !claims.admin {
        return Ok(HttpResponse::Unauthorized().body("Access only for admins"));
    }

    let new_admin_status = form.admin.unwrap_or(false);
    let hashed_password = bcrypt::hash(form.password.clone())?;

    let student = db::blocked_access(&context.pool, move |conn| {
        use schema::students::dsl::*;

        diesel::update(students.find(student_id))
            .set((
                email.eq(form.email.clone()),
                first_name.eq(form.first_name.clone()),
                last_name.eq(form.last_name.clone()),
                password_hash.eq(hashed_password),
                admin.eq(new_admin_status),
            ))
            .get_result::<models::student::Student>(conn)
            .optional()
    })
    .await??;

    if let Some(student) = student {
        Ok(HttpResponse::Ok().json(student))
    } else {
        Ok(HttpResponse::NotFound()
            .body(format!("No student found with student_id: {}", student_id)))
    }
}

#[delete("/delete/{student_id}")]
async fn delete(
    context: web::Data<RequestContext>,
    req: HttpRequest,
    student_id: web::Path<i32>,
) -> RequestResult {
    let token_settings = &context.settings.token;

    let token = auth::token_from_request(req.clone())?;
    let claims = auth::validate_token(&token.token, token_settings.secret.clone())?;

    let student_id = student_id.into_inner();

    if !claims.admin && student_id != claims.id {
        return Ok(HttpResponse::Unauthorized().body("Access only for admins"));
    }

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
