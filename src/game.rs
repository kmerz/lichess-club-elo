use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::process::exit;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    id: String,
    rated: bool,
    variant: String,
    speed: String,
    perf: String,
    created_at: i64,
    last_move_at: i64,
    pub status: String,
    pub winner: Option<String>,
    pub players: Players,
}

#[derive(Deserialize, Serialize)]
pub struct Players {
    pub white: Color,
    pub black: Color,
}

#[derive(Deserialize, Serialize)]
pub struct Color {
    pub user: User,
}

#[derive(Deserialize, Serialize)]
pub struct User {
    pub name: String,
    pub id: String,
}

impl Game {
    pub fn new(json: &str) -> Game {
        let game: Game = match serde_json::from_str(json) {
            Ok(game) => game,
            Err(e) => {
                eprintln!("Could not read game from json: {}\n\n{}", e, json);
                exit(1);
            }
        };
        return game;
    }
}
