//! Stores the Pos struct and pos macro

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

/// A macro to create a position from x and y
/// 
/// # Example
/// ```rust
/// use battleship_bot::position::Pos;
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
