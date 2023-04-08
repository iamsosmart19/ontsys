// use crate::tag_eng;
mod tag_eng;

use tag_eng::TagId;

use tag_eng::tag_name_store::TagNameStore;

use tag_eng::tagged_object::TaggedObject;

use actix_web::{middleware, web, App, HttpRequest, HttpServer, HttpResponse, Responder, error};
use actix_web::http::StatusCode;
use actix_web::http::header::ContentType;

use serde::Deserialize;
use serde::Serialize;

use log::{debug, error, info, log_enabled, Level};

use std::sync::Arc;
use std::sync::RwLock;

use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
enum MyError {
    #[display(fmt = "internal error")]
    InternalError,

    #[display(fmt = "bad request")]
    BadClientData,

    #[display(fmt = "timeout")]
    Timeout,
}

impl error::ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            MyError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::BadClientData => StatusCode::BAD_REQUEST,
            MyError::Timeout => StatusCode::GATEWAY_TIMEOUT,
        }
    }
}

struct AppState {
    tag_database: Arc<RwLock<TagNameStore>>,
    objects: Arc<RwLock<Vec<TaggedObject>>>,
}

#[derive(Deserialize)]
struct GetTagsInfo {
    tag: String,
}

#[derive(Deserialize,Serialize)]
struct JsonRet {
    tag: TagId,
}

async fn index(req: HttpRequest, data: web::Data<AppState>) -> HttpResponse {
    let tag_database = data.tag_database.read().unwrap();
    println!("REQ: {req:?}");
    let body = match tag_database.name_from_id(2) {
        Some(s) => {
            format!("Tag with id 2 is: {s}")
        }
        None => "No tag with this id exists".to_owned(),
    };
    HttpResponse::Ok().body(body)
}

async fn jsontest(data: web::Data<AppState>, info: web::Json<GetTagsInfo>) -> Result<impl Responder, MyError> {
    let tag_database = data.tag_database.read().unwrap();
    match tag_database.id_from_name(&info.tag) {
        Some(t) => {
            Ok(web::Json(JsonRet {
                tag: t,
            }))
        }
        None => {
            Err(MyError::InternalError)
        }
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
            .service(web::resource("/").to(index))
            .route("/jsontest", web::post().to(jsontest))
    })
    .workers(4)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
