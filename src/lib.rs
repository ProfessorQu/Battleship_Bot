pub (crate) mod battleship;
pub (crate) mod player;

pub use crate::battleship::game::Battleship;
pub use crate::battleship::game::Recording;
pub use crate::battleship::position::Pos;
pub use crate::player::players::{place, shoot};

pub use crate::battleship::constants::{BoatMap, ShotMap};
pub use crate::battleship::boat::Boat;
pub use crate::battleship::shot::Shot;
