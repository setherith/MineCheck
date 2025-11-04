use serde::{Deserialize};
use std::{fs::File, io::{BufReader}};

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub local_server_location: String,
}

pub fn load_settings() -> Settings {
    let file = File::open("./settings.json").expect("Fail");
    let reader = BufReader::new(file);
    let settings: Settings = serde_json::from_reader(reader).expect("Fail");
    settings
}