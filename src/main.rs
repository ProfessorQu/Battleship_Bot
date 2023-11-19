#![allow(dead_code)]

use std::time::SystemTime;

mod battleship;

use battleship::Battleship;
use battleship::players;

fn main() {
    // let mut battleship = Battleship::new(
    //     players::place::place_boats_cluster,
    //     players::place::place_boats_random,

    //     players::shoot::random_find_and_destroy,
    //     players::shoot::random_find_and_destroy,
    // );

    let now = SystemTime::now();

    let games_per_fn = 10_000;
    Battleship::save_games(games_per_fn, "data.csv");

    // let num_games = 10_000;
    // let (p1_wins, p2_wins) = battleship.play_games(num_games);
    // let (p1_winrate, p2_winrate) = (p1_wins as f32 / num_games as f32, p2_wins as f32 / num_games as f32);
    // println!("p1: {}; p2: {}\np1 winrate: {}, p2 winrate: {}", p1_wins, p2_wins, p1_winrate, p2_winrate);

    // battleship.play_and_show_game();

    println!("Elapsed: {} ms", now.elapsed().expect("No time passed").as_millis());
}