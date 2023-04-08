use crate::tag_eng;

use tag_eng::TagId;

use tag_eng::tag_name_store::TagNameStore;

use std::path::PathBuf;

use std::collections::HashSet;

use itertools::Itertools;

#[derive(PartialEq, Debug)]
pub enum AddTagError {
    TagNotInDatabase,
}

pub struct TaggedObject {
    file_loc: PathBuf,
    tags: HashSet<TagId>,
}
unsafe impl Send for TaggedObject {}
unsafe impl Sync for TaggedObject {}

impl TaggedObject {
    pub fn new() -> Self {
        Self {
            file_loc: PathBuf::new(),
            tags: HashSet::new(),
        }
    }

    pub fn from(tag_store: &TagNameStore, fl: String, tags:&[TagId]) -> Self {
        let mut temp_tags: HashSet<TagId> = HashSet::new();
        for x in tags {
            match tag_store.contains_id(*x) {
                true => {
                    temp_tags.insert(*x);
                }
                false => {
                    println!("{x} not in database");
                }
            }
        }
        Self {
            file_loc: PathBuf::from(fl),
            tags: temp_tags,
        }
    }

    pub fn add_tag_from_id(
        &mut self,
        tag_store: &TagNameStore,
        id: TagId,
    ) -> Result<(), AddTagError> {
        match tag_store.contains_id(id) {
            true => {
                self.tags.insert(id);
                Ok(())
            }
            false => Err(AddTagError::TagNotInDatabase),
        }
    }

    pub fn add_tag_from_str(
        &mut self,
        tag_store: &TagNameStore,
        s: String,
    ) -> Result<(), AddTagError> {
        match tag_store.id_from_name(&s.to_string()) {
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

    pub fn has_tag_str(&self, tag_store: &TagNameStore, s: String) -> bool {
        match tag_store.id_from_name(&s.to_string()) {
            Some(t) => self.tags.contains(&t),
            None => false,
        }
    }

    pub fn rm_tag_id(&mut self, id: TagId) {
        self.tags.remove(&id);
    }

    pub fn rm_tag_str(&mut self, tag_store: &TagNameStore, s: String) {
        match tag_store.id_from_name(&s.to_string()) {
            Some(t) => {
                self.tags.remove(&t);
            }
            None => (),
        }
    }

    pub fn tags_iter(&self) -> impl Iterator<Item = &TagId> + '_ {
        self.tags.iter().sorted()
    }

    pub fn file_loc_str(&self) -> String {
        self.file_loc.clone().into_os_string().into_string().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        let store: TagNameStore =
            TagNameStore::from(&["red", "yellow", "brown", "green", "blue", "black"]);
        let _mfw: TaggedObject = TaggedObject::from(&store, "./myface.png".to_string(), &[]);;
    }

    #[test]
    fn test_add_tag_from_str() {
        let store: TagNameStore =
            TagNameStore::from(&["red", "yellow", "brown", "green", "blue", "black"]);
        let mut mfw: TaggedObject = TaggedObject::from(&store, "./myface.png".to_string(), &[]);;
        match mfw.add_tag_from_str(&store, "blue".to_string()) {
            Err(AddTagError::TagNotInDatabase) => {
                println!("are you dumb")
            }
            _ => {}
        }
        assert!(mfw.has_tag_id(5));
        assert_eq!(
            mfw.add_tag_from_str(&store, "rainbow".to_string()),
            Err(AddTagError::TagNotInDatabase)
        );
    }

    #[test]
    fn test_add_tag_from_id() {
        let store: TagNameStore =
            TagNameStore::from(&["red", "yellow", "brown", "green", "blue", "black"]);
        let mut mfw: TaggedObject = TaggedObject::from(&store, "./myface.png".to_string(), &[]);;

        match mfw.add_tag_from_id(&store, 6) {
            Err(AddTagError::TagNotInDatabase) => {
                println!("are you dumb")
            }
            _ => {}
        }
        assert!(mfw.has_tag_id(6));
        assert_eq!(
            mfw.add_tag_from_id(&store, 100),
            Err(AddTagError::TagNotInDatabase)
        );
    }

    #[test]
    fn test_rm_tag_id() {
        let store: TagNameStore =
            TagNameStore::from(&["red", "yellow", "brown", "green", "blue", "black"]);
        let mut mfw: TaggedObject = TaggedObject::from(&store, "./myface.png".to_string(), &[]);;
        match mfw.add_tag_from_str(&store, "blue".to_string()) {
            Err(AddTagError::TagNotInDatabase) => {
                println!("are you dumb")
            }
            _ => {}
        }
        match mfw.add_tag_from_str(&store, "black".to_string()) {
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
        let store: TagNameStore =
            TagNameStore::from(&["red", "yellow", "brown", "green", "blue", "black"]);
        let mut mfw: TaggedObject = TaggedObject::from(&store, "./myface.png".to_string(), &[]);;
        match mfw.add_tag_from_str(&store, "blue".to_string()) {
            Err(AddTagError::TagNotInDatabase) => {
                println!("are you dumb")
            }
            _ => {}
        }
        match mfw.add_tag_from_str(&store, "black".to_string()) {
            Err(AddTagError::TagNotInDatabase) => {
                println!("are you dumb")
            }
            _ => {}
        }
        mfw.rm_tag_str(&store, "black".to_string());
        mfw.rm_tag_str(&store, "blue".to_string());
        assert!(
            !mfw.has_tag_str(&store, "black".to_string())
                && !mfw.has_tag_str(&store, "blue".to_string())
        );
    }

    #[test]
    fn test_ret_all_tags() {
        let store: TagNameStore =
            TagNameStore::from(&["red", "yellow", "brown", "green", "blue", "black"]);
        let mut mfw: TaggedObject = TaggedObject::from(&store, "./myface.png".to_string(), &[]);;
        for x in mfw.tags_iter() {
            println!("x: {x}");
        }
    }
}
