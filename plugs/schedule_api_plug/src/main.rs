mod endpoints;

use endpoints::schedule::*;
use actix_web::{App, post, get, web, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;
use env_logger::Env;

#[get("/health")]
async fn get_health() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(get_health)
            .service(get_schedule)
            .service(edit_schedule)
            .service(create_schedule)
    }).bind(("127.0.0.1", 8080))?.run().await
}
