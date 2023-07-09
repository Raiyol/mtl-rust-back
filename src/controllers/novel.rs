use actix_web::{error, HttpResponse, Responder, web};
use actix_web_validator::Query;

use crate::beans::pageable::Pageable;
use crate::db::DbPool;
use crate::services::novel_service;
use crate::controllers::chapter::init_chapters_routes;

pub fn init_novels_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::resource("")
                .route(web::get().to(get_novels))
        )
        .service(
            web::scope("/{novel_url}/chapters")
                .configure(init_chapters_routes)
        )
        .service(
            web::resource("/{novel_url}")
                .route(web::get().to(get_novel_by_url))
        );
}

async fn get_novels(pool: web::Data<DbPool>, pageable: Query<Pageable>) -> actix_web::Result<impl Responder> {
    let novels = web::block(move || {
        let mut conn = pool.get().expect("Error getting connection to DB");
        novel_service::find_all_novel(&mut *conn, pageable.into_inner())
    }).await?
        // map diesel query errors to a 500 error response
        .map_err(error::ErrorInternalServerError);

    Ok(HttpResponse::Ok().json(novels.expect("There should be novels in db")))
}

async fn get_novel_by_url(pool: web::Data<DbPool>, novel_url: web::Path<String>) -> actix_web::Result<impl Responder> {
    let nov = web::block(move || {
        let mut conn = pool.get().expect("Error getting connection to DB");
        novel_service::find_novel_by_url_with_chapters_info(&mut *conn, novel_url.into_inner())
    }).await?
        // map diesel query errors to a 500 error response
        .map_err(error::ErrorInternalServerError);

    Ok(HttpResponse::Ok().json(nov.expect("Novel not found")))
}
