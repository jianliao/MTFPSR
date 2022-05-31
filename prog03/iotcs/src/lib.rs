use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use puzzle::Puzzle;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Object {
    Corner,
    Barn,    // 0 cows can be carried over
    CropRow, // 1 cows can be carried over
    HayBale, // 3 cows can be carried over
    Fences,  // 2 cows can be carried over
    WallPlacehoder,
    Ufo,
    Silo,
    AzureCow,
    OrangeCow,
    PurpleCow,
    YellowCow,
    RedBull,
}

/// The (private) `pos` module ensures that the `Pos` type can only be created
/// via the `new` associated function and accessed via the `x`, `y`, and `xy`
/// methods.
mod pos {
    /// The `Pos` type represents a position on the gameboard.  The North-West
    /// corner of the gameboard is `Pos {x: 0, y: 0}`, the North-East corner of the
    /// gameboard `Pos {x: 4, y: 0}`, the South-West corner of the gameboard is
    /// `Pos {x: 0, y: 4}`, and the South-East corner of the gameboard is
    /// `Pos {x: 4, y: 4}`.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Pos {
        x: usize,
        y: usize,
    }
    impl Pos {
        pub(super) fn new(x: usize, y: usize) -> Self {
            if x >= 7 {
                panic!("Pos::new x (is {}) should be less than 7", x)
            }
            if y >= 7 {
                panic!("Pos::new y (is {}) should be less than 7", y)
            }
            Pos { x, y }
        }
        pub(super) fn xy(&self) -> (usize, usize) {
            (self.x, self.y)
        }
    }
}
use self::pos::Pos;

impl Pos {
    fn is_corner(&self) -> bool {
        let (x, y) = self.xy();
        (x == 1 && y == 1)
            || (x == 1 && y == 3)
            || (x == 1 && y == 5)
            || (x == 3 && y == 1)
            || (x == 3 && y == 3)
            || (x == 3 && y == 5)
            || (x == 5 && y == 1)
            || (x == 5 && y == 3)
            || (x == 5 && y == 5)
    }

    fn is_wall(&self) -> bool {
        let (x, y) = self.xy();
        y == 1
            || y == 3
            || y == 5
            || ((x == 1 || x == 3 || x == 5) && (y == 0 || y == 2 || y == 4 || y == 6))
    }

    fn can_put_none_wall_object(&self) -> bool {
        !self.is_corner() && !self.is_wall()
    }

    fn can_put_wall(&self) -> bool {
        !self.is_corner() && self.is_wall()
    }

    fn is_on_side(&self) -> bool {
        let (x, y) = self.xy();
        x == 0 || x == 6 || y == 0 || y == 6
    }

    /// Return `Some(pos)` if `pos` is the position on the gameboard that is one
    /// step from `self` in the direction `dir`.  Returns `None` if there is no
    /// position on the gameboard that is one step from `self` in the direction
    /// `dir` (i.e., would move off the edge of the gameboard).
    fn step(&self, dir: Direction) -> Option<Self> {
        let (x, y) = self.xy();
        let (x, y) = match dir {
            Direction::North => {
                if x == 0 {
                    return None;
                }
                (x - 1, y)
            }
            Direction::South => {
                if x == 6 {
                    return None;
                }
                (x + 1, y)
            }
            Direction::West => {
                if y == 0 {
                    return None;
                }
                (x, y - 1)
            }
            Direction::East => {
                if y == 6 {
                    return None;
                }
                (x, y + 1)
            }
        };
        Some(Pos::new(x, y))
    }

    /// An iterator over all positions of the gameboard.
    pub fn values() -> impl Iterator<Item = Self> {
        (0..6).flat_map(|y| (0..6).map(move |x| Pos::new(x, y)))
    }
}
impl Display for Pos {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let (x, y) = self.xy();
        write!(f, "({},{})", x, y)
    }
}

