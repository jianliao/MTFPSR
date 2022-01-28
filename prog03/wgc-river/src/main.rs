use puzzle::Puzzle;
use std::fmt::{Display, Formatter};

/// Solves the
/// https://en.wikipedia.org/wiki/Wolf,_goat_and_cabbage_problem[Wolf, Goat, and
/// Cabbage River Puzzle].

fn main() {
    match puzzle::solve(WGCRiverPuzzle::START) {
        None => println!("no solution"),
        Some((mvs, _)) => {
            println!("solution:");
            for m in mvs {
                println!("{}", m);
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Bank {
    East,
    West,
}
impl Bank {
    fn other(&self) -> Bank {
        match self {
            Bank::East => Bank::West,
            Bank::West => Bank::East,
        }
    }
}
impl Display for Bank {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        f.write_str(match self {
            Bank::East => "east",
            Bank::West => "west",
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Object {
    Wolf,
    Goat,
    Cabbage,
}
impl Display for Object {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        f.write_str(match self {
            Object::Wolf => "wolf",
            Object::Goat => "goat",
            Object::Cabbage => "cabbage",
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Move(Bank, Option<Object>);
impl Display for Move {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "cross to {} bank with {}",
            self.0,
            match self.1 {
                None => String::from("nothing"),
                Some(obj) => format!("{}", obj),
            }
        )
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct WGCRiverPuzzle {
    farmer: Bank,
    objects: [Bank; 3],
}

impl WGCRiverPuzzle {
    const START: Self = WGCRiverPuzzle {
        farmer: Bank::East,
        objects: [Bank::East, Bank::East, Bank::East],
    };
    const GOAL: Self = WGCRiverPuzzle {
        farmer: Bank::West,
        objects: [Bank::West, Bank::West, Bank::West],
    };
}

impl Puzzle for WGCRiverPuzzle {
    type Move = Move;

    fn is_goal(&self) -> bool {
        self == &WGCRiverPuzzle::GOAL
    }

    fn next(&self) -> Vec<(Move, Self)> {
        fn ok(objects: &[Bank; 3], bank: Bank) -> bool {
            let cg_ok = !(objects[Object::Cabbage as usize] == bank
                && objects[Object::Goat as usize] == bank);
            let gw_ok =
                !(objects[Object::Goat as usize] == bank && objects[Object::Wolf as usize] == bank);
            cg_ok && gw_ok
        }
        let old_farmer_bank = self.farmer;
        let new_farmer_bank = old_farmer_bank.other();
        let mut res = vec![];
        if ok(&self.objects, old_farmer_bank) {
            res.push((
                Move(new_farmer_bank, None),
                WGCRiverPuzzle {
                    farmer: new_farmer_bank,
                    objects: self.objects,
                },
            ))
        }
        for obj in [Object::Wolf, Object::Goat, Object::Cabbage] {
            if self.objects[obj as usize] == old_farmer_bank {
                let mut new_objects = self.objects;
                new_objects[obj as usize] = new_farmer_bank;
                if ok(&new_objects, old_farmer_bank) {
                    res.push((
                        Move(new_farmer_bank, Some(obj)),
                        WGCRiverPuzzle {
                            farmer: new_farmer_bank,
                            objects: new_objects,
                        },
                    ))
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::{Bank, Move, Object, WGCRiverPuzzle};

    #[test]
    fn check_wgc_river_soln() {
        let moves1 = vec![
            Move(Bank::West, Some(Object::Goat)),
            Move(Bank::East, None),
            Move(Bank::West, Some(Object::Wolf)),
            Move(Bank::East, Some(Object::Goat)),
            Move(Bank::West, Some(Object::Cabbage)),
            Move(Bank::East, None),
            Move(Bank::West, Some(Object::Goat)),
        ];
        let moves2 = vec![
            Move(Bank::West, Some(Object::Goat)),
            Move(Bank::East, None),
            Move(Bank::West, Some(Object::Cabbage)),
            Move(Bank::East, Some(Object::Goat)),
            Move(Bank::West, Some(Object::Wolf)),
            Move(Bank::East, None),
            Move(Bank::West, Some(Object::Goat)),
        ];
        assert!([
            Some((moves1, WGCRiverPuzzle::GOAL)),
            Some((moves2, WGCRiverPuzzle::GOAL))
        ]
        .contains(&puzzle::solve(WGCRiverPuzzle::START)),)
    }
}
