use actix_web::{error, web, HttpResponse, Responder};
use actix_web_validator::Query;

use crate::beans::pageable::Pageable;
use crate::db::DbPool;
use crate::services::comment_service;

pub fn init_chapters_comment_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("").route(web::get().to(get_comments)));
}

async fn get_comments(
    pool: web::Data<DbPool>,
    path: web::Path<u32>,
    pageable: Query<Pageable>,
) -> actix_web::Result<impl Responder> {
    let comments = web::block(move || {
        let mut conn = pool.get().expect("Error getting connection to DB");
        comment_service::get_comments_by_chapter_id(
            &mut *conn,
            path.into_inner(),
            pageable.into_inner(),
        )
    })
    .await?
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError);

    Ok(HttpResponse::Ok().json(comments.expect("Comments not found")))
}
