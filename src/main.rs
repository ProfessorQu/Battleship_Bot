#![allow(dead_code)]

mod battleship;
use battleship::Game;
use battleship::Player;
use battleship::PlayerType;
use battleship::Random;

fn main() {
    let game = Game::new(
        Random::place_boats(),
        Random::place_boats()
    );

    game.show_boats(PlayerType::P1);
    game.show_boats(PlayerType::P2);
}