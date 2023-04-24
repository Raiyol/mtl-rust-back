use actix_web::{App, get, HttpResponse, HttpServer, post, Responder, web};

mod models;
mod schema;
mod db;
mod controllers;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let connection = &mut db::establish_connection();
    let pool = db::init_pool();

    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(hello)
            .service(echo)
            .service(
                web::scope("/novel")
                    .configure(controllers::novel::init_routes)
            )
            .route("/hey", web::get().to(manual_hello))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
