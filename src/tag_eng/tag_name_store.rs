use crate::tag_eng;

use tag_eng::TagId;

use std::collections::HashMap;

pub struct TagNameStore {
    id_lookup: HashMap<TagId, String>,
    str_lookup: HashMap<String, TagId>,
    cur_index: TagId,
}

impl TagNameStore {
    pub fn new() -> Self {
        Self {
            id_lookup: HashMap::new(),
            str_lookup: HashMap::new(),
            cur_index: 1,
        }
    }

    pub fn from(lst: &[&str]) -> Self {
        let mut cnt: TagId = 1;
        let mut idhsh: HashMap<TagId, String> = HashMap::new();
        let mut strhsh: HashMap<String, TagId> = HashMap::new();
        for tag in lst {
            idhsh.insert(cnt, tag.to_string());
            strhsh.insert(tag.to_string(), cnt);
            cnt += 1;
        }
        Self {
            id_lookup: idhsh,
            str_lookup: strhsh,
            cur_index: cnt,
        }
    }

    // Adds tag to database, returns Id of tag added
    // If tag already exists, it just returns its id
    pub fn add_tag(&mut self, name: &str) -> TagId {
        match self.str_lookup.get(&name.to_string()) {
            Some(t) => *t,
            None => {
                self.id_lookup.insert(self.cur_index, name.to_string());
                self.str_lookup.insert(name.to_string(), self.cur_index);
                let s = self.cur_index;
                self.cur_index += 1;
                s
            }
        }
    }

    // Finds tag in database and returns its id
    // Returns None if tag not found
    pub fn get_tag_id(&self, name: &str) -> Option<TagId> {
        match self.str_lookup.get(&name.to_string()) {
            Some(t) => return Some(*t),
            None => None,
        }
    }

    pub fn name_from_id(&self, t: TagId) -> Option<String> {
        match self.id_lookup.get(&t) {
            Some(a) => Some(a.to_string()),
            None => None,
        }
    }
}
