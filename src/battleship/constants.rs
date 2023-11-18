use super::{boat::Boat, game::Shot};

pub const NUM_ROWS: usize = 10;
pub const NUM_COLS: usize = 10;

pub const OFFSETS: [(i32, i32); 4] = [
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1)
];

pub type BoatMap = [[Boat; NUM_ROWS]; NUM_COLS];
pub type ShotMap = [[Option<Shot>; NUM_ROWS]; NUM_COLS];
