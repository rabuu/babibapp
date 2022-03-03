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
        .service(get_vote)
        .service(create)
        .service(do_upvote)
        .service(do_downvote)
        .service(do_unvote)
        .service(delete);
}

#[get("/get/{comment_id}")]
async fn get(
    context: web::Data<RequestContext>,
    req: HttpRequest,
    comment_id: web::Path<i32>,
) -> RequestResult {
    let token_settings = &context.settings.token;

    let token = auth::token_from_request(req.clone())?;
    let claims = auth::validate_token(&token.token, token_settings.secret.clone())?;

    let comment_id = comment_id.into_inner();

    let comment = db::blocked_access(&context.pool, move |conn| {
        use schema::teacher_comments::dsl::*;

        teacher_comments
            .find(comment_id)
            .first::<models::comment::TeacherComment>(conn)
            .optional()
    })
    .await??;

    log::debug!("Database response: {:?}", comment);

    if let Some(comment) = comment {
        let comment_view = if claims.id == comment.author_id || claims.admin {
            models::comment::TeacherCommentView::Full(comment)
        } else {
            let limited = models::comment::LimitedViewTeacherComment {
                id: comment.id,
                receiver_id: comment.receiver_id,
                body: comment.body,
                published: comment.published,
            };
            models::comment::TeacherCommentView::Limited(limited)
        };

        return Ok(HttpResponse::Ok().json(comment_view));
    }

    Ok(HttpResponse::NotFound().body(format!("No comment found with comment_id: {}", comment_id)))
}

#[get("/get_all")]
async fn get_all(context: web::Data<RequestContext>, req: HttpRequest) -> RequestResult {
    let token_settings = &context.settings.token;

    let token = auth::token_from_request(req.clone())?;
    let claims = auth::validate_token(&token.token, token_settings.secret.clone())?;

    let comments = db::blocked_access(&context.pool, |conn| {
        use schema::teacher_comments::table;
        let list = table.load::<models::comment::TeacherComment>(conn)?;
        Ok(list) as Result<Vec<models::comment::TeacherComment>, BabibappError>
    })
    .await??;

    log::debug!("Database response: {:?}", comments);

    let comment_views: Vec<models::comment::TeacherCommentView> = comments
        .into_iter()
        .map(|c| {
            if claims.id == c.author_id || claims.admin {
                models::comment::TeacherCommentView::Full(c)
            } else {
                let limited = models::comment::LimitedViewTeacherComment {
                    id: c.id,
                    receiver_id: c.receiver_id,
                    body: c.body,
                    published: c.published,
                };
                models::comment::TeacherCommentView::Limited(limited)
            }
        })
        .collect();

    Ok(HttpResponse::Ok().json(comment_views))
}

#[get("/get_vote/{comment_id}")]
async fn get_vote(
    context: web::Data<RequestContext>,
    req: HttpRequest,
    comment_id: web::Path<i32>,
) -> RequestResult {
    let token_settings = &context.settings.token;

    let token = auth::token_from_request(req.clone())?;
    let _ = auth::validate_token(&token.token, token_settings.secret.clone())?;

    let query_comment_id = comment_id.into_inner();

    let vote = db::blocked_access(&context.pool, move |conn| {
        use diesel::dsl::count_star;
        use schema::teacher_comment_votes::dsl::*;

        let upvotes = teacher_comment_votes
            .filter(comment_id.eq(query_comment_id))
            .filter(upvote.eq(true))
            .select(count_star())
            .get_result::<i64>(conn)?;

        let downvotes = teacher_comment_votes
            .filter(comment_id.eq(query_comment_id))
            .filter(upvote.eq(false))
            .select(count_star())
            .get_result::<i64>(conn)?;

        Ok(upvotes - downvotes) as Result<i64, BabibappError>
    })
    .await??;

    Ok(HttpResponse::Ok().json(vote))
}

#[post("/create")]
async fn create(
    context: web::Data<RequestContext>,
    req: HttpRequest,
    form: web::Json<models::comment::CreateTeacherComment>,
) -> RequestResult {
    let token_settings = &context.settings.token;

    let token = auth::token_from_request(req.clone())?;
    let claims = auth::validate_token(&token.token, token_settings.secret.clone())?;

    let comment = db::blocked_access(&context.pool, move |conn| {
        use schema::teacher_comments::dsl::*;

        let new_comment = models::comment::NewTeacherComment {
            author_id: claims.id,
            receiver_id: form.receiver_id,
            body: form.body.clone(),
            published: Some(SystemTime::now()),
        };

        diesel::insert_into(teacher_comments)
            .values(&new_comment)
            .get_result::<models::comment::TeacherComment>(conn)
    })
    .await??;

    log::debug!("Database response: {:?}", comment);

    Ok(HttpResponse::Ok().json(comment))
}

