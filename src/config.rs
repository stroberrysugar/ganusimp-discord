use serde::{Deserialize, Serialize};
use serenity::{
    model::id::{GuildId, UserId},
    prelude::TypeMapKey,
};
use std::{fs, io::ErrorKind};

pub fn read_config() -> Config {
    match fs::read_to_string("config.toml") {
        Err(e) if e.kind() == ErrorKind::NotFound => Config::generate(),
        Err(e) => panic!("Failed to read config.toml: {:?}", e),
        Ok(x) => toml::from_str(&x).expect("Failed to parse config.toml"),
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Config {
    pub application_id: u64,
    pub guild_id: GuildId,
    pub token: String,
    #[serde(rename = "member")]
    pub members: Vec<Member>,
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Member {
    pub discord: UserId,
    pub username: String,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum MemberIdentifier {
    Cracked { username: String },
    Premium { uuid: String },
}

impl TypeMapKey for Config {
    type Value = Config;
}

impl Default for Config {
    fn default() -> Config {
        Config {
            application_id: 849145431744839690,
            guild_id: GuildId(883354392093409280),
            token: "".to_string(),
            members: vec![Member {
                discord: UserId(0),
                username: "".to_string(),
            }],
        }
    }
}

impl Config {
    pub fn generate() -> Config {
        let config = Config::default();
        let toml = match toml::to_string_pretty(&config) {
            Ok(n) => n,
            Err(e) => panic!("{:?}", e),
        };
        fs::write("config.toml", &toml).expect("Failed to write to config.toml");
        config
    }
}
