use actix_web::{App, get, HttpResponse, HttpServer, Responder, web, middleware::Logger};
use dotenvy::dotenv;

mod models;
mod schema;
mod db;
mod controllers;
mod services;
mod beans;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let pool = db::init_pool();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(hello)
            .service(
                web::scope("/api/novels")
                    .configure(controllers::novel::init_novels_routes)
            )
            .service(
                web::scope("/api/chapters")
                    .configure(controllers::chapter::init_chapters_routes)
            )
            .wrap(Logger::default())
    })
        .bind(("127.0.0.1", 5000))?
        .run()
        .await
}
