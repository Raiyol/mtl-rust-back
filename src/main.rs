use actix_web::{App, get, HttpResponse, HttpServer, Responder, web};
use dotenvy::dotenv;

mod models;
mod schema;
mod db;
mod controllers;
mod beans;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let pool = db::init_pool();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(hello)
            .service(
                web::scope("/novel")
                    .configure(controllers::novel::init_routes)
            )
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
