use crate::tag_eng;

use tag_eng::TagId;

use tag_eng::tag_name_store::TagNameStore;

use std::path::PathBuf;
use std::rc::Rc;

use std::collections::HashSet;

#[derive(PartialEq,Debug)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        let store: TagNameStore = TagNameStore::from(&["red", "yellow", "brown", "green", "blue", "black"]);
        let mfw: TaggedObject = TaggedObject::from(Rc::new(store), "./myface.png".to_string());
    }

    #[test]
    fn test_add_tag_from_str() {
        let store: TagNameStore = TagNameStore::from(&["red", "yellow", "brown", "green", "blue", "black"]);
        let mut mfw: TaggedObject = TaggedObject::from(Rc::new(store), "./myface.png".to_string());
        match mfw.add_tag_from_str("blue".to_string()) {
            Err(AddTagError::TagNotInDatabase) => {
                println!("are you dumb")
            }
            _ => {}
        }
        assert!(mfw.has_tag_id(5));
        assert_eq!(mfw.add_tag_from_str("rainbow".to_string()), Err(AddTagError::TagNotInDatabase));
    }

    #[test]
    fn test_add_tag_from_id() {
        let store: TagNameStore = TagNameStore::from(&["red", "yellow", "brown", "green", "blue", "black"]);
        let mut mfw: TaggedObject = TaggedObject::from(Rc::new(store), "./myface.png".to_string());
        
        match mfw.add_tag_from_id(6) {
            Err(AddTagError::TagNotInDatabase) => {
                println!("are you dumb")
            }
            _ => {}
        }
        assert!(mfw.has_tag_id(6));
        assert_eq!(mfw.add_tag_from_id(100),Err(AddTagError::TagNotInDatabase));
    }

    #[test]
    fn test_rm_tag_id() {
        let store: TagNameStore = TagNameStore::from(&["red", "yellow", "brown", "green", "blue", "black"]);
        let mut mfw: TaggedObject = TaggedObject::from(Rc::new(store), "./myface.png".to_string());
        match mfw.add_tag_from_str("blue".to_string()) {
            Err(AddTagError::TagNotInDatabase) => {
                println!("are you dumb")
            }
            _ => {}
        }
        match mfw.add_tag_from_str("black".to_string()) {
            Err(AddTagError::TagNotInDatabase) => {
                println!("are you dumb")
            }
            _ => {}
        }
        mfw.rm_tag_id(5);
        mfw.rm_tag_id(6);
        assert!(!mfw.has_tag_id(6) && !mfw.has_tag_id(5));
    }

    #[test]
    fn test_rm_tag_str() {
        let store: TagNameStore = TagNameStore::from(&["red", "yellow", "brown", "green", "blue", "black"]);
        let mut mfw: TaggedObject = TaggedObject::from(Rc::new(store), "./myface.png".to_string());
        match mfw.add_tag_from_str("blue".to_string()) {
            Err(AddTagError::TagNotInDatabase) => {
                println!("are you dumb")
            }
            _ => {}
        }
        match mfw.add_tag_from_str("black".to_string()) {
            Err(AddTagError::TagNotInDatabase) => {
                println!("are you dumb")
            }
            _ => {}
        }
        mfw.rm_tag_str("black".to_string());
        mfw.rm_tag_str("blue".to_string());
        assert!(!mfw.has_tag_str("black".to_string()) && !mfw.has_tag_str("blue".to_string()));
    }
}
