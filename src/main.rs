use game::Game;

mod config;
mod game;
mod lichess;

fn main() {
    let c = config::read_config();
    let user_names_a = c.user_names.to_vec();
    let mut games: Vec<Game> = vec![];
    for (index_player_a, player_a) in user_names_a.into_iter().enumerate() {
        let user_names_b = c.user_names.to_vec();
        let vs_players: Vec<String> = user_names_b
            .into_iter()
            .enumerate()
            .filter(|&(i, _)| i != index_player_a)
            .map(|(_, v)| v.clone())
            .collect();
        for player_b in vs_players {
            let waiting_period = std::time::Duration::from_secs(2);
            std::thread::sleep(waiting_period);
            let mut new_games = lichess::get_games(&player_a, &player_b);
            games.append(&mut new_games);
        }
    }

    for game in games {
        let game_str = serde_json::to_string(&game).unwrap();
        println!("{}", game_str);
    }
}
