// use crate::tag_eng;
mod tag_eng;

use tag_eng::TagId;

use tag_eng::tag_name_store::TagNameStore;

use tag_eng::tagged_object::AddTagError;
use tag_eng::tagged_object::TaggedObject;

use std::rc::Rc;

fn main() {
    let mut store: TagNameStore = TagNameStore::from(&["red", "yellow", "brown", "green", "blue"]);

    // let purple: TagId = store.borrow_mut().add_tag("purple");
    // let black: TagId = store.borrow_mut().add_tag("black");
    let purple: TagId = store.add_tag("purple");
    let black: TagId = store.add_tag("black");

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
                println!("The tag with id '{i}' is {}.", t);
            }
            None => {
                println!("The tag with id '{}' does not exist in this database", i);
            }
        }
    }

    let mut mfw: TaggedObject = TaggedObject::from(Rc::new(store), "./myface.png".to_string());
    match mfw.add_tag_from_str("blue".to_string()) {
        Err(AddTagError::TagNotInDatabase) => {
            println!("are you dumb")
        }
        _ => {}
    }
    match mfw.add_tag_from_str("rainbow".to_string()) {
        Err(AddTagError::TagNotInDatabase) => {
            println!("are you dumb")
        }
        _ => {}
    }
    match mfw.add_tag_from_id(black) {
        Err(AddTagError::TagNotInDatabase) => {
            println!("are you dumb")
        }
        _ => {}
    }
    if mfw.has_tag_id(black) && mfw.has_tag_str("blue".to_string()) {
        println!("Yeah but you should see the other guy");
    }
    mfw.rm_tag_id(black);
    mfw.rm_tag_str("blue".to_string());
    if !mfw.has_tag_id(black) && !mfw.has_tag_str("blue".to_string()) {
        println!("I do not recall");
    }
}
