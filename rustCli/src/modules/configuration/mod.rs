use std::fs;
use serde_json;
use serde_json::{Result, Value};

pub fn read_config(config_file: String) -> serde_json::Value {
    //println!("DEBUG:  Reading config file: {}", config_file);

    let file = fs::File::open(config_file)
        .expect("file should open read only");

    let settings: serde_json::Value = serde_json::from_reader(file)
        .expect("file should be proper JSON");

    //println!("{}", settings["database"]["host"]);


    settings
}
