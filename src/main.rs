// use crate::tag_eng;
mod tag_eng;

use tag_eng::TagId;

use tag_eng::tag_name_store::TagNameStore;

use tag_eng::tagged_object::TaggedObject;

mod handlers;

use actix_web::{middleware, web, App, HttpRequest, HttpServer, HttpResponse, Responder, error, error::Error};
use actix_web::http::StatusCode;
use actix_web::http::header::ContentType;

use log::{debug, error, info, log_enabled, Level};

use std::sync::Arc;
use std::sync::RwLock;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("Starting server NOW");

    let tag_app_state = web::Data::new(handlers::info_structs::AppState {
        tag_database: Arc::new(RwLock::new(TagNameStore::from(&[
            "red", "yellow", "brown", "green", "blue", "black",
        ]))),
        objects: Arc::new(RwLock::new(Vec::new())),
        // objects: Arc::new(RwLock::new(vec![TaggedObject::from(tag_database, "./dogs.txt", &[2])])),
    });

    HttpServer::new(move || {
          let json_config = web::JsonConfig::default()
            .limit(4096)
            .error_handler(|err, _req| {
                // create custom error response
                error::InternalError::from_response(err,
                    HttpResponse::Conflict().finish())
                    // HttpResponse::BadRequest()
                    // .content_type("application/json")
                    // .body(format!(r#"{{"error":"{}"}}"#, err)),
                    // )
                    .into()
            });

        App::new()
            .wrap(middleware::Logger::default())
            .app_data(tag_app_state.clone())
            .app_data(json_config)
            .service(web::resource("/index.html").to(|| async { "Hello World" }))
            .service(web::resource("/").to(handlers::index))
            .route("/jsontest", web::post().to(handlers::jsontest))
            .route("/addtag", web::post().to(handlers::add_tag))
    })
    .workers(4)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
