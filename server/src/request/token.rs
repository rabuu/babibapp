use actix_web::{post, web, HttpResponse};
use diesel::prelude::*;

use babibapp_models as models;
use babibapp_schema::schema;
use pwhash::bcrypt;

use crate::auth;
use crate::db;
use crate::request::{RequestContext, RequestResult};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(generate).service(validate);
}

#[post("/generate")]
async fn generate(
    context: web::Data<RequestContext>,
    form: web::Json<models::student::LoginStudent>,
) -> RequestResult {
    let token_settings = &context.settings.token;
    let root_settings = &context.settings.root;

    let login_email = form.email.clone();
    let login_email_move = form.email.clone();
    let login_password = form.password.clone();

    if login_email == root_settings.email && login_password == root_settings.password {
        return Ok(HttpResponse::Ok().json(auth::TokenWrapper::from_claims(
            auth::Claims::root(root_settings.expiration_minutes),
            token_settings.secret.clone(),
        )?));
    }

    let student = db::blocked_access(&context.pool, move |conn| {
        use schema::students::dsl::*;

        students
            .filter(email.eq(login_email_move))
            .first::<models::student::Student>(conn)
            .optional()
    })
    .await??;

    if let Some(student) = student {
        if bcrypt::verify(&login_password, &student.password_hash) {
            let claims = auth::Claims::new(student, token_settings.expiration_hours);
            Ok(HttpResponse::Ok().json(auth::TokenWrapper::from_claims(
                claims,
                token_settings.secret.clone(),
            )?))
        } else {
            Ok(HttpResponse::Unauthorized().body(format!("Wrong password: {}", login_password)))
        }
    } else {
        Ok(HttpResponse::NotFound().body(format!("No student found with email: {}", login_email)))
    }
}

#[post("/validate")]
async fn validate(
    context: web::Data<RequestContext>,
    token: web::Json<auth::TokenWrapper>,
) -> RequestResult {
    let token_settings = &context.settings.token;

    match token.validate(token_settings.secret.clone()) {
        Ok(_) => Ok(HttpResponse::Ok().body("Valid token")),
        Err(e) => Ok(HttpResponse::Unauthorized().body(format!("{}", e))),
    }
}
