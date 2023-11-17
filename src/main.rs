#![allow(dead_code)]

mod battleship;

use std::time::SystemTime;

use battleship::Battleship;
use battleship::players;

fn main() {
    let mut battleship = Battleship::new(
        players::random::place,
        players::random::place,

        players::random::shoot_and_random_focus,
        players::random::shoot_and_focus,
    );

    let now = SystemTime::now();

    let num_games = 10_000;
    let (p1_wins, p2_wins) = battleship.play_games(num_games);
    println!("Elapsed: {} ms", now.elapsed().expect("No time passed").as_millis());
    let (p1_winrate, p2_winrate) = (p1_wins as f32 / num_games as f32, p2_wins as f32 / num_games as f32);
    println!("p1: {}; p2: {}\np1 winrate: {}, p2 winrate: {}", p1_wins, p2_wins, p1_winrate, p2_winrate);

    // battleship.play_and_show_game();

}