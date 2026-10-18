use std::{
    env,
    fmt::Display,
    fs::{self, File},
    io,
    process::exit,
};

use minecraft_uuid::get_username_by_uuid;
use serde_json::Value;

#[tokio::main]
async fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        help();
    }

    let mut entries = fs::read_dir(&args[1])?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    entries.sort();
    let mut players: Vec<Player> = vec![];
    for entry in entries {
        let uuid = entry.file_stem().unwrap().to_str().unwrap().to_owned();
        let name = get_username_by_uuid(&uuid).await.unwrap();

        let contents = fs::read_to_string(entry)?;

        let v: Value = serde_json::from_str(&contents)?;

        let playtime: &u64 = &v["stats"]["minecraft:custom"]["minecraft:play_time"]
            .as_u64()
            .unwrap();

        let playtime = playtime / 20;

        let p = Player::new(name, uuid, playtime);

        println!("{}", p);

        players.push(p);
    }

    let path = "playtimes.txt";
    let mut contents = String::new();
    for p in players {
        contents.push_str(&p.to_string());
        contents.push_str("\n");
    }
    fs::write(path, contents)?;

    Ok(())
}

fn format_time_from_seconds(seconds: u64) -> String {
    let minutes = &seconds / 60;
    let hours = &minutes / 60;

    let seconds = seconds - minutes * 60;
    let minutes = minutes - hours * 60;

    let o = format!("{}h {}m {}s", hours, minutes, seconds);

    o
}

fn help() {
    println!("Usage: getplaytime <server path>/world/stats/");
    exit(1);
}

struct Player {
    name: String,
    uuid: String,
    playtime: u64,
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} : {}",
            self.name,
            format_time_from_seconds(self.playtime)
        )
    }
}

impl Player {
    fn new(name: String, uuid: String, playtime: u64) -> Self {
        Self {
            name,
            uuid,
            playtime,
        }
    }
}
