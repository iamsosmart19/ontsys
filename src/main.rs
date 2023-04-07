// use crate::tag_eng;
mod tag_eng;
use actix_web::{middleware, web, App, HttpRequest, HttpServer};
use log::{debug, error, log_enabled, info, Level};


async fn index(req: HttpRequest) -> &'static str {
    println!("REQ: {req:?}");
    "Hello World"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("Starting server NOW");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/index.html").to(|| async { "Hello World" }))
            .service(web::resource("/").to(index))
    })
    .workers(4)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
