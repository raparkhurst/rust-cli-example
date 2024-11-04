use std::fs;
use anyhow::{Context, Result};
use confy;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use serde_json;
use serde_json::{Value};
use crate::platform;


const APP_NAME: &str = "rustCli";
const FILE_STEM: &str = "config";


#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Config {
    pub repo: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            repo: "".into(),
        }
    }
}

pub fn path() -> Result<PathBuf> {
    confy::get_configuration_file_path(APP_NAME, FILE_STEM)
        .with_context(|| "unable to find the config file")
}

pub fn load() -> Result<Config> {
    confy::load(APP_NAME, FILE_STEM).with_context(|| "unable to load config")
}



pub fn save(config: Config) -> Result<()> {
    //let result = confy::get_configuration_file_path(APP_NAME, FILE_STEM);
    //println!("{:?}", result);
    confy::store(APP_NAME, FILE_STEM, config).with_context(|| "unable to save config")
}

pub fn settings() -> serde_json::Value {
    let filename = crate::app::config().repo.to_string() + "/config.json";

    let file = fs::File::open(filename)
        .expect("file should open read only");

    let settings: serde_json::Value = serde_json::from_reader(file)
        .expect("file should be proper JSON");

    settings
}
