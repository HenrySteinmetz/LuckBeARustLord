use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,     // Oben
    South,     // Unten
    East,      // Rechts
    West,      // Links
    NorthEast, // Oben rechts
    NorthWest, // Oben links
    SouthEast, // Unten rechts
    SouthWest, // Unten links
}

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
    Cheese,
    Cherry,
    CoconutHalf,
    Coin,
    Flower,
    GoldenEgg,
    Martini,
    MatryoshkaDollFive,
    Milk,
    BronzeArrow(Direction),
    SilverArrow(Direction),
    GoldArrow(Direction),
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

pub fn arrow_lookup(arrow: Item, location: u8) -> Option<Vec<u8>> {
    let mut ret_vec: Vec<u8> = vec![];
    let direction = match arrow {
        Item::BronzeArrow(d) => Some(d),
        _ => None,
    }
    .unwrap();

    match direction {
        Direction::North => match location {
            0 | 1 | 2 | 3 | 4 => (),
            5 | 6 | 7 | 8 | 9 => ret_vec.push(location - 5),
            10 | 11 | 12 | 13 | 14 => {
                ret_vec.push(location - 5);
                ret_vec.push(location - 10);
            }
            15 | 16 | 17 | 18 | 19 => {
                ret_vec.push(location - 5);
                ret_vec.push(location - 10);
                ret_vec.push(location - 15);
            }
            _ => panic!("Index out of range"),
        },
        Direction::South => match location {
            15 | 16 | 17 | 18 | 19 => (),
            10 | 11 | 12 | 13 | 14 => ret_vec.push(location + 5),
            5 | 6 | 7 | 8 | 9 => {
                ret_vec.push(location + 5);
                ret_vec.push(location + 10);
            }
            0 | 1 | 2 | 3 | 4 => {
                ret_vec.push(location + 5);
                ret_vec.push(location + 10);
                ret_vec.push(location + 15);
            }
            _ => panic!("Index out of range"),
        },
        Direction::East => match location {
            4 | 9 | 14 | 19 => (),
            3 | 8 | 13 | 18 => ret_vec.push(location - 1),
            2 | 7 | 12 | 17 => {
                ret_vec.push(location - 1);
                ret_vec.push(location - 2);
            }
            1 | 6 | 11 | 16 => {
                ret_vec.push(location - 1);
                ret_vec.push(location - 2);
                ret_vec.push(location - 3);
            }
            0 | 5 | 10 | 15 => {
                ret_vec.push(location - 1);
                ret_vec.push(location - 2);
                ret_vec.push(location - 3);
                ret_vec.push(location - 4);
            }
            _ => panic!("Index out of range"),
        },
        Direction::West => match location {
            0 | 5 | 10 | 15 => (),
            1 | 6 | 11 | 16 => ret_vec.push(location + 1),
            2 | 7 | 12 | 17 => {
                ret_vec.push(location + 1);
                ret_vec.push(location + 2);
            }
            3 | 8 | 13 | 18 => {
                ret_vec.push(location + 1);
                ret_vec.push(location + 2);
                ret_vec.push(location + 3);
            }

            4 | 9 | 14 | 19 => {
                ret_vec.push(location + 1);
                ret_vec.push(location + 2);
                ret_vec.push(location + 3);
                ret_vec.push(location + 4);
            }
            _ => panic!("Index out of range"),
        },
        Direction::NorthEast => match location {
            0 | 1 | 2 | 3 | 4 | 9 | 14 | 19 => (),
            5 | 6 | 7 | 8 | 13 | 18 => ret_vec.push(location - 4),
            10 | 11 | 12 | 17 => {
                ret_vec.push(location - 4);
                ret_vec.push(location - 8);
            }
            15 | 16 => {
                ret_vec.push(location - 4);
                ret_vec.push(location - 8);
                ret_vec.push(location - 12);
            }
            _ => panic!("Index out of range"),
        },
        Direction::NorthWest => match location {
            0 | 1 | 2 | 3 | 4 | 5 | 10 | 15 => (),
            6 | 7 | 8 | 9 | 11 | 16 => ret_vec.push(location - 6),
            12 | 13 | 14 | 17 => {
                ret_vec.push(location - 6);
                ret_vec.push(location - 12);
            }
            18 | 19 => {
                ret_vec.push(location - 6);
                ret_vec.push(location - 12);
                ret_vec.push(location - 18);
            }
            _ => panic!("Index out of range"),
        },
        Direction::SouthEast => match location {
            0 | 5 | 10 | 15 | 16 | 17 | 18 | 19 => (),
            1 | 6 | 11 | 12 | 13 | 14 => ret_vec.push(location + 4),
            2 | 7 | 8 | 9 => {
                ret_vec.push(location + 4);
                ret_vec.push(location + 8);
            }
            3 | 4 => {
                ret_vec.push(location + 4);
                ret_vec.push(location + 8);
                ret_vec.push(location + 12);
            }
            _ => panic!("Index out of range"),
        },
        Direction::SouthWest => match location {
            4 | 9 | 14 | 15 | 16 | 17 | 18 | 19 => (),
            3 | 8 | 11 | 12 | 13 => ret_vec.push(location + 6),
            2 | 5 | 6 | 7 => {
                ret_vec.push(location + 6);
                ret_vec.push(location + 12);
            }
            0 | 1 => {
                ret_vec.push(location + 6);
                ret_vec.push(location + 12);
                ret_vec.push(location + 18);
            }
            _ => panic!("Index out of range"),
        },
    }

    if !ret_vec.len() == 0 {
        return Some(ret_vec);
    } else {
        return None;
    }
}

