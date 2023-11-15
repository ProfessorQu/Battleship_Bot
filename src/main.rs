#![allow(dead_code)]

mod battleship;

use battleship::Game;
use battleship::Player;
use battleship::PlayerType;
use battleship::Random;

fn main() {
    let mut game = Game::new(
        Random::place_boats(),
        Random::place_boats(),

        Random::shoot,
        Random::shoot
    );

    let mut won = None;

    while won.is_none() {
        game.step();
        won = game.won();
    }

    println!("{:?}", won);

    println!("P1 BOATS ================================");
    game.show_boats(PlayerType::P1);
    game.show_shots(PlayerType::P2);

    println!("P2 BOATS ================================");
    game.show_boats(PlayerType::P2);
    game.show_shots(PlayerType::P1);
}