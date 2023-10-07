use crate::game::Item;

pub struct Empty {
    pub vector_pos: Option<usize>,
}
// The death starts now
pub fn higher_order_add_items<'a>(
    items: Vec<Item>,
) -> Vec<Box<dyn Fn(Vec<Item>) -> Vec<Item> + 'a>> {
    let mut ret_vec: Vec<Box<dyn Fn(Vec<Item>) -> Vec<Item> + 'a>> = vec![];
    for item in items {
        ret_vec.push(Box::new(move |items: Vec<Item>| {
            let mut mut_items = items.clone();
            mut_items.push(item);
            mut_items
        }));
    }
    ret_vec
}

pub fn higher_order_add_item<'a>(item: Item) -> Box<dyn Fn(Vec<Item>) -> Vec<Item> + 'a> {
    Box::new(move |items: Vec<Item>| {
        let mut mut_items = items.clone();
        mut_items.push(item);
        mut_items
    })
}

pub fn get_empty(items: Vec<Item>) -> Empty {
    for i in 0..items.len() {
        if items[i] == Item::Empty {
            return Empty {
                vector_pos: Some(i),
            };
        }
    }
    Empty { vector_pos: None }
}

// Is always called at the end of the selection
pub fn add_item(item: Item, items: Vec<Item>) -> Vec<Item> {
    let mut mut_copy = items.clone();
    let empty = get_empty(items.to_vec());
    match empty.vector_pos {
        Some(_) => {
            mut_copy.remove(empty.vector_pos.unwrap());
            mut_copy.push(item);
        }
        None => mut_copy.push(item),
    }
    mut_copy
}

pub fn remove_empty(items: Vec<Item>) -> Vec<Item> {
    let mut ret_items = items.clone();
    let diff = (19 - items.len() as i32).abs();
    for _ in 0..diff - 1 {
        let maybe_empty = get_empty(ret_items.clone()).vector_pos;
        match maybe_empty {
            Some(_) => {
                ret_items.remove(maybe_empty.unwrap());
            }
            None => (),
        }
    }
    ret_items
}
