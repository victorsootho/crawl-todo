use crate::task::UserSettings;
use chrono::Utc;
use dirs::home_dir;
use serde_json;
use std::fs::{self, File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;

fn get_config_path() -> PathBuf {
    let mut path = home_dir().expect("Unable to find home directory");
    path.push(".config");
    path.push("crawl-todo");
    path.push("user_settings.json");
    path
}

fn ensure_config_dir() {
    let mut dir = get_config_path();
    dir.pop(); // Remove the filename to get the directory
    fs::create_dir_all(dir).expect("Unable to create config directory");
}

pub fn load_user_settings() -> UserSettings {
    ensure_config_dir();
    let path = get_config_path();
    let file = File::open(&path).unwrap_or_else(|_| {
        let default_settings = UserSettings::new(Utc::now().format("%d/%m/%Y").to_string());
        save_user_settings(&default_settings);
        File::open(&path).expect("Unable to create settings file")
    });
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).expect("Unable to parse settings file")
}

pub fn save_user_settings(user_settings: &UserSettings) {
    ensure_config_dir();
    let path = get_config_path();
    println!("Saving settings to: {:?}", path); // Debug print
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(&path)
        .expect("Unable to open settings file for writing");
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, user_settings).expect("Unable to write settings");
    println!("Settings saved successfully"); // Debug print
}
