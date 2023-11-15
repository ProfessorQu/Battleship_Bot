use super::constants::{NUM_COLS, NUM_ROWS};

pub trait Player {
    fn place_boats() -> [[usize; NUM_ROWS]; NUM_COLS];
    fn shoot() -> (usize, usize);
}