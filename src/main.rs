// use crate::tag_eng;
mod tag_eng;

use tag_eng::TagId;

use tag_eng::tag_name_store::TagNameStore;

use tag_eng::tagged_object::AddTagError;
use tag_eng::tagged_object::TaggedObject;

use std::sync::Arc;

use actix_web::{middleware, web, App, HttpRequest, HttpServer};
use log::{debug, error, info, log_enabled, Level};
use std::sync::RwLock;

struct AppState {
    tag_database: Arc<RwLock<TagNameStore>>,
    objects: Arc<RwLock<Vec<TaggedObject>>>,
}

async fn index(req: HttpRequest, data: web::Data<AppState>) -> String {
    let tag_database = data.tag_database.read().unwrap();
    println!("REQ: {req:?}");
    match tag_database.name_from_id(2) {
        Some(s) => {
            format!("Tag with id 2 is: {s}")
        }
        None => "No tag with this id exists".to_owned(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("Starting server NOW");

    let tag_app_state = web::Data::new(AppState {
        tag_database: Arc::new(RwLock::new(TagNameStore::from(&[
            "red", "yellow", "brown", "green", "blue", "black",
        ]))),
        objects: Arc::new(RwLock::new(Vec::new())),
    });

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(tag_app_state.clone())
            .service(web::resource("/index.html").to(|| async { "Hello World" }))
            .service(web::resource("/").to(index))
    })
    .workers(4)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
