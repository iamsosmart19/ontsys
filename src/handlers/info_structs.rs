use std::sync::{Arc,RwLock};

use crate::tag_eng::tag_name_store::TagNameStore;
use crate::tag_eng::tagged_object::TaggedObject;
use crate::tag_eng::TagId;

use serde::Deserialize;
use serde::Serialize;

pub struct AppState {
    pub tag_database: Arc<RwLock<TagNameStore>>,
    pub objects: Arc<RwLock<Vec<TaggedObject>>>,
}

#[derive(Deserialize)]
pub struct GetTagsInfo {
    pub tag: String,
}

#[derive(Deserialize,Serialize)]
pub struct JsonRet {
    pub tag: TagId,
}

#[derive(Deserialize,Serialize)]
pub struct AddTagInfo {
    pub tag: String,
}

#[derive(Deserialize,Serialize)]
pub struct AddTaggedObjectInfo {
    pub filepath: String,
    pub tags: Vec<TagId>,
}

#[derive(Deserialize,Serialize)]
pub struct TaggedObjectFlat {
    pub filepath: String,
    pub tags: Vec<TagId>,
}

impl TaggedObjectFlat {
    pub fn new() -> Self {
        Self {
            filepath: String::new(),
            tags: Vec::new(),
        }
    }

    pub fn from(obj: &TaggedObject) -> Self {
        let mut tag_vec: Vec<TagId> = Vec::new();
        for t in obj.tags_iter() {
            tag_vec.push(*t);
        }
        Self {
            filepath: obj.file_loc_str(),
            tags: tag_vec,
        }
    }
}

#[derive(Deserialize,Serialize)]
pub struct TaggedObjectsList {
    pub tobjs: Vec<TaggedObjectFlat>,
}
