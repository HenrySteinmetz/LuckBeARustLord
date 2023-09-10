use rand::Rng;
use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Item {
    Empty,
    Apple,
    Diamond,
    Anchor,
    Beehive,
    Honey,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum State {
    Roling,
    Selecting,
    Paused,
    Normal,
}

pub struct SlotMachine {
    state: State,
}

pub struct LastEmpty {
    vector_pos: Option<usize>,
}

pub fn is_adjecent(index: u8) -> Vec<u8> {
    match index {
        0 => {
            let vec: Vec<u8> = vec![1, 5, 6];
            return vec;
        }
        1|2|3 => {
            let vec: Vec<u8> = vec![index-1, index+1, index+4, index+5, index+6];
            return vec;
        }
        4 => {
            let vec: Vec<u8> = vec![3, 8, 9];
            return vec;
        }
        5 => { 
            let vec: Vec<u8> = vec![0, 1, 6, 10, 11];
            return vec;
        }
        6|7|8|11|12|13 => {
            let vec: Vec<u8> = vec![index-6, index-5, index-4, index-1, index+1, index+4, index+5, index+6];
            return vec;
        }
        9 => {
            let vec: Vec<u8> = vec![3, 4, 8, 13, 14];
            return vec;
        }
        10 => {
            let vec: Vec<u8> = vec![5, 6, 11, 15, 16];
            return vec;
        }
        14 => {
            let vec: Vec<u8> = vec![8, 9, 13, 18, 19];
            return vec;
        }
        15 => {
            let vec: Vec<u8> = vec![10, 11, 16];
            return vec;
        }
        16|17|18 => {
            let vec: Vec<u8> = vec![index-6, index-5, index-4, index-1, index+1];
            return vec;
        }
        19 => {
            let vec: Vec<u8> = vec![13, 14, 18];
            return vec;
        }
        _ => {
            panic!("Index to high!");
        }
    }
}

impl SlotMachine {
    pub fn new() -> (SlotMachine, Vec<Item>)  {
        let mut items: Vec<Item> = vec![];
        for _ in 0..20 {
            items.push(Item::Empty);
        }
        (SlotMachine {state: State::Selecting}, items)
    }
    
    pub fn get_empty(items: Vec<Item>) -> LastEmpty {
        let mut latest_empty: usize = 999;
        for i in 0..items.len() as u32 {
            if items[i as usize] == Item::Empty {
                latest_empty = i as usize;
            }
        }
        if latest_empty == 999 {LastEmpty{vector_pos: None}}
        else {LastEmpty{vector_pos: Some(latest_empty)}}
    }

    // Is always called at the end of the selection
    pub fn add_item(&mut self, item: Item, items: &mut Vec<Item>) -> Vec<Item> {
        if self.state == State::Selecting {
            let empty = Self::get_empty(items.to_vec());
            match empty.vector_pos {
                Some(_) => {
                    items.remove(empty.vector_pos.unwrap());
                    items.push(item);
                },
                None => items.push(item),
            }
        } else { 
            panic!("Wrong State for adding an item. Expected State Selecting!");
        }
        items.to_vec()
    }
    
    pub fn roll(items: Vec<Item>) -> Vec<Item> {
        let mut mut_copy = items;
        if mut_copy.len() >= 20 {
            let mut ret: Vec<Item> = vec![];
            let mut rng = rand::thread_rng();
            for _ in 0..20 {
                let items_size: usize = usize::try_from(mut_copy.len()).unwrap();
                let rand_num: usize = rng.gen_range(0..items_size);
                let item = mut_copy[rand_num];
                mut_copy.remove(rand_num);
                ret.push(item);
            }
            ret
        } else {
            panic!("Not enough items to roll!");
        }
    }
    pub fn calculate(&mut self, items: &mut Vec<Item>) -> (u128, Vec<Item>) {
        let mut val: u128 = 0;
        let mut ret_items = items.clone();
        for i in 0..items.len() {
            let adjecents: Vec<u8> = is_adjecent(i as u8);
            match items[i] {
                Item::Apple => val += 3,
                Item::Diamond => {
                    let mut diamond_value: u128 = 5;
                    for x in 0..adjecents.len() {
                        if items[adjecents[x] as usize] == Item::Diamond {diamond_value+=1}
                    }
                    val += diamond_value;
                }
                Item::Anchor => val += match i { 
                    0|4|15|19 => 5,
                    _ => 1,
                },
                Item::Beehive => {
                    let mut rng = rand::thread_rng();
                    if rng.gen_range(0..10) == 9 {
                        ret_items = self.add_item(Item::Honey, items);
                    }
                    val += 3
                }
                Item::Honey => val += 3,
                Item::Empty => (),
            }
        }
        (val, ret_items)
    }
}
