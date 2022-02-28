use actix_web::HttpResponse;
use actix_web::{post, web};
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl};

use babibapp_models as models;
use babibapp_schema::schema;

use crate::auth::{create_jwt, Claims, TokenWrapper};
use crate::db;
use crate::DbPool;

use super::ActionResult;

#[post("/token")]
async fn generate_token(
    pool: web::Data<DbPool>,
    form: web::Json<models::student::LoginStudent>,
) -> ActionResult {
    log::debug!("Called `generate_token`");

    let login_email = form.email.clone();
    let login_email_move = form.email.clone();
    let login_password = form.password.clone();

    let valid_password = db::blocked_access(&pool, move |conn| {
        use schema::students::dsl::*;

        students
            .select(password_hash)
            .filter(email.eq(login_email_move))
            .first::<String>(conn)
            .optional()
    })
    .await??;

    if let Some(valid_password) = valid_password {
        if valid_password == login_password {
            let claims = Claims::new(login_email);
            Ok(HttpResponse::Ok().json(TokenWrapper {
                token: create_jwt(claims)?,
            }))
        } else {
            Ok(HttpResponse::Unauthorized().body(format!("Wrong password: {}", login_password)))
        }
    } else {
        Ok(HttpResponse::NotFound().body(format!("No student found with email: {}", login_email)))
    }
}
