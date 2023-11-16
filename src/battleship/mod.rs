mod game;
pub use game::Battleship;
pub use game::Player;

pub mod constants;

mod player;
pub use player::PlayerTrait;

mod players;
pub use players::random::Random;
