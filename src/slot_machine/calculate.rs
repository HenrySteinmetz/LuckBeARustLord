pub mod item_functions;
pub use item_functions::*;
pub mod look_up_tables;
use look_up_tables::*;
use rand::Rng;
use crate::Item;
use crate::Direction;

pub fn convert_cards(items: Vec<Item>) -> Vec<Item> {
    let mut copy_items = items.clone();
    for i in 0..20 {
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

    for i in 0..20 {
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
    copy_items = convert_cards(copy_items);

    if !indecies_cards.len() == 0 {
        return Some((copy_items, indecies_cards));
    } else {
        return None;
    }
}
// This is the function to add all random symbols and manage, symbol lifetimes
pub fn events<'a>(items: Vec<Item>) ->  (Vec<Item>, Option<Vec<Box<dyn Fn(Vec<Item>) -> Vec<Item> + 'a >>>) {
    let mut ret_items = items.clone();
    let mut ret_add_items_vec: Vec<Box<dyn Fn(Vec<Item>) -> Vec<Item> + 'a>> = vec![];
    for i in 0..20 {
        match items[i] {
            Item::LightBulb(0) => {
                ret_items[i] = Item::Empty;
            }
            Item::Bartender => {
                let mut rng = rand::thread_rng();
                if rng.gen_range(0..10) == 9 {
                    match rng.gen_range(0..4) {
                        0 => {
                            match get_empty(items.clone()).vector_pos {
                                Some(x) => ret_items[x] = Item::ChemicalSeven,
                                None =>  ret_add_items_vec.push(higher_order_add_item(Item::ChemicalSeven)),
                            } 
                        } 
                        1 => {
                            match get_empty(items.clone()).vector_pos {
                                Some(x) => ret_items[x] = Item::Beer,
                                None =>  ret_add_items_vec.push(higher_order_add_item(Item::ChemicalSeven)),
                            } 
                        } 
                        2 => {
                            match get_empty(items.clone()).vector_pos {
                                Some(x) => ret_items[x] = Item::Wine,
                                None =>  ret_add_items_vec.push(higher_order_add_item(Item::ChemicalSeven)),
                            } 
                        } 
                        3 => {
                            match get_empty(items.clone()).vector_pos {
                                Some(x) => ret_items[x] = Item::Martini,
                                None =>  ret_add_items_vec.push(higher_order_add_item(Item::ChemicalSeven)),
                            } 
                        } 
                        _ => (),
                    }
                }
            }
            Item::Beehive => {
                let mut rng = rand::thread_rng();
                if rng.gen_range(0..10) == 9 {
                    match get_empty(items.clone()).vector_pos {
                        Some(x) => ret_items[x] = Item::Honey,
                        None => ret_add_items_vec.push(higher_order_add_item(Item::Honey)),

                    }
                }
            }
            Item::Chicken => {
                let mut rng = rand::thread_rng();
                match rng.gen_range(0..100) {
                    0|1|2|3|4|5|6|7|8|9 => {
                        match get_empty(items.clone()).vector_pos {
                            Some(x) => ret_items[x] = Item::Egg,
                            None => ret_add_items_vec.push(higher_order_add_item(Item::Egg)),
                        }
                    }
                    99 => {
                        match get_empty(items.clone()).vector_pos {
                            Some(x) => ret_items[x] = Item::GoldenEgg,
                            None => ret_add_items_vec.push(higher_order_add_item(Item::GoldenEgg)),
                        }
                    }
                    _ => (),
                }
            }
            Item::Goose => {
                let mut rng = rand::thread_rng();
                    match rng.gen_range(0..100) {
                    99 => {
                        match get_empty(items.clone()).vector_pos {
                            Some(x) => ret_items[x] = Item::GoldenEgg,
                            None => ret_add_items_vec.push(higher_order_add_item(Item::GoldenEgg)),
                        }
                    }
                    _ => (),
                }
            }
            Item::KingMidas =>{
                match get_empty(items.clone()).vector_pos {
                    Some(x) => ret_items[x] = Item::Coin,
                    None => ret_add_items_vec.push(higher_order_add_item(Item::Coin)),
                }
            }
            Item::LightBulb(mut x) => {
                x -= 1;
                ret_items[i] = Item::LightBulb(x);
            } 
            _ => (),
        }
    }
    if !ret_add_items_vec.len() == 0 {
        return (ret_items, Some(ret_add_items_vec))
    }else {
        return (ret_items, None)
    }
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
                ret_vec.push(temp_value as i64)
            }
            Item::Anchor => ret_vec.push(match i {
                0 | 4 | 15 | 19 => 5,
                _ => 1,
            }),
            Item::Apple => ret_vec.push(3),
            Item::Bartender => ret_vec.push(3),
            Item::Bear => {
                let mut val: i64 = 2;
                for x in 0..adjecents.len() {
                    match items[adjecents[x]] {
                        Item::Honey => {
                            mut_copy[adjecents[x]] = Item::Empty;
                            val += 40
                        }
                        _ => (),
                    }
                }
                ret_vec.push(val);
            }
            Item::Beastmaster => ret_vec.push(2),
            Item::Bee => {
                let mut val: i64 = 1;
                for x in 0..adjecents.len() {
                    match items[adjecents[x]] {
                        Item::Flower|Item::Beehive|Item::Honey => {
                            val += 1;
                        },
                        _ => (),
                    }
                }
                ret_vec.push(val);
            }
            Item::Beehive => ret_vec.push(3),
            Item::Beer => ret_vec.push(1),
            Item::Cheese => ret_vec.push(3),
            Item::Cherry => ret_vec.push(1),
            Item::CoconutHalf => ret_vec.push(2),
            Item::Coin => ret_vec.push(1),
            Item::Comedian => ret_vec.push(3),
            Item::Chicken => ret_vec.push(2),
            Item::Clubs => {
                let mut val: i64 = 1;
                for x in 0..adjecents.len() {
                    match items[adjecents[x]] {
                        Item::Clubs|Item::Spades|Item::Diamonds|Item::Hearts => {
                            val += 1;
                        }
                        _ => (),
                    }
                }
                ret_vec.push(val);
            }
            Item::Cultist => {
                let mut val: i64 = 0;
                let mut cultis_count: i64 = 0;
                for x in 0..20 {
                    match items[x] {
                        Item::Cultist => cultis_count += 1,
                        _ => (),
                    }
                }
                match cultis_count {
                    0 => (),
                    1|2 => val = cultis_count,
                    _ => val = cultis_count + 1,
                }
                ret_vec.push(val);
            }
            Item::Diamond => {
                let mut diamond_value: i64 = 5;
                for x in 0..adjecents.len() {
                    if items[adjecents[x]] == Item::Diamond {
                        diamond_value += 1
                    }
                }
                ret_vec.push(diamond_value);
            }
            Item::Diamonds => {
                let mut val: i64 = 1;
                for x in 0..adjecents.len() {
                    match items[adjecents[x]] {
                        Item::Clubs|Item::Spades|Item::Diamonds|Item::Hearts => {
                            val += 1;
                        }
                        _ => (),
                    }
                }
                ret_vec.push(val);
            }
            Item::Dog => {
                let mut val: i64 = 1;
                println!("{:?}", adjecents);
                for x in adjecents {
                    if !val == 3 {
                        match items[x] {
                            Item::RobinHood(_)|Item::Thief(_)|Item::Cultist|Item::Toddler|Item::BountyHunter|Item::Miner|Item::Dwarf|Item::KingMidas|
                            Item::Gambler(_)|Item::GeneralZaroff|Item::Witch|Item::Pirate|Item::Ninja|Item::MrsFruit|Item::Hooligan|Item::Farmer|
                            Item::Diver|Item::Dame|Item::Chef|Item::CardShark|Item::Beastmaster|Item::Geologist(_)|Item::Joker|Item::Comedian|
                            Item::Bartender => val += 2,
                            _ => (),
                        }
                    }
                }
                ret_vec.push(val);
            }
            Item::Farmer => ret_vec.push(2),
            Item::FiveSidedDie => {
                let mut val: i64 = 0;
                let mut rng = rand::thread_rng();
                    match rng.gen_range(0..5) {
                        0 => val +=1,
                        1 => val +=2,
                        2 => val +=3,
                        3 => val +=4,
                        4 => val +=5,
                        _ => (),
                    }
            }
            Item::Flower => ret_vec.push(1),
            Item::GoldenEgg => ret_vec.push(4),
            Item::Goose => ret_vec.push(1),
            Item::Hearts => {
                let mut val: i64 = 1;
                for x in 0..adjecents.len() {
                    match items[adjecents[x]] {
                        Item::Clubs|Item::Spades|Item::Diamonds|Item::Hearts => {
                            val += 1;
                        }
                        _ => (),
                    }
                }
                ret_vec.push(val);
            }
            Item::Honey => ret_vec.push(3),
            Item::Joker => ret_vec.push(3),
            Item::KingMidas => ret_vec.push(1),
            Item::Martini => ret_vec.push(3),
            Item::MatryoshkaDollFive => ret_vec.push(4),
            Item::Milk => ret_vec.push(1),
            Item::Monkey => ret_vec.push(1),
            Item::Pearl => ret_vec.push(1),
            Item::Rain => ret_vec.push(2),
            Item::Sapphire => ret_vec.push(2),
            Item::Spades => {
                let mut val: i64 = 1;
                for x in 0..adjecents.len() {
                    match items[adjecents[x]] {
                        Item::Clubs|Item::Spades|Item::Diamonds|Item::Hearts => {
                            val += 1;
                        }
                        _ => (),
                    }
                }
                ret_vec.push(val);
            }
            Item::Sun => ret_vec.push(3),
            Item::FiveSidedDie => {
                let mut val: i64 = 0;
                let mut rng = rand::thread_rng();
                    match rng.gen_range(0..3) {
                        0 => val +=1,
                        1 => val +=2,
                        2 => val +=3,
                        _ => (),
                    }
            }
            Item::Wolf => ret_vec.push(2),
            Item::BuffingCapsule => ret_vec.push(0), //warum?
            Item::Empty => ret_vec.push(0),
            _ => ret_vec.push(0),
        }
    }
    (mut_copy, ret_vec)
}

