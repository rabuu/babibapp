use actix_web::{post, web, HttpResponse};
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};

use babibapp_models as models;
use babibapp_schema::schema;
use pwhash::bcrypt;

use crate::auth;
use crate::db;
use crate::settings::Settings;
use crate::DbPool;

use super::ActionResult;

#[post("/generate")]
async fn generate(
    pool: web::Data<DbPool>,
    settings: web::Data<Settings>,
    form: web::Json<models::student::LoginStudent>,
) -> ActionResult {
    let token_settings = &settings.token;
    let root_settings = &settings.root;

    let login_email = form.email.clone();
    let login_email_move = form.email.clone();
    let login_password = form.password.clone();

    if login_email == root_settings.email && login_password == root_settings.password {
        return Ok(HttpResponse::Ok().json(auth::TokenWrapper::from_claims(
            auth::Claims::root(root_settings.expiration_minutes),
            token_settings.secret.clone(),
        )?));
    }

    let student = db::blocked_access(&pool, move |conn| {
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
    settings: web::Data<Settings>,
    token: web::Json<auth::TokenWrapper>,
) -> ActionResult {
    let token_settings = &settings.token;

    match token.validate(token_settings.secret.clone()) {
        Ok(_) => Ok(HttpResponse::Ok().body("Valid token")),
        Err(e) => Ok(HttpResponse::Unauthorized().body(format!("{}", e))),
    }
}
