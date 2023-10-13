pub mod item_functions;
mod look_up_tables;
use crate::game::slot_machine::Direction;
use crate::game::Symbol;
pub use item_functions::*;
use look_up_tables::*;
use rand::Rng;

pub fn convert_cards(items: Vec<Symbol>) -> Vec<Symbol> {
    let mut copy_items = items.clone();
    for i in 0..20 {
        match copy_items[i] {
            Symbol::Clubs | Symbol::Spades | Symbol::Hearts | Symbol::Diamonds => {
                copy_items[i] = Symbol::Wildcard
            }
            _ => (),
        }
    }
    copy_items
}

pub fn re_add_cards(items: Vec<Symbol>, cards: Vec<(u8, Symbol)>) -> Vec<Symbol> {
    let mut copy_items = items.clone();
    for i in cards {
        copy_items[i.0 as usize] = i.1;
    }
    copy_items
}

pub fn preprocessing(items: Vec<Symbol>) -> Option<(Vec<Symbol>, Vec<(u8, Symbol)>)> {
    // Indecies of all cards that are adjecent to a card shark with their type
    let mut indecies_cards: Vec<(u8, Symbol)> = vec![];
    // Mutable copy of the input vector
    let mut copy_items: Vec<Symbol> = items.clone();

    for i in 0..20 {
        let adjecents: Vec<usize> = is_adjecent(i as u8);
        match copy_items[i] {
            Symbol::CardShark => {
                for x in 0..adjecents.len() {
                    match copy_items[adjecents[x] as usize] {
                        Symbol::Clubs => indecies_cards.push((adjecents[x] as u8, Symbol::Clubs)),
                        Symbol::Spades => indecies_cards.push((adjecents[x] as u8, Symbol::Spades)),
                        Symbol::Hearts => indecies_cards.push((adjecents[x] as u8, Symbol::Hearts)),
                        Symbol::Diamonds => indecies_cards.push((adjecents[x] as u8, Symbol::Diamonds)),
                        _ => (),
                    }
                }
            }
            _ => (),
        }
    }
    copy_items = convert_cards(copy_items);

    if !indecies_cards.len() == 0 {
        return Some((copy_items, indecies_cards));
    } else {
        return None;
    }
}
// This is the function to add all random symbols and manage, symbol lifetimes
pub fn events<'a>(
    items: Vec<Symbol>,
) -> (
    Vec<Symbol>,
    Option<Vec<Box<dyn Fn(Vec<Symbol>) -> Vec<Symbol> + 'a>>>,
) {
    let mut ret_items = items.clone();
    let mut ret_add_items_vec: Vec<Box<dyn Fn(Vec<Symbol>) -> Vec<Symbol> + 'a>> = vec![];
    for i in 0..20 {
        match items[i] {
            Symbol::LightBulb(0) => {
                ret_items[i] = Symbol::Empty;
            }
            Symbol::Bartender => {
                let mut rng = rand::thread_rng();
                if rng.gen_range(0..10) == 9 {
                    match rng.gen_range(0..4) {
                        0 => match get_empty(items.clone()).vector_pos {
                            Some(x) => ret_items[x] = Symbol::ChemicalSeven,
                            None => {
                                ret_add_items_vec.push(higher_order_add_item(Symbol::ChemicalSeven))
                            }
                        },
                        1 => match get_empty(items.clone()).vector_pos {
                            Some(x) => ret_items[x] = Symbol::Beer,
                            None => {
                                ret_add_items_vec.push(higher_order_add_item(Symbol::ChemicalSeven))
                            }
                        },
                        2 => match get_empty(items.clone()).vector_pos {
                            Some(x) => ret_items[x] = Symbol::Wine,
                            None => {
                                ret_add_items_vec.push(higher_order_add_item(Symbol::ChemicalSeven))
                            }
                        },
                        3 => match get_empty(items.clone()).vector_pos {
                            Some(x) => ret_items[x] = Symbol::Martini,
                            None => {
                                ret_add_items_vec.push(higher_order_add_item(Symbol::ChemicalSeven))
                            }
                        },
                        _ => (),
                    }
                }
            }
            Symbol::Beehive => {
                let mut rng = rand::thread_rng();
                if rng.gen_range(0..10) == 9 {
                    match get_empty(items.clone()).vector_pos {
                        Some(x) => ret_items[x] = Symbol::Honey,
                        None => ret_add_items_vec.push(higher_order_add_item(Symbol::Honey)),
                    }
                }
            }
            Symbol::Chicken => {
                let mut rng = rand::thread_rng();
                match rng.gen_range(0..100) {
                    0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 => {
                        match get_empty(items.clone()).vector_pos {
                            Some(x) => ret_items[x] = Symbol::Egg,
                            None => ret_add_items_vec.push(higher_order_add_item(Symbol::Egg)),
                        }
                    }
                    99 => match get_empty(items.clone()).vector_pos {
                        Some(x) => ret_items[x] = Symbol::GoldenEgg,
                        None => ret_add_items_vec.push(higher_order_add_item(Symbol::GoldenEgg)),
                    },
                    _ => (),
                }
            }
            Symbol::Goose => {
                let mut rng = rand::thread_rng();
                match rng.gen_range(0..100) {
                    99 => match get_empty(items.clone()).vector_pos {
                        Some(x) => ret_items[x] = Symbol::GoldenEgg,
                        None => ret_add_items_vec.push(higher_order_add_item(Symbol::GoldenEgg)),
                    },
                    _ => (),
                }
            }
            Symbol::KingMidas => match get_empty(items.clone()).vector_pos {
                Some(x) => ret_items[x] = Symbol::Coin,
                None => ret_add_items_vec.push(higher_order_add_item(Symbol::Coin)),
            },
            Symbol::LightBulb(mut x) => {
                x -= 1;
                ret_items[i] = Symbol::LightBulb(x);
            }
            _ => (),
        }
    }
    if !ret_add_items_vec.len() == 0 {
        return (ret_items, Some(ret_add_items_vec));
    } else {
        return (ret_items, None);
    }
}

