use super::constants::{COLUMNS, ROWS};

pub trait Player {
    fn place_boats() -> [[usize; COLUMNS]; ROWS];
    fn shoot() -> (usize, usize);
}