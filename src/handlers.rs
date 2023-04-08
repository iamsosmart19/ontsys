pub mod info_structs;

use crate::tag_eng::tagged_object::TaggedObject;

use actix_web::{error, error::Error, web, HttpRequest, HttpResponse, Responder};

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

pub async fn jsontest(
    data: web::Data<info_structs::AppState>,
    info: web::Json<info_structs::GetTagsInfo>,
) -> Result<impl Responder, Error> {
    let tag_database = data.tag_database.read().unwrap();
    match tag_database.id_from_name(&info.tag) {
        Some(t) => Ok(web::Json(info_structs::JsonRet { tag: t })),
        None => Err(error::ErrorBadRequest("tag does not exist")),
    }
}

pub async fn add_tag(
    data: web::Data<info_structs::AppState>,
    info: web::Json<info_structs::AddTagInfo>,
) -> Result<impl Responder, Error> {
    let mut tag_database = data.tag_database.write().unwrap();
    let id = tag_database.add_tag(&info.tag);
    drop(tag_database);

    Ok(web::Json(info_structs::JsonRet { tag: id }))
}

pub async fn add_tagged_object(data: web::Data<info_structs::AppState>, info: web::Json<info_structs::AddTaggedObjectInfo>) -> Result<impl Responder, Error> {
    let tag_database = data.tag_database.read().unwrap();
    let mut tobj = TaggedObject::from(&tag_database, info.filepath.clone(), &[]);
    for x in &info.tags {
        tobj.add_tag_from_id(&tag_database, *x);
    }
    drop(tag_database);
    // let mut tobjs = data.objects.write().unwrap();
    let mut tobjs = data.objects.write().unwrap();
    tobjs.push(tobj);
    Ok("Ok")
}

pub async fn list_tagged_objects(data: web::Data<info_structs::AppState>) -> Result<impl Responder, Error> {
    let mut list: Vec<info_structs::TaggedObjectFlat> = Vec::new();
    let tobjs = data.objects.read().unwrap();
    for obj in tobjs.iter() {
        list.push(info_structs::TaggedObjectFlat::from(obj));
    }
    Ok(web::Json(info_structs::TaggedObjectsList {
        tobjs: list,
    }))
}