pub fn base_value_array(items: Vec<Symbol>) -> (Vec<Symbol>, Vec<i64>) {
    let mut mut_copy = items.clone();
    let mut ret_vec: Vec<i64> = vec![];
    for i in 0..20 {
        let adjecents: Vec<usize> = is_adjecent(i as u8);
        match items[i] {
            Symbol::Amethyst(mut amethyst_value) => {
                let mut temp_value = 0;
                for x in 0..adjecents.len() {
                    match items[adjecents[x]] {
                        Symbol::Dame | Symbol::BuffingCapsule => {
                            amethyst_value += 1;
                            temp_value = amethyst_value * 2;
                        }
                        Symbol::LightBulb(mut lifetime) => {
                            amethyst_value += 1;
                            temp_value = amethyst_value * 2;
                            lifetime -= 1;
                        }
                        _ => (),
                    }
                }
                ret_vec.push(temp_value as i64)
            }
            Symbol::Anchor => ret_vec.push(match i {
                0 | 4 | 15 | 19 => 5,
                _ => 1,
            }),
            Symbol::Apple => ret_vec.push(3),
            Symbol::Bartender => ret_vec.push(3),
            Symbol::Bear => {
                let mut val: i64 = 2;
                for x in 0..adjecents.len() {
                    match items[adjecents[x]] {
                        Symbol::Honey => {
                            mut_copy[adjecents[x]] = Symbol::Empty;
                            val += 40
                        }
                        _ => (),
                    }
                }
                ret_vec.push(val);
            }
            Symbol::Beastmaster => ret_vec.push(2),
            Symbol::Bee => {
                let mut val: i64 = 1;
                for x in 0..adjecents.len() {
                    match items[adjecents[x]] {
                        Symbol::Flower | Symbol::Beehive | Symbol::Honey => {
                            val += 1;
                        }
                        _ => (),
                    }
                }
                ret_vec.push(val);
            }
            Symbol::Beehive => ret_vec.push(3),
            Symbol::Beer => ret_vec.push(1),
            Symbol::Cheese => ret_vec.push(3),
            Symbol::Cherry => ret_vec.push(1),
            Symbol::CoconutHalf => ret_vec.push(2),
            Symbol::Coin => ret_vec.push(1),
            Symbol::Comedian => ret_vec.push(3),
            Symbol::Chicken => ret_vec.push(2),
            Symbol::Clubs => {
                let mut val: i64 = 1;
                for x in 0..adjecents.len() {
                    match items[adjecents[x]] {
                        Symbol::Clubs | Symbol::Spades | Symbol::Diamonds | Symbol::Hearts => {
                            val += 1;
                        }
                        _ => (),
                    }
                }
                ret_vec.push(val);
            }
            Symbol::Cultist => {
                let mut val: i64 = 0;
                let mut cultis_count: i64 = 0;
                for x in 0..20 {
                    match items[x] {
                        Symbol::Cultist => cultis_count += 1,
                        _ => (),
                    }
                }
                match cultis_count {
                    0 => (),
                    1 | 2 => val = cultis_count,
                    _ => val = cultis_count + 1,
                }
                ret_vec.push(val);
            }
            Symbol::Diamond => {
                let mut diamond_value: i64 = 5;
                for x in 0..adjecents.len() {
                    if items[adjecents[x]] == Symbol::Diamond {
                        diamond_value += 1
                    }
                }
                ret_vec.push(diamond_value);
            }
            Symbol::Diamonds => {
                let mut val: i64 = 1;
                for x in 0..adjecents.len() {
                    match items[adjecents[x]] {
                        Symbol::Clubs | Symbol::Spades | Symbol::Diamonds | Symbol::Hearts => {
                            val += 1;
                        }
                        _ => (),
                    }
                }
                ret_vec.push(val);
            }
            Symbol::Dog => {
                let mut val: i64 = 1;
                for x in adjecents {
                    if !val == 3 {
                        match items[x] {
                            Symbol::RobinHood(_)
                            | Symbol::Thief(_)
                            | Symbol::Cultist
                            | Symbol::Toddler
                            | Symbol::BountyHunter
                            | Symbol::Miner
                            | Symbol::Dwarf
                            | Symbol::KingMidas
                            | Symbol::Gambler(_)
                            | Symbol::GeneralZaroff
                            | Symbol::Witch
                            | Symbol::Pirate(_)
                            | Symbol::Ninja
                            | Symbol::MrsFruit
                            | Symbol::Hooligan
                            | Symbol::Farmer
                            | Symbol::Diver
                            | Symbol::Dame
                            | Symbol::Chef
                            | Symbol::CardShark
                            | Symbol::Beastmaster
                            | Symbol::Geologist(_)
                            | Symbol::Joker
                            | Symbol::Comedian
                            | Symbol::Bartender => val += 2,
                            _ => (),
                        }
                    }
                }
                ret_vec.push(val);
            }
            Symbol::Farmer => ret_vec.push(2),
            Symbol::FiveSidedDie => {
                let val: i64 = rand::thread_rng().gen_range(0..5);
                ret_vec.push(val);
            }
            Symbol::Flower => ret_vec.push(1),
            Symbol::GoldenEgg => ret_vec.push(4),
            Symbol::Goose => ret_vec.push(1),
            Symbol::Hearts => {
                let mut val: i64 = 1;
                for x in 0..adjecents.len() {
                    match items[adjecents[x]] {
                        Symbol::Clubs | Symbol::Spades | Symbol::Diamonds | Symbol::Hearts => {
                            val += 1;
                        }
                        _ => (),
                    }
                }
                ret_vec.push(val);
            }
            Symbol::Honey => ret_vec.push(3),
            Symbol::Joker => ret_vec.push(3),
            Symbol::KingMidas => ret_vec.push(1),
            Symbol::Martini => ret_vec.push(3),
            Symbol::MatryoshkaDollFive => ret_vec.push(4),
            Symbol::Milk => ret_vec.push(1),
            Symbol::Monkey => ret_vec.push(1),
            Symbol::Pearl => ret_vec.push(1),
            Symbol::Rain => ret_vec.push(2),
            Symbol::Sapphire => ret_vec.push(2),
            Symbol::Spades => {
                let mut val: i64 = 1;
                for x in 0..adjecents.len() {
                    match items[adjecents[x]] {
                        Symbol::Clubs | Symbol::Spades | Symbol::Diamonds | Symbol::Hearts => {
                            val += 1;
                        }
                        _ => (),
                    }
                }
                ret_vec.push(val);
            }
            Symbol::Sun => ret_vec.push(3),
            Symbol::Wolf => ret_vec.push(2),
            Symbol::BuffingCapsule => ret_vec.push(0), //warum? weil die Buffing capsule selbst
            //keinen wert hat sondern nur den Wert aller
            //angrenzenden Symbole verdoppelt
            Symbol::Empty => ret_vec.push(0),
            _ => ret_vec.push(0),
        }
    }
    (mut_copy, ret_vec)
}