pub fn multipliers(items: Vec<Item>, value_vec: Vec<i64>) -> i128 {
    let mut mut_value_vec = value_vec.clone();
    for i in 0..20 {
        let adjecents: Vec<usize> = is_adjecent(i as u8);
        match items[i] {
            Item::Bee => {
                for x in 0..adjecents.len() {
                    match items[adjecents[x]] {
                        Item::Flower|Item::Beehive|Item::Honey => {
                            mut_value_vec[adjecents[x]] *= 2;
                        },
                        _ => (),
                    }
                }
            },
            Item::BuffingCapsule => {
                for x in 0..adjecents.len() {
                    mut_value_vec[adjecents[x]] *= 2;
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
            Item::Comedian => {
                for x in 0..adjecents.len() {
                    match items[adjecents[x]] {
                        Item::Banana|Item::BananaPeel|Item::Dog|Item::Monkey|Item::Toddler|Item::Joker => {
                            mut_value_vec[adjecents[x]] *= 3;
                        },
                        _ => (),
                    }
                }
            }
            Item::Dame => {
                for x in 0..adjecents.len() {
                    match items[adjecents[x]] {
                        Item::Amethyst(_) | Item::Diamond | Item::Emerald | Item::Pearl | Item::Ruby | 
                        Item::Sapphire | Item::ShinyPebble | Item::VoidStone => {
                            mut_value_vec[adjecents[x]] *= 2
                        }
                        _ => (),
                    }
                }
            }
            Item::Flower => {
                for x in 0..adjecents.len() {
                    match items[adjecents[x]] {
                        Item::Rain => {
                            mut_value_vec[adjecents[x]] *= 2
                        }
                        Item::Sun =>  {
                            mut_value_vec[adjecents[x]] *= 5
                        }
                        _ => (),
                    }
                }
            }
            Item::Joker => {
                for x in 0..adjecents.len() {
                    match items[adjecents[x]] {
                        Item::Clubs | Item::Diamonds | Item::Hearts | Item::Spades => {
                            mut_value_vec[adjecents[x]] *= 2
                        }
                        _ => (),
                    }
                }
            }
            Item::KingMidas => {
                for x in 0..adjecents.len() {
                    match items[adjecents[x]] {
                        Item::Coin => {
                            mut_value_vec[adjecents[x]] *= 3;
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
pub fn value_calc<'a>(items: Vec<Item>) -> (i128, Vec<Item>, Option<Vec<Box<dyn Fn(Vec<Item>) -> Vec<Item> + 'a >>>) {
    let (event_items, ret_funcs) = events(items.clone());
    let value_vec = base_value_array(event_items.clone());
    (multipliers(value_vec.0, value_vec.1), event_items, ret_funcs)
}

pub fn postprocessing<'a>(items: Vec<Item>, funcs: Option<Vec<Box<dyn Fn(Vec<Item>) -> Vec<Item> + 'a >>>) -> Vec<Item> {
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
