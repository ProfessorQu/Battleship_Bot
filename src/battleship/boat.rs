use std::fmt::Debug;

const LENGTHS: [usize; 5] = [2, 3, 3, 4, 5];

/// Stores the type of Boat that is on a cell
/// 
/// Can either be `Empty` or one of the 5 boats in standard Battleship.
#[derive(Clone, Copy, PartialEq)]
pub enum Boat {
    Empty,
    Destroyer,
    Submarine,
    Cruiser,
    Battleship,
    Carrier
}

impl Debug for Boat {
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

    pub fn length(&self) -> usize {
        LENGTHS[*self as usize - 1]
    }
}

pub const BOATS: [Boat; 5] = [
    Boat::Destroyer,
    Boat::Submarine,
    Boat::Cruiser,
    Boat::Battleship,
    Boat::Carrier,
];