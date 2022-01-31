use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use serde::{Deserialize, Serialize};

use puzzle::Puzzle;

/// The `Farm` type represents the initial and fixed elements of an
/// https://www.thinkfun.com/products/invasion-of-the-cow-snatchers/[Invasion of
/// the Cow Snatchers] puzzle: the initial positions of the UFO and the cattle
/// and the fixed positions of the walls and the silo.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Farm {
    // Your code here
}

#[derive(Debug)]
pub struct FarmParseError;
impl FromStr for Farm {
    type Err = FarmParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Your code here
        unimplemented!()
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
    // Your code here
}
impl<'a> IotCS<'a> {
    pub fn new(farm: &'a Farm) -> Self {
        // Your code here
        unimplemented!()
    }
    pub fn ufo_with_cattle_to_string(&self) -> String {
        // Your code here
        unimplemented!()
    }
}

impl<'a> Puzzle for IotCS<'a> {
    type Move = Direction;

    fn is_goal(&self) -> bool {
        // Your code here
        unimplemented!()
    }

    fn next(&self) -> Vec<(Self::Move, Self)> {
        // Your code here
        unimplemented!()
    }
}

#[cfg(test)]
mod tests;
