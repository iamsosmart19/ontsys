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

    pub fn contains_id(&self, t: TagId) -> bool {
        self.id_lookup.contains_key(&t)
    }

    // Finds tag in database from string and returns its id
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        let mut store: TagNameStore = TagNameStore::from(&["red", "yellow", "brown", "green", "blue"]);
    }

    #[test]
    fn test_add_tag() {
        let mut store: TagNameStore = TagNameStore::from(&["red", "yellow", "brown", "green", "blue"]);
        let purple: TagId = store.add_tag("purple");
        assert!(store.contains_id(purple));
    }

    #[test]
    fn test_get_tag_id() {
        let mut store: TagNameStore = TagNameStore::from(&["red", "yellow", "brown", "green", "blue"]);
        assert_eq!(store.get_tag_id("yellow"),Some(2));
        for i in 1..20 {
            match store.name_from_id(i) {
                Some(s) => {
                    match i {
                        1 => {
                            assert_eq!(s, "red");
                        }
                        2 => {
                            assert_eq!(s, "yellow");
                        }
                        3 => {
                            assert_eq!(s, "brown");
                        }
                        4 => {
                            assert_eq!(s, "green");
                        }
                        5 => {
                            assert_eq!(s, "blue");
                        }
                        _ => {}
                    }
                }
                None => {
                    assert!(i>=5);
                }
            }
        }
    }
}