#[post("/upvote/{comment_id}")]
async fn do_upvote(
    context: web::Data<RequestContext>,
    req: HttpRequest,
    comment_id: web::Path<i32>,
) -> RequestResult {
    let token_settings = &context.settings.token;

    let token = auth::token_from_request(req.clone())?;
    let claims = auth::validate_token(&token.token, token_settings.secret.clone())?;

    let query_comment_id = comment_id.into_inner();

    let comment_vote = db::blocked_access(&context.pool, move |conn| {
        use schema::teacher_comment_votes::dsl::*;

        diesel::update(teacher_comment_votes)
            .filter(comment_id.eq(query_comment_id))
            .filter(student_id.eq(claims.id))
            .set(upvote.eq(true))
            .get_result::<models::comment::TeacherCommentVote>(conn)
            .optional()
    })
    .await??;

    let comment_vote = if let Some(comment_vote) = comment_vote {
        comment_vote
    } else {
        db::blocked_access(&context.pool, move |conn| {
            use schema::teacher_comment_votes::dsl::*;

            let new_comment_vote = models::comment::NewTeacherCommentVote {
                comment_id: query_comment_id,
                student_id: claims.id,
                upvote: true,
            };

            diesel::insert_into(teacher_comment_votes)
                .values(&new_comment_vote)
                .get_result::<models::comment::TeacherCommentVote>(conn)
        })
        .await??
    };

    Ok(HttpResponse::Ok().json(comment_vote))
}

#[post("/downvote/{comment_id}")]
async fn do_downvote(
    context: web::Data<RequestContext>,
    req: HttpRequest,
    comment_id: web::Path<i32>,
) -> RequestResult {
    let token_settings = &context.settings.token;

    let token = auth::token_from_request(req.clone())?;
    let claims = auth::validate_token(&token.token, token_settings.secret.clone())?;

    let query_comment_id = comment_id.into_inner();

    let comment_vote = db::blocked_access(&context.pool, move |conn| {
        use schema::teacher_comment_votes::dsl::*;

        diesel::update(teacher_comment_votes)
            .filter(comment_id.eq(query_comment_id))
            .filter(student_id.eq(claims.id))
            .set(upvote.eq(false))
            .get_result::<models::comment::TeacherCommentVote>(conn)
            .optional()
    })
    .await??;

    let comment_vote = if let Some(comment_vote) = comment_vote {
        comment_vote
    } else {
        db::blocked_access(&context.pool, move |conn| {
            use schema::teacher_comment_votes::dsl::*;

            let new_comment_vote = models::comment::NewTeacherCommentVote {
                comment_id: query_comment_id,
                student_id: claims.id,
                upvote: false,
            };

            diesel::insert_into(teacher_comment_votes)
                .values(&new_comment_vote)
                .get_result::<models::comment::TeacherCommentVote>(conn)
        })
        .await??
    };

    Ok(HttpResponse::Ok().json(comment_vote))
}

#[delete("/unvote/{comment_id}")]
async fn do_unvote(
    context: web::Data<RequestContext>,
    req: HttpRequest,
    comment_id: web::Path<i32>,
) -> RequestResult {
    let token_settings = &context.settings.token;

    let token = auth::token_from_request(req.clone())?;
    let claims = auth::validate_token(&token.token, token_settings.secret.clone())?;

    let query_comment_id = comment_id.into_inner();

    let comment_vote = db::blocked_access(&context.pool, move |conn| {
        use schema::teacher_comment_votes::dsl::*;
        diesel::delete(
            teacher_comment_votes
                .filter(student_id.eq(claims.id))
                .filter(comment_id.eq(query_comment_id)),
        )
        .get_result::<models::comment::TeacherCommentVote>(conn)
        .optional()
    })
    .await??;

    if let Some(comment_vote) = comment_vote {
        Ok(HttpResponse::Ok().json(comment_vote))
    } else {
        Ok(HttpResponse::Ok().body(format!("No vote on comment: {}", query_comment_id)))
    }
}

#[delete("/delete/{comment_id}")]
async fn delete(
    context: web::Data<RequestContext>,
    req: HttpRequest,
    comment_id: web::Path<i32>,
) -> RequestResult {
    let token_settings = &context.settings.token;

    let token = auth::token_from_request(req.clone())?;
    let claims = auth::validate_token(&token.token, token_settings.secret.clone())?;

    let comment_id = comment_id.into_inner();

    let comment = db::blocked_access(&context.pool, move |conn| {
        use schema::teacher_comments::dsl::*;

        let author = teacher_comments
            .find(comment_id)
            .select(author_id)
            .get_result::<i32>(conn)?;

        if claims.admin || author == claims.id {
            return diesel::delete(teacher_comments.find(comment_id))
                .get_result::<models::comment::TeacherComment>(conn);
        }

        Err(diesel::result::Error::NotFound)
    })
    .await??;

    log::debug!("Database response: {:?}", comment);

    Ok(HttpResponse::Ok().json(comment))
}
