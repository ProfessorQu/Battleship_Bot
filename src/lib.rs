pub (crate) mod battleship;
pub (crate) mod player;

pub use battleship::game::Battleship;
pub use battleship::game::Recording;
pub use player::players::{place, shoot};