pub fn is_adjecent(index: u8) -> Vec<u8> {
    match index {
        0 => {
            let vec: Vec<u8> = vec![1, 5, 6];
            return vec;
        }
        1 | 2 | 3 => {
            let vec: Vec<u8> = vec![index - 1, index + 1, index + 4, index + 5, index + 6];
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
        6 | 7 | 8 | 11 | 12 | 13 => {
            let vec: Vec<u8> = vec![
                index - 6,
                index - 5,
                index - 4,
                index - 1,
                index + 1,
                index + 4,
                index + 5,
                index + 6,
            ];
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
        16 | 17 | 18 => {
            let vec: Vec<u8> = vec![index - 6, index - 5, index - 4, index - 1, index + 1];
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

impl SlotMachine {
    pub fn new() -> (SlotMachine, Vec<Item>) {
        let mut items: Vec<Item> = vec![];
        for _ in 0..20 {
            items.push(Item::Empty);
        }
        (
            SlotMachine {
                state: State::Selecting,
            },
            items,
        )
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
                Item::Clubs | Item::Spades | Item::Hearts | Item::Diamonds => {
                    copy_items[i] = Item::Wildcard
                }
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
                }
                _ => (),
            }
        }
        println!("{}", indecies_cards.len());
        copy_items = Self::convert_cards(copy_items);

        if !indecies_cards.len() == 0 {
            return Some((copy_items, indecies_cards));
        } else {
            return None;
        }
    }

    pub fn base_value_array(items: Vec<Item>) -> Vec<u64> {
        let mut ret_vec: Vec<u64> = vec![];
        let mut bord_value_vec: Vec<u64> = vec![];
        for i in 0..20 {
            let adjecents: Vec<u8> = is_adjecent(i as u8);
            match items[i] {
                Item::Apple => ret_vec.push(3),
                Item::Diamond => {
                    let mut diamond_value: u64 = 5;
                    for x in 0..adjecents.len() {
                        if items[adjecents[x] as usize] == Item::Diamond {
                            diamond_value += 1
                        }
                    }
                    ret_vec.push(diamond_value);
                }
                Item::Anchor => ret_vec.push(match i {
                    0 | 4 | 15 | 19 => 5,
                    _ => 1,
                }),
                Item::Beehive => ret_vec.push(3),
                Item::Honey => ret_vec.push(3),
                Item::Amethyst(mut amethyst_value) => {
                    let mut temp_value = 0;
                    for x in 0..adjecents.len() {
                        match items[adjecents[x] as usize] {
                            Item::Dame | Item::BuffingCapsule => {
                                amethyst_value += 1;
                                temp_value = amethyst_value * 2;
                            }
                            Item::LightBulb(mut lifetime) => {
                                amethyst_value += 1;
                                temp_value = amethyst_value * 2;
                                lifetime -= 1;
                            }
                            _ => (),
                        }
                    }
                }
                Item::Cheese => ret_vec.push(3),
                Item::Cherry => ret_vec.push(1),
                Item::CoconutHalf => ret_vec.push(2),
                Item::Coin => ret_vec.push(1),
                Item::Flower => ret_vec.push(1),
                Item::GoldenEgg => ret_vec.push(4),
                Item::Martini => ret_vec.push(3),
                Item::MatryoshkaDollFive => ret_vec.push(4),
                Item::Milk => ret_vec.push(1),
                Item::BuffingCapsule => ret_vec.push(0),
                Item::Empty => (),
                _ => (),
            }
        }
        bord_value_vec
    }

    pub fn multipliers(items: Vec<Item>, value_vec: Vec<u64>) -> u128 {
        let mut mut_value_vec = value_vec.clone();
        for i in 0..items.len() {
            let adjecents: Vec<u8> = is_adjecent(i as u8);
            match items[i] {
                Item::BuffingCapsule => {
                    for x in 0..adjecents.len() {
                        mut_value_vec[adjecents[x] as usize] *= 2;
                    }
                }
                Item::Dame => {
                    for x in 0..adjecents.len() {
                        match items[adjecents[x] as usize] {
                            Item::Amethyst(_) | Item::Diamond => {
                                mut_value_vec[adjecents[x] as usize] *= 2
                            } // das sind nicht alle gemstones wenn neue im enum auftachen füg sie bitte hinzu
                            _ => (),
                        }
                    }
                }
                Item::BronzeArrow(mut d) | Item::SilverArrow(mut d) | Item::GoldArrow(mut d) => {
                    let mut rng = rand::thread_rng();
                    let rand = rng.gen_range(0..8);
                    match rand {
                        0 => d = Direction::North,
                        1 => d = Direction::South,
                        2 => d = Direction::East,
                        3 => d = Direction::West,
                        4 => d = Direction::NorthEast,
                        5 => d = Direction::NorthWest,
                        6 => d = Direction::SouthEast,
                        7 => d = Direction::SouthWest,
                        _ => panic!("Broken rng function"),
                    }
                    let arrow_vec = arrow_lookup(Item::BronzeArrow(d), i as u8);
                    match items[i] {
                        Item::GoldArrow(_) => {
                            for x in arrow_vec.unwrap() {
                                mut_value_vec[x as usize] *= 4;
                            }
                        }
                        Item::SilverArrow(_) => {
                            for x in arrow_vec.unwrap() {
                                mut_value_vec[x as usize] *= 3;
                            }
                        }
                        Item::BronzeArrow(_) => {
                            for x in arrow_vec.unwrap() {
                                mut_value_vec[x as usize] *= 2;
                            }
                        }
                        _ => panic!("Rust comparisin is broken"),
                    }
                }

                // Bitte immer als letztes lassen
                Item::Wildcard => {
                    mut_value_vec[i] = adjecents
                        .iter()
                        .map(|x| value_vec[*x as usize])
                        .max()
                        .unwrap()
                }
                _ => (),
            }
        }
        mut_value_vec.iter().fold(0, |acc, x| acc + *x as u128)
    }
    pub fn value_calc(&mut self, items: Vec<Item>) -> (u128, Vec<Item>) {
        let event_items = events(items.clone());
        let value_vec = Self::base_value_array(event_items.clone());
        (Self::multipliers(items, value_vec), event_items)
    }
    pub fn calculate(&mut self, items: Vec<Item>) -> (u128, Vec<Item>, SlotMachine) {
        let (temp_items, cards): (Vec<Item>, Vec<(u8, Item)>) =
            Self::preprocessing(items.clone()).unwrap_or((items, vec![]));
        let (val, its) = self.value_calc(temp_items);
        return (
            val,
            Self::re_add_cards(its, cards),
            SlotMachine {state: State::Normal,},
        );
    }
}
