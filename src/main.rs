#![allow(dead_code)]

mod battleship;

use battleship::Game;
use battleship::Player;
use battleship::PlayerType;
use battleship::Random;

fn main() {
    let mut game = Game::new(
        Random::place_boats(),
        Random::place_boats()
    );

    let mut won = None;

    let mut current = PlayerType::P1;

    while won.is_none() {
        let pos = Random::shoot(game.get_shots(current));
        // let pos = (x, y);
        game.shoot(current, pos);

        won = game.won();

        current = current.opponent();
    }

    println!("{:?}", won);

    println!("P1 BOATS ================================");
    game.show_boats(PlayerType::P1);
    game.show_shots(PlayerType::P2);

    println!("P2 BOATS ================================");
    game.show_boats(PlayerType::P2);
    game.show_shots(PlayerType::P1);

}