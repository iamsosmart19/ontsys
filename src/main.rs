// use crate::tag_eng;
mod tag_eng;

use tag_eng::TagId;

use tag_eng::tag_name_store::TagNameStore;

fn main() {
    let mut store: TagNameStore = TagNameStore::from(&["red", "yellow", "brown", "green", "blue"]);

    let purple: TagId = store.add_tag("purple");

    println!("The id of the tag 'purple' is {}.", purple);
    match store.get_tag_id("yellow") {
        Some(t) => {
            println!("The id of the tag 'yellow' is {}.", t);
        }
        None => {
            println!("The tag yellow does not exist in this database.")
        }
    }
    for i in 1..20 {
        match store.name_from_id(i) {
            Some(t) => {
                println!("The tag with id 'i' is {}.", t);
            }
            None => {
                println!("The tag with id '{}' does not exist in this database",i);
            }
        }
    }
}
