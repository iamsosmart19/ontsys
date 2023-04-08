pub mod info_structs;

use actix_web::{middleware, web, App, HttpRequest, HttpServer, HttpResponse, Responder, error, error::Error};

pub async fn index(req: HttpRequest, data: web::Data<info_structs::AppState>) -> HttpResponse {
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

pub async fn jsontest(data: web::Data<info_structs::AppState>, info: web::Json<info_structs::GetTagsInfo>) -> Result<impl Responder, Error> {
    let tag_database = data.tag_database.read().unwrap();
    match tag_database.id_from_name(&info.tag) {
        Some(t) => {
            Ok(web::Json(info_structs::JsonRet {
                tag: t,
            }))
        }
        None => {
            Err(error::ErrorBadRequest("tag does not exist"))
        }
    }
}

pub async fn add_tag(data: web::Data<info_structs::AppState>, info: web::Json<info_structs::AddTagInfo>) -> Result<impl Responder, Error> {
    let mut tag_database = data.tag_database.write().unwrap();
    let id = tag_database.add_tag(&info.tag);
    drop(tag_database);

    Ok(web::Json(info_structs::JsonRet {
        tag: id,
    }))
}