pub fn multipliers(items: Vec<Symbol>, value_vec: Vec<i64>) -> i128 {
    let mut mut_value_vec = value_vec.clone();
    for i in 0..20 {
        let adjecents: Vec<usize> = is_adjecent(i as u8);
        match items[i] {
            Symbol::Bee => {
                let _ = adjecents.iter().map(|x| match items[*x] {
                    Symbol::Flower | Symbol::Beehive | Symbol::Honey => mut_value_vec[*x] *= 2,
                    _ => (),
                });
            }
            Symbol::BuffingCapsule => {
                let _ = adjecents.iter().map(|a| mut_value_vec[*a] *= 2);
            }
            Symbol::BronzeArrow(mut d) | Symbol::SilverArrow(mut d) | Symbol::GoldenArrow(mut d) => {
                match rand::thread_rng().gen_range(0..8) {
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
                let arrow_vec = arrow_lookup(Symbol::BronzeArrow(d), i as u8);
                match items[i] {
                    Symbol::GoldenArrow(_) => {
                        let _ = arrow_vec
                            .unwrap()
                            .into_iter()
                            .map(|x| mut_value_vec[x as usize] *= 4);
                    }
                    Symbol::SilverArrow(_) => {
                        let _ = arrow_vec
                            .unwrap()
                            .into_iter()
                            .map(|x| mut_value_vec[x as usize] *= 3);
                    }
                    Symbol::BronzeArrow(_) => {
                        let _ = arrow_vec
                            .unwrap()
                            .into_iter()
                            .map(|x| mut_value_vec[x as usize] *= 2);
                    }
                    _ => unreachable!("Rust comparisin is broken"),
                }
            }
            Symbol::Comedian => {
                let _ = adjecents.into_iter().map(|x| {
                    match items[x] {
                        Symbol::Banana
                        | Symbol::BananaPeel
                        | Symbol::Dog
                        | Symbol::Monkey
                        | Symbol::Toddler
                        | Symbol::Joker => {
                            mut_value_vec[x] *= 3;
                        }
                        _ => (),
                    }
                });
            }
            Symbol::Dame => {
                let _ = adjecents.into_iter().map(|x| {
                    match items[x] {
                        Symbol::Amethyst(_)
                        | Symbol::Diamond
                        | Symbol::Emerald
                        | Symbol::Pearl
                        | Symbol::Ruby
                        | Symbol::Sapphire
                        | Symbol::ShinyPebble
                        | Symbol::VoidStone => mut_value_vec[x] *= 2,
                        _ => (),
                    }
                });

                
            }
            Symbol::Joker => {
                let _ = adjecents.into_iter().map(|x| {
                    match items[x] {
                        Symbol::Clubs | Symbol::Diamonds | Symbol::Hearts | Symbol::Spades => {
                            mut_value_vec[x] *= 2
                        }
                        _ => (),
                    }
                });

            }
            Symbol::KingMidas => {
                let _ = adjecents.into_iter().map(|x| {
                    match items[x] {
                        Symbol::Coin => {
                            mut_value_vec[x] *= 3;
                        }
                        _ => (),
                    }
                });
            }
            Symbol::Monkey => {
                let _ = adjecents.into_iter().map(|x| {
                    match items[x] {
                        Symbol::CoconutHalf | Symbol::Banana => {
                            mut_value_vec[i] = value_vec[x] * 6
                        }
                        _ => (),
                    }
                });

            }
            // Bitte immer als letztes lassen
            Symbol::Wildcard => {
                let mut max: i64 = 0;
                let _ = adjecents.into_iter().map(|x| {
                    if mut_value_vec[x] > max {
                        max = mut_value_vec[x];
                    }
                });
                mut_value_vec[i] = max;
            }
            _ => (),
        }
    }
    mut_value_vec.iter().fold(0, |acc, x| acc + *x as i128)
}
pub fn value_calc<'a>(
    items: Vec<Symbol>,
) -> (
    i128,
    Vec<Symbol>,
    Option<Vec<Box<dyn Fn(Vec<Symbol>) -> Vec<Symbol> + 'a>>>,
) {
    let (event_items, ret_funcs) = events(items.clone());
    let value_vec = base_value_array(event_items.clone());
    (
        multipliers(value_vec.0, value_vec.1),
        event_items,
        ret_funcs,
    )
}

pub fn postprocessing<'a>(
    items: Vec<Symbol>,
    funcs: Option<Vec<Box<dyn Fn(Vec<Symbol>) -> Vec<Symbol> + 'a>>>,
) -> Vec<Symbol> {
    let mut mut_copy = items.clone();
    match funcs {
        Some(_) => {
            for func in funcs.unwrap() {
                mut_copy = func(mut_copy);
            }
            return mut_copy;
        }
        None => return mut_copy,
    }
}

pub fn roll(items: Vec<Symbol>) -> (Vec<Symbol>, Option<Vec<Symbol>>) {
    let mut mut_copy = items.clone();
    let mut ret_vec: Vec<Symbol> = vec![];
    if mut_copy.len() >= 20 {
        let mut removed = vec![];
        for _ in 0..20 {
            let items_size: usize = mut_copy.len();
            let rand_num: usize = rand::thread_rng().gen_range(0..items_size);
            let item = mut_copy[rand_num];
            removed.push(mut_copy.remove(rand_num));
            ret_vec.push(item);
        }
        (ret_vec, Some(removed))
    } else {
        while mut_copy.len() < 20 {
            mut_copy.push(Symbol::Empty);
        }
        (ret_vec, None)
    }
}
