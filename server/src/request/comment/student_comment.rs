use actix_web::{get, web, HttpRequest, HttpResponse};
use diesel::prelude::*;

use crate::error::BabibappError;
use crate::request::{RequestContext, RequestResult};
use crate::{auth, db};

use babibapp_models as models;
use babibapp_schema::schema;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(view_all).service(view);
}

//
// GET
//

#[get("/view_all")]
async fn view_all(context: web::Data<RequestContext>, req: HttpRequest) -> RequestResult {
    let token_settings = &context.settings.token;

    let token = auth::TokenWrapper::from_request(req.clone())?;
    let _ = token.validate(token_settings.secret.clone())?;

    let comments = db::blocked_access(&context.pool, |conn| {
        use schema::student_comments::table;
        let list = table.load::<models::comment::StudentComment>(conn)?;
        Ok(list) as Result<Vec<models::comment::StudentComment>, BabibappError>
    })
    .await??;

    log::debug!("Database response: {:?}", comments);

    Ok(HttpResponse::Ok().json(comments))
}

#[get("/{comment_id}")]
async fn view(
    context: web::Data<RequestContext>,
    req: HttpRequest,
    comment_id: web::Path<i32>,
) -> RequestResult {
    let token_settings = &context.settings.token;

    let token = auth::TokenWrapper::from_request(req.clone())?;
    let _ = token.validate(token_settings.secret.clone())?;

    let comment_id = comment_id.into_inner();

    let comment = db::blocked_access(&context.pool, move |conn| {
        use schema::student_comments::dsl::*;

        student_comments
            .filter(id.eq(comment_id))
            .first::<models::comment::StudentComment>(conn)
            .optional()
    })
    .await??;

    log::debug!("Database response: {:?}", comment);

    if let Some(comment) = comment {
        Ok(HttpResponse::Ok().json(comment))
    } else {
        Ok(HttpResponse::NotFound()
            .body(format!("No comment found with comment_id: {}", comment_id)))
    }
}
