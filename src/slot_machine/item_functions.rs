use crate::Item;
use rand::Rng;

pub struct LastEmpty {
    vector_pos: Option<usize>,
}
// This is the function to add all random symbols and manage symbol lifetimes
pub fn events(items: Vec<Item>) -> Vec<Item> {
    let mut ret_items = items.clone();
    let mut emptys_to_remove: u8 = 0;
    for i in 0..20 {
        match ret_items[i] {
            Item::Beehive => {
                let mut rng = rand::thread_rng();
                if rng.gen_range(0..10) == 9 {
                    ret_items = add_item(Item::Honey, &mut ret_items);
                }
            }
            Item::LightBulb(0) => {
                emptys_to_remove += 1;
                ret_items[i] = Item::Empty;
            }
            Item::LightBulb(mut x) => x -= 1,
            _ => (),
        }
    }
    ret_items
}
pub fn get_empty(items: Vec<Item>) -> LastEmpty {
    let mut latest_empty: usize = 999;
    for i in 0..items.len() {
        if items[i] == Item::Empty {
            latest_empty = i;
        }
    }
    if latest_empty == 999 {
        LastEmpty { vector_pos: None }
    } else {
        LastEmpty {
            vector_pos: Some(latest_empty),
        }
    }
}
// Is always called at the end of the selection
pub fn add_item(item: Item, items: &mut Vec<Item>) -> Vec<Item> {
    let empty = get_empty(items.to_vec());
    match empty.vector_pos {
        Some(_) => {
            items.remove(empty.vector_pos.unwrap());
            items.push(item);
        }
        None => items.push(item),
    }
    items.to_vec()
}
pub fn remove_empty(items: Vec<Item>) -> Vec<Item> {
    let mut ret_items = items.clone();
    while items.len() >= 20 {
        ret_items.remove(get_empty(ret_items.clone()).vector_pos.unwrap());
    }
    ret_items
}
