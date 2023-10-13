use crate::game::slot_machine::Direction;
use crate::game::Symbol;

pub fn arrow_lookup(arrow: Symbol, location: u8) -> Option<Vec<u8>> {
    let mut ret_vec: Vec<u8> = vec![];
    let direction = match arrow {
        Symbol::BronzeArrow(d) => Some(d),
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

pub fn is_adjecent(index: u8) -> Vec<usize> {
    let ind = index as usize;
    match ind {
        0 => {
            let vec: Vec<usize> = vec![1, 5, 6];
            return vec;
        }
        1 | 2 | 3 => {
            let vec: Vec<usize> = vec![ind - 1, ind + 1, ind + 4, ind + 5, ind + 6];
            return vec;
        }
        4 => {
            let vec: Vec<usize> = vec![3, 8, 9];
            return vec;
        }
        5 => {
            let vec: Vec<usize> = vec![0, 1, 6, 10, 11];
            return vec;
        }
        6 | 7 | 8 | 11 | 12 | 13 => {
            let vec: Vec<usize> = vec![
                ind - 6,
                ind - 5,
                ind - 4,
                ind - 1,
                ind + 1,
                ind + 4,
                ind + 5,
                ind + 6,
            ];
            return vec;
        }
        9 => {
            let vec: Vec<usize> = vec![3, 4, 8, 13, 14];
            return vec;
        }
        10 => {
            let vec: Vec<usize> = vec![5, 6, 11, 15, 16];
            return vec;
        }
        14 => {
            let vec: Vec<usize> = vec![8, 9, 13, 18, 19];
            return vec;
        }
        15 => {
            let vec: Vec<usize> = vec![10, 11, 16];
            return vec;
        }
        16 | 17 | 18 => {
            let vec: Vec<usize> = vec![ind - 6, ind - 5, ind - 4, ind - 1, ind + 1];
            return vec;
        }
        19 => {
            let vec: Vec<usize> = vec![13, 14, 18];
            return vec;
        }
        _ => {
            panic!("Index to high!");
        }
    }
}
