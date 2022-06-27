use crate::game::Game;
use std::process::exit;

const LICHESSURL: &str = "https://lichess.org/api/games/user/";

pub fn get_games(player_a: &str, player_b: &str) -> Vec<Game> {
    let request_url = format!("{}{}?vs={}", LICHESSURL, player_a, player_b);
    println!("request url: {}", request_url);
    let client = reqwest::blocking::Client::new();
    let res = client
        .get(request_url)
        .header("Accept", "application/x-ndjson")
        .send()
        .unwrap_or_else(|e| {
            eprintln!("Error during reqwest game: {}", e);
            exit(1);
        });
    let game_strs = res.text().unwrap_or_else(|e| {
        eprintln!("Could not unwrap text: {}", e);
        exit(1);
    });

    let games: Vec<Game> = game_strs
        .split("\n")
        .filter(|str| return str.len() > 0)
        .map(|g_str| {
            return Game::new(g_str);
        })
        .collect();

    return games;
}
