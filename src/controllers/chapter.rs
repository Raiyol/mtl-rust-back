use actix_web::{error, HttpResponse, Responder, web};
use crate::controllers::comment::init_chapters_comment_routes;

use crate::db::DbPool;
use crate::services::chapter_service;

pub fn init_novel_chapters_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("/{number}")
                .route(web::get().to(get_chapters_short))
        );
}

pub fn init_chapters_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("/recent")
                .route(web::get().to(get_recent_chapters))
        )
        .service(web::scope("/{id_chapter}/comments").configure(init_chapters_comment_routes));
}

async fn get_chapters_short(pool: web::Data<DbPool>, path: web::Path<(String, u32)>) -> actix_web::Result<impl Responder> {
    let (novel_url, chapter_number) = path.into_inner();
    let novels = web::block(move || {
        let mut conn = pool.get().expect("Error getting connection to DB");
        chapter_service::get_chapter_by_id(&mut *conn, novel_url, chapter_number)
    }).await?
        // map diesel query errors to a 500 error response
        .map_err(error::ErrorInternalServerError);

    Ok(HttpResponse::Ok().json(novels.expect("There should be novels in db")))
}

async fn get_recent_chapters(pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    let chap = web::block(move || {
        let mut conn = pool.get().expect("Error getting connection to DB");
        chapter_service::get_recent_chapters(&mut *conn)
    }).await?
        // map diesel query errors to a 500 error response
        .map_err(error::ErrorInternalServerError);

    Ok(HttpResponse::Ok().json(chap.expect("Chapters not found")))
}
