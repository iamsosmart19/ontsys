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

