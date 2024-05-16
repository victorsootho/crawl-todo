use chrono::{Datelike, Duration, NaiveDateTime, NaiveTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufReader, BufWriter};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    minutes_spent: u64,
}

#[derive(Serialize, Deserialize, Debug)]
struct TodaysTasks {
    date: String,
    todays_tasks: HashMap<String, Task>,
}

#[derive(Serialize, Deserialize, Debug)]
struct UserSettings {
    end_time: Option<String>,
    today: TodaysTasks,
}

impl UserSettings {
    fn new() -> Self {
        UserSettings {
            end_time: None,
            today: TodaysTasks {
                date: Utc::now().format("%d/%m/%Y").to_string(),
                todays_tasks: HashMap::new(),
            },
        }
    }
}

fn main() {
    let mut user_settings = load_user_settings();

    let current_time = Utc::now() + Duration::hours(2);

    println!(
        "Welcome to a new day. Time is {}",
        current_time.format("%H:%M:%S")
    );

    let current_weekday = current_time.date_naive().weekday();
    let formatted_date = current_time.format("%d/%m/%Y").to_string();

    // Check if the date has changed
    if user_settings.today.date != formatted_date {
        user_settings.today = TodaysTasks {
            date: formatted_date.clone(),
            todays_tasks: HashMap::new(),
        };
    }

    println!("Schedule for {}, {}", formatted_date, current_weekday);

    // The hour at which the day closes
    let end_of_day = match &user_settings.end_time {
        Some(end_time) => NaiveDateTime::parse_from_str(
            &format!("{} {}", current_time.format("%Y-%m-%d"), end_time),
            "%Y-%m-%d %H:%M:%S",
        )
        .expect("Invalid date format")
        .and_utc(),
        None => {
            println!("Enter the desired end time for your day (e.g., 23:00:00):");
            let mut end_time = String::new();
            io::stdin()
                .read_line(&mut end_time)
                .expect("Failed to read line");
            let end_time = end_time.trim().to_string();
            user_settings.end_time = Some(end_time.clone());
            save_user_settings(&user_settings);
            NaiveDateTime::parse_from_str(
                &format!("{} {}", current_time.format("%Y-%m-%d"), end_time),
                "%Y-%m-%d %H:%M:%S",
            )
            .expect("Invalid date format")
            .and_utc()
        }
    };

    let remaining_hours = end_of_day - current_time;
    let hours_left = remaining_hours.num_hours();
    let minutes_left = remaining_hours.num_minutes() % 60;

    println!(
        "You have {} hours and {} minutes left.",
        hours_left, minutes_left
    );

    println!("Here Are Today's and Tomorrow's Deadlines");

    let mut total_productivity_minutes: u64 = 0;

    loop {
        println!("Enter task code (C for Coding, R for Reading, A for Audio, W for Writing, L for Learning, or X to exit):");
        let mut task_code = String::new();
        io::stdin()
            .read_line(&mut task_code)
            .expect("Failed to read line");
        let task_code = task_code.trim().to_lowercase();

        if task_code == "x" {
            break;
        }

        let task_name = match task_code.as_str() {
            "c" => "Coding",
            "r" => "Reading",
            "a" => "Audio",
            "w" => "Writing",
            "l" => "Learning",
            _ => {
                println!("Invalid task code. Please try again.");
                continue;
            }
        };

        let start_time = prompt_for_time(&format!(
            "Enter start time for {} (HH:MM) or 'now' for the current time:",
            task_name
        ));

        let end_time = prompt_for_time(&format!(
            "Enter end time for {} (HH:MM) or 'now' for the current time:",
            task_name
        ));

        let duration = end_time.signed_duration_since(start_time);
        let duration_minutes = duration.num_minutes() as u64;

        let task_entry = user_settings
            .today
            .todays_tasks
            .entry(task_name.to_string())
            .or_insert(Task { minutes_spent: 0 });
        task_entry.minutes_spent += duration_minutes;

        total_productivity_minutes += duration_minutes;

        let hours_productive = total_productivity_minutes / 60;
        let minutes_productive = total_productivity_minutes % 60;

        println!(
            "You have been productive for {} hours and {} minutes",
            hours_productive, minutes_productive
        );

        for (task, task_data) in &user_settings.today.todays_tasks {
            let task_hours = task_data.minutes_spent / 60;
            let task_minutes = task_data.minutes_spent % 60;
            println!(
                "{}: {} hours and {} minutes",
                task, task_hours, task_minutes
            );
        }

        save_user_settings(&user_settings);
    }
}

fn prompt_for_time(prompt: &str) -> NaiveTime {
    loop {
        println!("{}", prompt);
        let mut time_input = String::new();
        io::stdin()
            .read_line(&mut time_input)
            .expect("Failed to read line");
        let time_input = time_input.trim();
        if time_input.eq_ignore_ascii_case("now") {
            return (Utc::now() + Duration::hours(2)).time();
        } else {
            match NaiveTime::parse_from_str(time_input, "%H:%M") {
                Ok(time) => return time,
                Err(_) => println!("Invalid time format. Please try again."),
            }
        }
    }
}

fn load_user_settings() -> UserSettings {
    let file = File::open("user_settings.json")
        .unwrap_or_else(|_| File::create("user_settings.json").unwrap());
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).unwrap_or_else(|_| UserSettings::new())
}

fn save_user_settings(user_settings: &UserSettings) {
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("user_settings.json")
        .unwrap();
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, user_settings).unwrap();
}