/// The `Farm` type represents the initial and fixed elements of an
/// https://www.thinkfun.com/products/invasion-of-the-cow-snatchers/[Invasion of
/// the Cow Snatchers] puzzle: the initial positions of the UFO and the cattle
/// and the fixed positions of the walls and the silo.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Farm {
    pub map: [[Option<Object>; 7]; 7],
    pub ufo_pos: Option<Pos>,
    pub cattle_number: i32,
}

impl Farm {
    pub fn new() -> Self {
        Farm {
            map: [[None; 7]; 7],
            ufo_pos: None,
            cattle_number: 0,
        }
    }
}

#[derive(Debug)]
pub struct FarmParseError;
impl FromStr for Farm {
    type Err = FarmParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Your code here
        let mut farm = Farm::new();
        let mut cs = s.chars();
        let mut has_ufo = false;
        let mut has_silo = false;
        let mut has_azure = false;
        let mut has_yellow = false;
        let mut has_purple = false;
        let mut has_orange = false;
        let mut has_red = false;
        for x in 0..7 {
            for y in 0..7 {
                let pos = Pos::new(x, y);
                match cs.next() {
                    None => return Err(FarmParseError),
                    Some(c) => match c {
                        ' ' => {
                            if !pos.can_put_none_wall_object() {
                                return Err(FarmParseError);
                            }
                        }
                        '|' => {
                            if pos.can_put_wall() {
                                farm.map[x][y] = Some(Object::WallPlacehoder)
                            } else {
                                return Err(FarmParseError);
                            }
                        }
                        '-' => {
                            if pos.can_put_wall() {
                                farm.map[x][y] = Some(Object::WallPlacehoder)
                            } else {
                                return Err(FarmParseError);
                            }
                        }
                        '+' => {
                            if pos.is_corner() {
                                farm.map[x][y] = Some(Object::Corner)
                            } else {
                                return Err(FarmParseError);
                            }
                        }
                        'U' => {
                            if pos.can_put_none_wall_object() && !has_ufo {
                                has_ufo = true;
                                farm.map[x][y] = Some(Object::Ufo);
                                farm.ufo_pos = Some(Pos::new(x, y));
                            } else {
                                return Err(FarmParseError);
                            }
                        }
                        'A' => {
                            if pos.can_put_none_wall_object() && !has_azure {
                                has_azure = true;
                                farm.map[x][y] = Some(Object::AzureCow);
                                farm.cattle_number += 1;
                            } else {
                                return Err(FarmParseError);
                            }
                        }
                        'Y' => {
                            if pos.can_put_none_wall_object() && !has_yellow {
                                has_yellow = true;
                                farm.map[x][y] = Some(Object::YellowCow);
                                farm.cattle_number += 1;
                            } else {
                                return Err(FarmParseError);
                            }
                        }
                        'P' => {
                            if pos.can_put_none_wall_object() && !has_purple {
                                has_purple = true;
                                farm.map[x][y] = Some(Object::PurpleCow);
                                farm.cattle_number += 1;
                            } else {
                                return Err(FarmParseError);
                            }
                        }
                        'O' => {
                            if pos.can_put_none_wall_object() && !has_orange {
                                has_orange = true;
                                farm.map[x][y] = Some(Object::OrangeCow);
                                farm.cattle_number += 1;
                            } else {
                                return Err(FarmParseError);
                            }
                        }
                        'R' => {
                            if pos.can_put_none_wall_object() && !has_red {
                                has_red = true;
                                farm.map[x][y] = Some(Object::RedBull);
                                farm.cattle_number += 1;
                            } else {
                                return Err(FarmParseError);
                            }
                        }
                        'B' => {
                            if pos.can_put_wall() {
                                farm.map[x][y] = Some(Object::Barn);
                            } else {
                                return Err(FarmParseError);
                            }
                        }
                        'C' => {
                            if pos.can_put_wall() {
                                farm.map[x][y] = Some(Object::CropRow);
                            } else {
                                return Err(FarmParseError);
                            }
                        }
                        'H' => {
                            if pos.can_put_wall() {
                                farm.map[x][y] = Some(Object::HayBale);
                            } else {
                                return Err(FarmParseError);
                            }
                        }
                        'F' => {
                            if pos.can_put_wall() {
                                farm.map[x][y] = Some(Object::Fences);
                            } else {
                                return Err(FarmParseError);
                            }
                        }
                        'S' => {
                            if pos.can_put_none_wall_object() && !has_silo {
                                has_silo = true;
                                farm.map[x][y] = Some(Object::Silo);
                            } else {
                                return Err(FarmParseError);
                            }
                        }
                        _ => return Err(FarmParseError),
                    },
                }
            }
            match cs.next() {
                Some('\n') => (),
                _ => return Err(FarmParseError),
            }
        }
        match cs.next() {
            Some(_) => return Err(FarmParseError),
            None => {}
        }
        match (has_ufo, has_red) {
            (true, true) => {}
            (_, _) => return Err(FarmParseError),
        }
        Ok(farm)
    }
}

