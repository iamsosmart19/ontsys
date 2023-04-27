pub mod info_structs;

mod templates;

use crate::tag_eng::tagged_object::TaggedObject;
use crate::tag_eng::TagId;

use sailfish::TemplateOnce;

use actix_web::{error, error::Error, web, HttpRequest, HttpResponse, Responder};

pub async fn index(req: HttpRequest) -> actix_web::Result<impl Responder> {
    let body = templates::Home{}
        .render_once()
        .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().body(body))
}
pub async fn page_tags(req: HttpRequest) -> actix_web::Result<impl Responder> {
    let body = templates::Tags {
		}.render_once()
        .map_err(error::ErrorInternalServerError)?;
    Ok(HttpResponse::Ok().body(body))
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

pub async fn add_tagged_object(
    data: web::Data<info_structs::AppState>,
    info: web::Json<info_structs::AddTaggedObjectInfo>,
) -> Result<impl Responder, Error> {
    let check_name = data.objects.read().unwrap();
    for obj in check_name.iter() {
        if obj.get_name() == info.name {
            drop(check_name);
            return Err(error::ErrorBadRequest("Tag named {info.name} already exists"))
        }
    }
    drop(check_name);
    let tag_database = data.tag_database.read().unwrap();
    let mut tobj = TaggedObject::from(info.name.clone(), &tag_database, info.filepath.clone(), &[]);
    for x in &info.tags {
        tobj.add_tag_from_id(&tag_database, *x);
    }
    drop(tag_database);
    // let mut tobjs = data.objects.write().unwrap();
    let mut tobjs = data.objects.write().unwrap();
    tobjs.push(tobj);
    Ok("Ok")
}

pub async fn list_tagged_objects(
    data: web::Data<info_structs::AppState>,
) -> Result<impl Responder, Error> {
    let mut list: Vec<info_structs::TaggedObjectFlat> = Vec::new();
    let tobjs = data.objects.read().unwrap();
    for obj in tobjs.iter() {
        list.push(info_structs::TaggedObjectFlat::from(obj));
    }
    Ok(web::Json(info_structs::TaggedObjectsList { tobjs: list }))
}

pub async fn list_tags(data: web::Data<info_structs::AppState>) -> Result<impl Responder, Error> {
    let tag_database = data.tag_database.read().unwrap();
    let mut tlist: Vec<TagId> = Vec::new();
    let mut slist: Vec<String> = Vec::new();
    for (t, s) in tag_database.tags_iter() {
        tlist.push(*t);
        slist.push(s.clone());
    }
    Ok(web::Json(info_structs::TagList {
        tlist: tlist,
        slist: slist,
    }))
}

pub async fn and_filter(
    data: web::Data<info_structs::AppState>,
    info: web::Json<info_structs::QueryTagList>,
) -> Result<impl Responder, Error> {
    let mut list: Vec<info_structs::TaggedObjectFlat> = Vec::new();
    let tobjs = data.objects.read().unwrap();
    for obj in tobjs.iter() {
        let mut add: bool = true;
        for t in &info.tags {
            add = add && obj.has_tag_id(*t);
        }
        if add {
            list.push(info_structs::TaggedObjectFlat::from(obj));
        }
    }
    Ok(web::Json(info_structs::TaggedObjectsList { tobjs: list }))
}

pub async fn or_filter(
    data: web::Data<info_structs::AppState>,
    info: web::Json<info_structs::QueryTagList>,
) -> Result<impl Responder, Error> {
    let mut list: Vec<info_structs::TaggedObjectFlat> = Vec::new();
    let tobjs = data.objects.read().unwrap();
    for obj in tobjs.iter() {
        let mut add: bool = false;
        for t in &info.tags {
            add = add || obj.has_tag_id(*t);
        }
        if add {
            list.push(info_structs::TaggedObjectFlat::from(obj));
        }
    }
    Ok(web::Json(info_structs::TaggedObjectsList { tobjs: list }))
}
