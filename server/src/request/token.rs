use actix_web::HttpResponse;
use actix_web::{post, web};
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};

use babibapp_models as models;
use babibapp_schema::schema;

use crate::auth;
use crate::db;
use crate::DbPool;

use super::ActionResult;

#[post("/generate")]
async fn generate(
    pool: web::Data<DbPool>,
    form: web::Json<models::student::LoginStudent>,
) -> ActionResult {
    let login_email = form.email.clone();
    let login_email_move = form.email.clone();
    let login_password = form.password.clone();

    let student = db::blocked_access(&pool, move |conn| {
        use schema::students::dsl::*;

        students
            .filter(email.eq(login_email_move))
            .first::<models::student::Student>(conn)
            .optional()
    })
    .await??;

    if let Some(student) = student {
        if student.password_hash == login_password {
            let claims = auth::Claims::new(student);
            Ok(HttpResponse::Ok().json(auth::TokenWrapper::from_claims(claims)?))
        } else {
            Ok(HttpResponse::Unauthorized().body(format!("Wrong password: {}", login_password)))
        }
    } else {
        Ok(HttpResponse::NotFound().body(format!("No student found with email: {}", login_email)))
    }
}

#[post("/validate")]
async fn validate(token: web::Json<auth::TokenWrapper>) -> ActionResult {
    match token.validate() {
        Ok(_) => Ok(HttpResponse::Ok().body("Valid token")),
        Err(e) => Ok(HttpResponse::Unauthorized().body(format!("{}", e))),
    }
}