/// The `Direction` type represents the cardinal directions, in which the UFO
/// may be moved.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Direction {
    North,
    South,
    West,
    East,
}
impl Direction {
    /// An iterator over all directions.
    fn values() -> impl Iterator<Item = Self> {
        [
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ]
        .into_iter()
    }
}
impl Display for Direction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(match self {
            Direction::North => "↑",
            Direction::South => "↓",
            Direction::West => "←",
            Direction::East => "→",
        })
    }
}

/// The `IotCS` type represents an
/// https://www.thinkfun.com/products/invasion-of-the-cow-snatchers/[Invasion of
/// the Cow Snatchers] puzzle state: a _reference_ to a `Farm` value (providing
/// information about the initial positions of the cattle and the fixed
/// positions of the walls and the silo) and the current status of the UFO
/// (position and collection of beamed-up cattle).  Because the `IotCS` type
/// implements `Clone`, it is important that the `IotCS` type contain a
/// _reference_ to a `Farm` value rather than an owned `Farm` value; the former
/// is simply a pointer that implments `Copy` and may therefore be efficiently
/// cloned, while the latter would require a deep copy to be cloned.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IotCS<'a> {
    farm: &'a Farm,
    current_ufo_pos: Pos,
    beamed_up_cattles: String,
    remain_cattle_num: i32,
}
impl<'a> IotCS<'a> {
    pub fn new(farm: &'a Farm) -> Self {
        IotCS {
            farm,
            current_ufo_pos: farm.ufo_pos.expect("should have a ufo"),
            beamed_up_cattles: String::from("U:"),
            remain_cattle_num: farm.cattle_number,
        }
    }
    pub fn ufo_with_cattle_to_string(&self) -> String {
        // Your code here
        self.beamed_up_cattles.clone()
    }
}

impl<'a> Puzzle for IotCS<'a> {
    type Move = Direction;

    fn is_goal(&self) -> bool {
        // Your code here
        self.current_ufo_pos.is_on_side()
            && self.remain_cattle_num == 0
            && self
                .beamed_up_cattles
                .chars()
                .last()
                .expect("Should at least has one character")
                == 'R'
    }

