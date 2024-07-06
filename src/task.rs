use crate::serialization::save_user_settings;
use crate::user_interaction::get_time_from_user; // Import the function
use chrono::{NaiveTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap; // Import the function

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub minutes_spent: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TodaysTasks {
    pub date: String,
    pub start_time: Option<String>,
    pub todays_tasks: HashMap<String, Task>,
    pub todays_chores: HashMap<String, Task>,
    pub todays_entertainment: HashMap<String, Task>,
}

impl TodaysTasks {
    pub fn new(date: String, start_time: Option<String>) -> Self {
        TodaysTasks {
            date,
            start_time,
            todays_tasks: HashMap::new(),
            todays_chores: HashMap::new(),
            todays_entertainment: HashMap::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserSettings {
    pub end_time: Option<String>,
    pub today: TodaysTasks,
    pub past_tasks: Vec<TodaysTasks>,
}

impl UserSettings {
    pub fn new(date: String) -> Self {
        UserSettings {
            end_time: None,
            today: TodaysTasks::new(date, None),
            past_tasks: Vec::new(),
        }
    }

    pub fn get_start_time(&mut self) -> NaiveTime {
        let default_start_time = "Unknown".to_string();
        let start_time = self
            .today
            .start_time
            .as_ref()
            .unwrap_or(&default_start_time);

        if start_time == "Unknown" {
            let user_start_time =
                get_time_from_user("Enter your start time for today (HH:MM or 'now'):");
            self.today.start_time = Some(user_start_time.format("%H:%M:%S").to_string());
            save_user_settings(&self);
            user_start_time
        } else {
            NaiveTime::parse_from_str(start_time, "%H:%M:%S").expect("Invalid start time format")
        }
    }

    pub fn get_end_time(&mut self, current_time: &chrono::DateTime<Utc>) -> chrono::DateTime<Utc> {
        loop {
            match &self.end_time {
                Some(end_time) => {
                    match chrono::NaiveDateTime::parse_from_str(
                        &format!("{} {}", current_time.format("%Y-%m-%d"), end_time),
                        "%Y-%m-%d %H:%M",
                    ) {
                        Ok(datetime) => break datetime.and_utc(),
                        Err(_) => {
                            println!("Invalid end time format. Please try again.");
                            continue;
                        }
                    }
                }
                None => {
                    println!("Enter the desired end time for your day (e.g., 23:00:00):");
                    let mut end_time = String::new();
                    std::io::stdin()
                        .read_line(&mut end_time)
                        .expect("Failed to read line");
                    let end_time = end_time.trim().to_string();
                    match chrono::NaiveDateTime::parse_from_str(
                        &format!("{} {}", current_time.format("%Y-%m-%d"), end_time),
                        "%Y-%m-%d %H:%M",
                    ) {
                        Ok(datetime) => {
                            self.end_time = Some(end_time.clone());
                            save_user_settings(&self);
                            break datetime.and_utc();
                        }
                        Err(_) => {
                            println!("Invalid end time format. Please try again!!");
                            continue;
                        }
                    }
                }
            }
        }
    }
}
