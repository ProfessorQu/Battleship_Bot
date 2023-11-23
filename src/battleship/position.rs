//! Stores the Pos struct and pos macro

use rand::distributions::{Distribution, Standard};

use crate::battleship::constants::{NUM_COLS, NUM_ROWS};

/// Saves an x and y for a position on a board
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pos {
    pub x: usize,
    pub y: usize
}

impl Pos {
    #[doc(hidden)]
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x, y
        }
    }
}

impl Distribution<Pos> for Standard {
    fn sample<R: rand::prelude::Rng + ?Sized>(&self, rng: &mut R) -> Pos {
        Pos::new(
            rng.gen_range(0..NUM_COLS),
            rng.gen_range(0..NUM_ROWS)
        )
    }
}

/// A macro to create a position from x and y
/// 
/// # Example
/// ```rust
/// use battleship_bot::Pos;
/// use battleship_bot::pos;
/// 
/// let pos = pos!(0, 4);
/// 
/// assert_eq!(pos.x, 0);
/// assert_eq!(pos.y, 4);
/// ```
#[macro_export]
macro_rules! pos {
    ($x:expr, $y:expr) => {
        Pos::new($x, $y)
    };
}