    fn next(&self) -> Vec<(Self::Move, Self)> {
        // Your code here
        let mut res: Vec<(Direction, IotCS)> = Vec::new();
        for dir in Direction::values() {
            if let Some(pos) = self.current_ufo_pos.step(dir) {
                let (x, y) = pos.xy();
                let mut try_move_ufo = |limit: usize| {
                    if self.beamed_up_cattles.len() < limit {
                        if let Some(new_pos) = pos.step(dir) {
                            let (new_x, new_y) = new_pos.xy();
                            if let Some(new_obj) = self.farm.map[new_x][new_y] {
                                let has_beamed_up =
                                    |name: char| -> bool { self.beamed_up_cattles.contains(name) };
                                match new_obj {
                                    Object::AzureCow => {
                                        let mut beamed_up_cattles = self.beamed_up_cattles.clone();
                                        let mut remain_cattle_num = self.remain_cattle_num;
                                        if !has_beamed_up('A') {
                                            beamed_up_cattles.push('A');
                                            remain_cattle_num = self.remain_cattle_num - 1;
                                        }
                                        let new_state = IotCS {
                                            farm: self.farm,
                                            current_ufo_pos: new_pos,
                                            beamed_up_cattles,
                                            remain_cattle_num,
                                        };
                                        res.push((dir, new_state));
                                    }
                                    Object::OrangeCow => {
                                        let mut beamed_up_cattles = self.beamed_up_cattles.clone();
                                        let mut remain_cattle_num = self.remain_cattle_num;
                                        if !has_beamed_up('O') {
                                            beamed_up_cattles.push('O');
                                            remain_cattle_num = self.remain_cattle_num - 1;
                                        }
                                        let new_state = IotCS {
                                            farm: self.farm,
                                            current_ufo_pos: new_pos,
                                            beamed_up_cattles,
                                            remain_cattle_num,
                                        };
                                        res.push((dir, new_state));
                                    }
                                    Object::PurpleCow => {
                                        let mut beamed_up_cattles = self.beamed_up_cattles.clone();
                                        let mut remain_cattle_num = self.remain_cattle_num;
                                        if !has_beamed_up('P') {
                                            beamed_up_cattles.push('P');
                                            remain_cattle_num = self.remain_cattle_num - 1;
                                        }
                                        let new_state = IotCS {
                                            farm: self.farm,
                                            current_ufo_pos: new_pos,
                                            beamed_up_cattles,
                                            remain_cattle_num,
                                        };
                                        res.push((dir, new_state));
                                    }
                                    Object::YellowCow => {
                                        let mut beamed_up_cattles = self.beamed_up_cattles.clone();
                                        let mut remain_cattle_num = self.remain_cattle_num;
                                        if !has_beamed_up('Y') {
                                            beamed_up_cattles.push('Y');
                                            remain_cattle_num = self.remain_cattle_num - 1;
                                        }
                                        let new_state = IotCS {
                                            farm: self.farm,
                                            current_ufo_pos: new_pos,
                                            beamed_up_cattles,
                                            remain_cattle_num,
                                        };
                                        res.push((dir, new_state));
                                    }
                                    Object::RedBull
                                        if self.remain_cattle_num == 1
                                            || self.remain_cattle_num == 0 =>
                                    {
                                        let mut beamed_up_cattles = self.beamed_up_cattles.clone();
                                        let mut remain_cattle_num = self.remain_cattle_num;
                                        if !has_beamed_up('R') {
                                            beamed_up_cattles.push('R');
                                            remain_cattle_num = self.remain_cattle_num - 1;
                                        }
                                        let new_state = IotCS {
                                            farm: self.farm,
                                            current_ufo_pos: new_pos,
                                            beamed_up_cattles,
                                            remain_cattle_num,
                                        };
                                        res.push((dir, new_state));
                                    }
                                    Object::Ufo => {
                                        let new_state = IotCS {
                                            farm: self.farm,
                                            current_ufo_pos: new_pos,
                                            beamed_up_cattles: self.beamed_up_cattles.clone(),
                                            remain_cattle_num: self.remain_cattle_num,
                                        };
                                        res.push((dir, new_state));
                                    }
                                    _ => {}
                                }
                            } else {
                                res.push((
                                    dir,
                                    IotCS {
                                        farm: self.farm,
                                        current_ufo_pos: new_pos,
                                        beamed_up_cattles: self.beamed_up_cattles.clone(),
                                        remain_cattle_num: self.remain_cattle_num,
                                    },
                                ));
                            }
                        }
                    }
                };
                match self.farm.map[x][y] {
                    Some(obj) => match obj {
                        Object::Barn => try_move_ufo(3), // As "U:".len() is 2, so 3 - 2 = 1
                        Object::CropRow => try_move_ufo(4), // 4 - 2 = 2
                        Object::Fences => try_move_ufo(5), // 5 - 2 = 3
                        Object::HayBale => try_move_ufo(6), // 6 - 2 = 4
                        Object::WallPlacehoder => try_move_ufo(std::usize::MAX), // No limit as there is no wall
                        _ => {}
                    },
                    None => {}
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests;
