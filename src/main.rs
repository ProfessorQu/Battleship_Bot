#![allow(dead_code)]

mod battleship;

use std::time::SystemTime;

use battleship::Battleship;
use battleship::players;

fn main() {
    let mut battleship = Battleship::new(
        players::random::place,
        players::random::place,

        players::random::shoot,
        players::random::shoot_and_focus,
    );

    let now = SystemTime::now();

    let (p1_won, p2_won) = battleship.play_games(10_000);
    println!("p1: {}; p2: {}", p1_won, p2_won);

    println!("Elapsed: {} ms", now.elapsed().expect("No time passed").as_millis());

    // battleship.play_and_show_game();
}