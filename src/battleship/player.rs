use super::{constants::{NUM_COLS, NUM_ROWS}, game::Shot};

pub trait PlayerTrait {
    fn place_boats() -> [[usize; NUM_ROWS]; NUM_COLS];
    fn shoot(shots: [[Option<Shot>; NUM_ROWS]; NUM_COLS]) -> (usize, usize);
}