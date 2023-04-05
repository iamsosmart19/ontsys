#[allow(dead_code)]
use crate::tag_eng;

use tag_eng::TagId;

use tag_eng::tag_name_store::TagNameStore;

use std::path::PathBuf;
use std::rc::Rc;

use std::collections::HashSet;

pub enum AddTagError {
    TagNotInDatabase,
}

pub struct TaggedObject {
    file_loc: PathBuf,
    tags: HashSet<TagId>,
    tag_store: Rc<TagNameStore>,
}

impl TaggedObject {
    pub fn new() -> Self {
        Self {
            file_loc: PathBuf::new(),
            tags: HashSet::new(),
            tag_store: Rc::new(TagNameStore::new()),
        }
    }

    pub fn from(db: Rc<TagNameStore>, fl: String) -> Self {
        Self {
            file_loc: PathBuf::from(fl),
            tags: HashSet::new(),
            tag_store: Rc::clone(&db),
        }
    }

    pub fn add_tag_from_id(&mut self, id: TagId) -> Result<(), AddTagError> {
        match self.tag_store.contains_id(id) {
            true => {
                self.tags.insert(id);
                Ok(())
            }
            false => Err(AddTagError::TagNotInDatabase),
        }
    }

    pub fn add_tag_from_str(&mut self, s: String) -> Result<(), AddTagError> {
        match self.tag_store.get_tag_id(&s.to_string()) {
            Some(t) => {
                self.tags.insert(t);
                Ok(())
            }
            None => Err(AddTagError::TagNotInDatabase),
        }
    }

    pub fn has_tag_id(&self, id: TagId) -> bool {
        self.tags.contains(&id)
    }

    pub fn has_tag_str(&self, s: String) -> bool {
        match self.tag_store.get_tag_id(&s.to_string()) {
            Some(t) => self.tags.contains(&t),
            None => false,
        }
    }

    pub fn rm_tag_id(&mut self, id: TagId) {
        self.tags.remove(&id);
    }

    pub fn rm_tag_str(&mut self, s: String) {
        match self.tag_store.get_tag_id(&s.to_string()) {
            Some(t) => {
                self.tags.remove(&t);
            }
            None => (),
        }
    }
}
