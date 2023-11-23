use crate::battleship::{boat::Boat, shot::Shot};

use super::position::Pos;

pub const NUM_ROWS: usize = 10;
pub const NUM_COLS: usize = 10;

pub const OFFSETS: [(i32, i32); 4] = [
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1)
];

pub type ShootFn = fn(Pos, ShotMap) -> Pos;
pub type PlaceFn = fn() -> BoatMap;

/// This is supposed to be returned by any `place` function.
/// 
/// If you want to implement your own [`place`](crate::place) function, you'd have to return this or at least the type it represents.
pub type BoatMap = [[Boat; NUM_ROWS]; NUM_COLS];

/// This is supposed to be a parameter to any `shoot` function.
/// 
/// If you want to implement your own [`shoot`](crate::shoot) function, you'd have to input this or at least the type it represents.
pub type ShotMap = [[Option<Shot>; NUM_ROWS]; NUM_COLS];
