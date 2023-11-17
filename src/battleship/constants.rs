use std::fmt::Display;

pub const NUM_ROWS: usize = 10;
pub const NUM_COLS: usize = 10;

#[derive(Clone, Copy, PartialEq)]
pub enum Boat {
    Empty,
    Destroyer,
    Submarine,
    Cruiser,
    Battleship,
    Carrier
}

impl Display for Boat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", *self as usize)
    }
}

impl Boat {
    pub fn is_empty(&self) -> bool {
        *self == Boat::Empty
    }

    pub fn has_some(&self) -> bool {
        !self.is_empty()
    }
}

pub const BOATS: [Boat; 5] = [
    Boat::Destroyer,
    Boat::Submarine,
    Boat::Cruiser,
    Boat::Battleship,
    Boat::Carrier,
];

pub const MIN_SHOTS: usize = 2 + 3 + 3 + 4 + 5;
