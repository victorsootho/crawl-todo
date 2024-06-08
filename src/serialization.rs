use crate::task::UserSettings;
use chrono::Utc;
use serde_json;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};

pub fn load_user_settings() -> UserSettings {
    let file = File::open("user_settings.json")
        .unwrap_or_else(|_| File::create("user_settings.json").unwrap());
    let reader = BufReader::new(file);
    serde_json::from_reader(reader)
        .unwrap_or_else(|_| UserSettings::new(Utc::now().format("%d/%m/%Y").to_string()))
}

pub fn save_user_settings(user_settings: &UserSettings) {
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("user_settings.json")
        .unwrap();
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, user_settings).unwrap();
}
