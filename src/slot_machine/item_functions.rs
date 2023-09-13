use crate::Item;

pub struct LastEmpty {
    pub vector_pos: Option<usize>,
}
// The death starts now
pub fn higher_order_add_items<'a>(items: Vec<Item>) -> Vec<Box<dyn Fn(Vec<Item>) -> Vec<Item> + 'a >> {
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
pub fn higher_order_add_item<'a>(item: Item ) -> Box<dyn Fn(Vec<Item>) -> Vec<Item> + 'a > {
        Box::new(move |items: Vec<Item>| {
            let mut mut_items = items.clone();
            mut_items.push(item);
            mut_items
        })
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
