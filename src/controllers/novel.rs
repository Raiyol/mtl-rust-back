use actix_web::{error, HttpResponse, Responder, web};
use diesel::prelude::*;

use crate::db::DbPool;
use crate::models::novel::Novel;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::get().to(get_novels))
    );
}

async fn get_novels(pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    use crate::schema::schema::novel::dsl::*;
    let novels = web::block(move || {
        let mut conn = pool.get().expect("Error getting connection to DB");
        novel.load::<Novel>(&mut conn)
    }).await?
        // map diesel query errors to a 500 error response
        .map_err(error::ErrorInternalServerError);

    Ok(HttpResponse::Ok().json(novels.expect("test")))

    // Ok(match novels {
    //     // user was found; return 200 response with JSON formatted user object
    //     Some(novels) => HttpResponse::Ok().json(novels),
    //
    //     // user was not found; return 404 response with error message
    //     None => HttpResponse::NotFound().body("test"),
    // })
}
