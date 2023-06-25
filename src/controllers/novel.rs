use actix_web::{error, HttpResponse, Responder, web};
use diesel::prelude::*;
use diesel::query_dsl::QueryDsl;
use validator::Validate;

use crate::beans::pageable::Pageable;
use crate::db::DbPool;
use crate::models::novel::Novel;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::get().to(get_novels))
    );
}

async fn get_novels(pool: web::Data<DbPool>, pageable: web::Json<Pageable>) -> actix_web::Result<impl Responder> {
    use crate::schema::schema::novel::dsl::*;
    let novels = web::block(move || {
        let mut conn = pool.get().expect("Error getting connection to DB");
        novel
            .limit(pageable.page_size)
            .offset(pageable.page_number * pageable.page_size)
            .load::<Novel>(&mut conn)
    }).await?
        // map diesel query errors to a 500 error response
        .map_err(error::ErrorInternalServerError);

    Ok(HttpResponse::Ok().json(novels.expect("There should be novels in db")))
}
