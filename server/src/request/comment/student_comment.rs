use std::time::SystemTime;

use actix_web::{delete, get, post, web, HttpRequest, HttpResponse};
use diesel::prelude::*;

use crate::error::BabibappError;
use crate::request::{RequestContext, RequestResult};
use crate::{auth, db};

use babibapp_models as models;
use babibapp_schema::schema;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get)
        .service(get_all)
        .service(create)
        .service(delete);
}

#[get("/get/{comment_id}")]
async fn get(
    context: web::Data<RequestContext>,
    req: HttpRequest,
    comment_id: web::Path<i32>,
) -> RequestResult {
    let token_settings = &context.settings.token;

    let token = auth::TokenWrapper::from_request(req.clone())?;
    let claims = token.validate(token_settings.secret.clone())?;

    let comment_id = comment_id.into_inner();

    let comment = db::blocked_access(&context.pool, move |conn| {
        use schema::student_comments::dsl::*;

        student_comments
            .find(comment_id)
            .first::<models::comment::StudentComment>(conn)
            .optional()
    })
    .await??;

    log::debug!("Database response: {:?}", comment);

    if let Some(comment) = comment {
        if claims.id == comment.author_id || claims.admin {
            return Ok(HttpResponse::Ok().json(comment));
        } else {
            let limited_view_student_comment = models::comment::LimitedViewStudentComment {
                id: comment.id,
                receiver_id: comment.receiver_id,
                body: comment.body,
                published: comment.published,
            };
            return Ok(HttpResponse::Ok().json(limited_view_student_comment));
        }
    }

    Ok(HttpResponse::NotFound().body(format!("No comment found with comment_id: {}", comment_id)))
}

#[get("/get_all")]
async fn get_all(context: web::Data<RequestContext>, req: HttpRequest) -> RequestResult {
    let token_settings = &context.settings.token;

    let token = auth::TokenWrapper::from_request(req.clone())?;
    let claims = token.validate(token_settings.secret.clone())?;

    let comments = db::blocked_access(&context.pool, |conn| {
        use schema::student_comments::table;
        let list = table.load::<models::comment::StudentComment>(conn)?;
        Ok(list) as Result<Vec<models::comment::StudentComment>, BabibappError>
    })
    .await??;

    log::debug!("Database response: {:?}", comments);

    if claims.admin {
        Ok(HttpResponse::Ok().json(comments))
    } else {
        let limited_view_student_comments: Vec<models::comment::LimitedViewStudentComment> =
            comments
                .into_iter()
                .map(|c| models::comment::LimitedViewStudentComment {
                    id: c.id,
                    receiver_id: c.receiver_id,
                    body: c.body,
                    published: c.published,
                })
                .collect();
        Ok(HttpResponse::Ok().json(limited_view_student_comments))
    }
}

#[post("/create")]
async fn create(
    context: web::Data<RequestContext>,
    req: HttpRequest,
    form: web::Json<models::comment::CreateStudentComment>,
) -> RequestResult {
    let token_settings = &context.settings.token;

    let token = auth::TokenWrapper::from_request(req.clone())?;
    let claims = token.validate(token_settings.secret.clone())?;

    let comment = db::blocked_access(&context.pool, move |conn| {
        use schema::student_comments::dsl::*;

        let new_comment = models::comment::NewStudentComment {
            author_id: claims.id,
            receiver_id: form.receiver_id,
            body: form.body.clone(),
            published: Some(SystemTime::now()),
        };

        diesel::insert_into(student_comments)
            .values(&new_comment)
            .get_result::<models::comment::StudentComment>(conn)
    })
    .await??;

    log::debug!("Database response: {:?}", comment);

    Ok(HttpResponse::Ok().json(comment))
}

#[delete("/delete/{comment_id}")]
async fn delete(
    context: web::Data<RequestContext>,
    req: HttpRequest,
    comment_id: web::Path<i32>,
) -> RequestResult {
    let token_settings = &context.settings.token;

    let token = auth::TokenWrapper::from_request(req.clone())?;
    let claims = token.validate(token_settings.secret.clone())?;

    let comment_id = comment_id.into_inner();

    let comment = db::blocked_access(&context.pool, move |conn| {
        use schema::student_comments::dsl::*;

        let author = student_comments
            .find(comment_id)
            .select(author_id)
            .get_result::<i32>(conn)?;

        if claims.admin || author == claims.id {
            return diesel::delete(student_comments.find(comment_id))
                .get_result::<models::comment::StudentComment>(conn);
        }

        Err(diesel::result::Error::NotFound)
    })
    .await??;

    log::debug!("Database response: {:?}", comment);

    Ok(HttpResponse::Ok().json(comment))
}
