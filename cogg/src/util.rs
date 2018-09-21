use colored::*;
use failure::Error;
use log::info;
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::sync::Arc;

pub type ArcHashMap<K, V> = Arc<HashMap<K, V>>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    pub(crate) server: ServerConfig,
    pub(crate) files: FilesConfig,
    pub(crate) protector: ProtectorConfig,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ServerConfig {
    pub ip: String,
    pub port: String,
}

#[derive(Debug, Deserialize)]
pub(crate) struct FilesConfig {
    pub paths: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ProtectorConfig {
    pub allow_cloud: bool,
    pub cheats: Vec<String>,
}

impl fmt::Display for ServerConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.ip, self.port)
    }
}

pub(crate) fn setup_config(path: &Path) -> Result<Config> {
    info!("{}", "Reading Config File".blue());
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let config: Config = toml::from_str(&contents)?;
    info!("{}", "Config OK!".blue());
    Ok(config)
}

#[allow(dead_code)]
pub fn calculate_hashes(paths: Vec<String>) -> Result<ArcHashMap<String, String>> {
    let mut hashes = Arc::new(HashMap::new());
    for path in paths {
        let hashes = Arc::make_mut(&mut hashes);
        let hash = fshash::get_hash_from(&path)?;
        hashes.insert(path, hash);
    }
    Ok(hashes)
}
