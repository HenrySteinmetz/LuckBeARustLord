pub mod item_functions;
use item_functions::*;
pub mod look_up_tables;
use look_up_tables::*;
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
    Amethyst(u16),
    Anchor,
    Apple,
    Banana,
    BananaPeel,
    BarOfSoap,
    Bartender,
    Bear,
    Beastmaster,
    Bee,
    Beehive,
    Beer,
    BigOre,
    BigUrn,
    Billionaire,
    BountyHunter,
    BronzeArrow(Direction),
    Bubble,
    BuffingCapsule,
    CardShark,
    Cheese,
    Cherry,
    Clubs,
    CoconutHalf,
    Coin,
    Dame,
    Diamond,
    Diamonds,
    Empty,
    Flower,
    GoldArrow(Direction),
    GoldenEgg,
    Hearts,
    Honey,
    LightBulb(u8),
    Martini,
    MatryoshkaDollFive,
    Milk,
    Monkey,
    SilverArrow(Direction),
    Spades,
    Wildcard,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum State {
    Selecting,
    Paused,
    Normal,
}

pub struct SlotMachine {
    state: State,
}

pub fn roll(items: Vec<Item>) -> Vec<Item> {
    let mut mut_copy = items.clone();
    let mut ret_vec: Vec<Item> = vec![];
    if mut_copy.len() >= 20 {
        let mut rng = rand::thread_rng();
        for _ in 0..20 {
            let items_size: usize = mut_copy.len();
            let rand_num: usize = rng.gen_range(0..items_size);
            let item = mut_copy[rand_num];
            mut_copy.remove(rand_num);
            ret_vec.push(item);
        }
        ret_vec
    } else {
        while mut_copy.len() < 20 {
            mut_copy.push(Item::Empty);
        };
        ret_vec
    }
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
            let adjecents: Vec<usize> = is_adjecent(i as u8);
            match copy_items[i] {
                Item::CardShark => {
                    for x in 0..adjecents.len() {
                        match copy_items[adjecents[x] as usize] {
                            Item::Clubs => indecies_cards.push((adjecents[x] as u8, Item::Clubs)),
                            Item::Spades => indecies_cards.push((adjecents[x] as u8, Item::Spades)),
                            Item::Hearts => indecies_cards.push((adjecents[x] as u8, Item::Hearts)),
                            Item::Diamonds => indecies_cards.push((adjecents[x] as u8, Item::Diamonds)),
                            _ => (),
                        }
                    }
                }
                _ => (),
            }
        }
        copy_items = Self::convert_cards(copy_items);

        if !indecies_cards.len() == 0 {
            return Some((copy_items, indecies_cards));
        } else {
            return None;
        }
    }
    // This is the function to add all random symbols and manage symbol lifetimes
    pub fn events<'a>(items: Vec<Item>) ->  (Vec<Item>, Vec<Box<dyn Fn(Vec<Item>) -> Vec<Item> + 'a >>) {
        let mut ret_items = items.clone();
        let mut ret_add_items_vec: Vec<Box<dyn Fn(Vec<Item>) -> Vec<Item> + 'a>> = vec![];
        for i in 0..20 {
            match ret_items[i] {
                Item::Beehive => {
                    let mut rng = rand::thread_rng();
                    if rng.gen_range(0..10) == 9 {
                        // Wenn ein item mehrere items hinufügt benutz die higher_order_add_items function
                        // hier ein Beispiel:
                        //let items_to_append: Vec<Item> = vec![Item::Apple, Item::Diamond];
                        //ret_add_items_vec.append(&mut higher_order_add_items(items_to_append));
                        ret_add_items_vec.push(higher_order_add_item(Item::Honey)); 
                    }
                }
                Item::LightBulb(0) => {
                    ret_items[i] = Item::Empty;
                }
                Item::LightBulb(mut x) => {
                    x -= 1;
                    ret_items[i] = Item::LightBulb(x);
                } 
                _ => (),
            }
        }
        (ret_items, ret_add_items_vec)
    }

    pub fn base_value_array(items: Vec<Item>) -> (Vec<Item>, Vec<i64>) {
        let mut mut_copy = items.clone();
        let mut ret_vec: Vec<i64> = vec![];
        for i in 0..20 {
            let adjecents: Vec<usize> = is_adjecent(i as u8);
            match items[i] {
                Item::Amethyst(mut amethyst_value) => {
                    let mut temp_value = 0;
                    for x in 0..adjecents.len() {
                        match items[adjecents[x]] {
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
                Item::Anchor => ret_vec.push(match i {
                    0 | 4 | 15 | 19 => 5,
                    _ => 1,
                }),
                Item::Apple => ret_vec.push(3),
                Item::Beastmaster => ret_vec.push(2),
                Item::Bee => {
                    let mut val: i64 = 0;
                    for x in 0..adjecents.len() {
                        match items[adjecents[x]] {
                            Item::Flower|Item::Beehive|Item::Honey => {
                                val += 1;
                            },
                            _ => (),
                        }
                    }
                    ret_vec.push(val);
                },
                Item::Beehive => ret_vec.push(3),
                Item::Cheese => ret_vec.push(3),
                Item::Cherry => ret_vec.push(1),
                Item::CoconutHalf => ret_vec.push(2),
                Item::Coin => ret_vec.push(1),
                Item::Diamond => {
                    let mut diamond_value: i64 = 5;
                    for x in 0..adjecents.len() {
                        if items[adjecents[x]] == Item::Diamond {
                            diamond_value += 1
                        }
                    }
                    ret_vec.push(diamond_value);
                }
                Item::Flower => ret_vec.push(1),
                Item::GoldenEgg => ret_vec.push(4),
                Item::Honey => ret_vec.push(3),
                Item::Martini => ret_vec.push(3),
                Item::MatryoshkaDollFive => ret_vec.push(4),
                Item::Milk => ret_vec.push(1),
                Item::Monkey => ret_vec.push(1),
                Item::BuffingCapsule => ret_vec.push(0),
                Item::Empty => ret_vec.push(0),
                _ => ret_vec.push(0),
            }
        }
        (mut_copy, ret_vec)
    }

    pub fn multipliers(items: Vec<Item>, value_vec: Vec<i64>) -> i128 {
        let mut mut_value_vec = value_vec.clone();
        for i in 0..items.len() {
            let adjecents: Vec<usize> = is_adjecent(i as u8);
            match items[i] {
                Item::BuffingCapsule => {
                    for x in 0..adjecents.len() {
                        mut_value_vec[adjecents[x]] *= 2;
                    }
                },
                Item::Bee => {
                    for x in 0..adjecents.len() {
                        match items[adjecents[x]] {
                            Item::Flower|Item::Beehive|Item::Honey => {
                                mut_value_vec[adjecents[x]] *= 2;
                            },
                            _ => (),
                        }
                    }
                }
                Item::Monkey => {
                    for x in 0..adjecents.len() {
                        match items[adjecents[x]] {
                            // Add items need to be implemented
                            Item::CoconutHalf|Item::Banana => mut_value_vec[i] = value_vec[adjecents[x]] * 6,
                            _ => (),
                        }
                    }
                }
                Item::Dame => {
                    for x in 0..adjecents.len() {
                        match items[adjecents[x]] {
                            Item::Amethyst(_) | Item::Diamond => {
                                mut_value_vec[adjecents[x]] *= 2
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
                    let mut max: i64 = 0;
                    for y in 0..adjecents.len() {
                        if value_vec[adjecents[y] as usize] > max {
                            max = value_vec[adjecents[y] as usize];
                        }
                    }
                    mut_value_vec[i] = max;
                }
                _ => (),
            }
        }
        mut_value_vec.iter().fold(0, |acc, x| acc + *x as i128)
    }
    pub fn value_calc<'a>(&mut self, items: Vec<Item>) -> (i128, Vec<Item>, Vec<Box<dyn Fn(Vec<Item>) -> Vec<Item> + 'a >>) {
        let (event_items, ret_funcs) = Self::events(items.clone());
        let value_vec = Self::base_value_array(event_items.clone());
        (Self::multipliers(value_vec.0, value_vec.1), event_items, ret_funcs)
    }

    pub fn postprocessing<'a>(items: Vec<Item>, funcs: Vec<Box<dyn Fn(Vec<Item>) -> Vec<Item> + 'a >>) -> Vec<Item> {
        let mut mut_copy = items.clone();
        for func in funcs {
            mut_copy = func(mut_copy);
        }
        mut_copy
    }

    pub fn calculate(&mut self, items: Vec<Item>) -> (i128, Vec<Item>, SlotMachine) {
        let (temp_items, cards): (Vec<Item>, Vec<(u8, Item)>) =
            Self::preprocessing(items.clone()).unwrap_or((items, vec![]));
        let (val, its, funcs) = self.value_calc(temp_items);
        
        let ret_items = Self::postprocessing(its, funcs);
        return (
            val,
            Self::re_add_cards(ret_items, cards),
            SlotMachine {state: State::Normal,},
        );
    }
}
