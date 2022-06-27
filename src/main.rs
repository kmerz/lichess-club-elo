use std::collections::HashMap;

use elo::EloRank;
use game::Game;

mod config;
mod game;
mod lichess;

fn main() {
    let c = config::read_config();

    let user_names_c = c.user_names.to_vec();

    let mut elo_table = HashMap::new();
    for user_name in user_names_c {
        elo_table.insert(user_name, 1500.0);
    }

    let user_names_a = c.user_names.to_vec();
    let mut games: Vec<Game> = vec![];
    for (index_player_a, player_a) in user_names_a.into_iter().enumerate() {
        let user_names_b = c.user_names.to_vec();
        let vs_players: Vec<String> = user_names_b
            .into_iter()
            .enumerate()
            .filter(|&(i, _)| i > index_player_a)
            .map(|(_, v)| v.clone())
            .collect();
        for player_b in vs_players {
            let waiting_period = std::time::Duration::from_secs(1);
            std::thread::sleep(waiting_period);
            let mut new_games = lichess::get_games(&player_a, &player_b);
            games.append(&mut new_games);
        }
    }

    for game in games {
        let player_white = game.players.white.user.id;
        let player_black = game.players.black.user.id;
        let winner = game.winner.unwrap_or("draw".to_string());
        match winner.as_str() {
            "white" => new_elo_ranks(player_white, player_black, &mut elo_table),
            "black" => new_elo_ranks(player_black, player_white, &mut elo_table),
            _ => (),
        }
    }

    let mut sorted: Vec<_> = elo_table.iter().collect();
    sorted.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());

    for (i, (key, value)) in sorted.iter().enumerate() {
        println!("{}: {} - {}", i + 1, key, value);
    }
}

fn new_elo_ranks(winner: String, loser: String, table: &mut HashMap<String, f64>) {
    let elo = EloRank { k: 32 };
    let winner_elo = table.get(&winner).unwrap();
    let loser_elo = table.get(&loser).unwrap();

    let (winner_new_elo, loser_new_elo) =
        elo.calculate(*winner_elo, *loser_elo, elo::MatchWinner::PlayerA);
    table.insert(winner, winner_new_elo);
    table.insert(loser, loser_new_elo);
}
