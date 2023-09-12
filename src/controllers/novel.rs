use actix_web::{error, web, HttpResponse, Responder};
use actix_web_validator::Query;

use crate::beans::pageable::Pageable;
use crate::beans::search_query::SearchQuery;
use crate::controllers::chapter::init_novel_chapters_routes;
use crate::db::DbPool;
use crate::services::novel_service;

pub fn init_novels_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("").route(web::get().to(get_novels)))
        .service(web::resource("/random").route(web::get().to(get_random_novels)))
        .service(web::resource("/search").route(web::get().to(search_novel)))
        .service(web::resource("/{novel_url}").route(web::get().to(get_novel_by_url)))
        .service(web::scope("/{novel_url}/chapters").configure(init_novel_chapters_routes))
        .service(web::resource("/{novel_id}/reviews").route(web::get().to(get_novel_by_url)));
}

async fn get_novels(
    pool: web::Data<DbPool>,
    pageable: Query<Pageable>,
) -> actix_web::Result<impl Responder> {
    let novels = web::block(move || {
        let mut conn = pool.get().expect("Error getting connection to DB");
        novel_service::find_all_novel(&mut *conn, pageable.into_inner())
    })
    .await?
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError);

    Ok(HttpResponse::Ok().json(novels.expect("There should be novels in db")))
}

async fn get_novel_by_url(
    pool: web::Data<DbPool>,
    novel_url: web::Path<String>,
) -> actix_web::Result<impl Responder> {
    let nov = web::block(move || {
        let mut conn = pool.get().expect("Error getting connection to DB");
        novel_service::find_novel_by_url_with_chapters_info(&mut *conn, novel_url.into_inner())
    })
    .await?
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError);

    Ok(HttpResponse::Ok().json(nov.expect("Novel not found")))
}

async fn get_random_novels(pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    let nov = web::block(move || {
        let mut conn = pool.get().expect("Error getting connection to DB");
        novel_service::get_random_novels(&mut *conn)
    })
    .await?
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError);

    Ok(HttpResponse::Ok().json(nov.expect("Novel not found")))
}

async fn search_novel(
    pool: web::Data<DbPool>,
    search: Query<SearchQuery>,
) -> actix_web::Result<impl Responder> {
    let nov = web::block(move || {
        let mut conn = pool.get().expect("Error getting connection to DB");
        novel_service::search(&mut *conn, search.into_inner().search)
    })
    .await?
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError);

    Ok(HttpResponse::Ok().json(nov.expect("Novel not found")))
}

async fn get_novel_review(
    pool: web::Data<DbPool>,
    novel_id: web::Path<u32>,
) -> actix_web::Result<impl Responder> {
    let nov = web::block(move || {
        let mut conn = pool.get().expect("Error getting connection to DB");
        novel_service::find_novel_review(&mut *conn, novel_id.into_inner())
    })
    .await?
    // map diesel query errors to a 500 error response
    .map_err(error::ErrorInternalServerError);

    Ok(HttpResponse::Ok().json(nov.expect("Reviews not found")))
}
