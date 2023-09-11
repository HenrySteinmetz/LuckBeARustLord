use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Item {
    Empty,
    Apple,
    Diamond,
    Anchor,
    Beehive,
    Honey,
    Diamonds,
    Spades,
    Hearts,
    Clubs,
    CardShark,
    Wildcard,
    Amethyst(u16),
    LightBulb(u8),
    Dame,
    BuffingCapsule,
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
        for i in 0..items.len() {
            if items[i] == Item::Empty {
                latest_empty = i;
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
                let items_size: usize = mut_copy.len();
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
    pub fn convert_cards(items: Vec<Item>) -> Vec<Item> {
        let mut copy_items = items.clone();
        for i in 0..items.len() {
            match copy_items[i] {
                Item::Clubs|Item::Spades|Item::Hearts|Item::Diamonds => copy_items[i] = Item::Wildcard,
                _ => (),
            }
        }
        copy_items
    }
    
    pub fn re_add_cards(items: Vec<Item>, cards: Vec<(u8, Item)>) -> Vec<Item> {
        let mut copy_items = items.clone();
        for i in cards {
            copy_items[i.0 as usize] = i.1;
        }
        copy_items
    }

    pub fn preprocessing(items: Vec<Item>) -> Option<(Vec<Item>, Vec<(u8, Item)>)> {
        // Indecies of all cards that are adjecent to a card shark with their type
        let mut indecies_cards: Vec<(u8, Item)> = vec![];
        // Mutable copy of the input vector 
        let mut copy_items: Vec<Item> = items.clone();

        for i in 0..items.len() {
            let adjecents: Vec<u8> = is_adjecent(i as u8);
            match copy_items[i] {
                Item::CardShark => {
                    for x in 0..adjecents.len() {
                        match copy_items[adjecents[x] as usize] {
                            Item::Clubs => indecies_cards.push((adjecents[x], Item::Clubs)),
                            Item::Spades => indecies_cards.push((adjecents[x], Item::Spades)),
                            Item::Hearts => indecies_cards.push((adjecents[x], Item::Hearts)),
                            Item::Diamonds => indecies_cards.push((adjecents[x], Item::Diamonds)),
                            _ => (),
                        }
                    }
                },
                _ => (),
            }
        }
        println!("{}", indecies_cards.len());
        copy_items = Self::convert_cards(copy_items);

        if !indecies_cards.len() == 0 {
            return Some((copy_items, indecies_cards));
        } else {return None;}
    }

    pub fn value_calc(&mut self, items: Vec<Item>) -> (u128, Vec<Item>) {
        let mut val: u128 = 0;
        let mut ret_items = items.clone();
        let mut bord_value_vec: Vec<u64> = vec![];
        for i in 0..20 {
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
                        ret_items = self.add_item(Item::Honey, &mut ret_items);
                    }
                    val += 3
                }
                Item::Honey => val += 3,
                Item::Amethyst(mut amethyst_value) => {
                    let mut temp_value = 0;
                    for x in 0..adjecents.len() {
                       match items[adjecents[x] as usize] {
                           Item::Dame|Item::BuffingCapsule => {
                                amethyst_value += 1;
                                temp_value = amethyst_value * 2;
                           },
                           Item::LightBulb(mut lifetime) => {
                                amethyst_value += 1;
                                temp_value = amethyst_value * 2;
                                lifetime -= 1;
                           },
                           _ => (),
                       }
                    }
                },
                Item::Empty => (),
                _ => (),
            }
        }
        (val, ret_items)
    }

    pub fn calculate(&mut self, items: Vec<Item>) -> (u128, Vec<Item>) {
        let (temp_items, cards): (Vec<Item>, Vec<(u8, Item)>) = 
            Self::preprocessing(items.clone()).unwrap_or((items, vec![]));
        let (val, its) = self.value_calc(temp_items);
        (val, Self::re_add_cards(its, cards))

    }
}
