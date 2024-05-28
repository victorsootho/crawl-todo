use chrono::{Datelike, Duration, NaiveDateTime, NaiveTime, Utc};
use colored::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufReader, BufWriter};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    minutes_spent: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TodaysTasks {
    date: String,
    start_time: Option<String>,
    todays_tasks: HashMap<String, Task>,
    todays_chores: HashMap<String, Task>,
    todays_entertainment: HashMap<String, Task>, // New field for entertainment
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct UserSettings {
    end_time: Option<String>,
    today: TodaysTasks,
    past_tasks: Vec<TodaysTasks>,
}

impl UserSettings {
    fn new(date: String) -> Self {
        UserSettings {
            end_time: None,
            today: TodaysTasks {
                date,
                start_time: None,
                todays_tasks: HashMap::new(),
                todays_chores: HashMap::new(),
                todays_entertainment: HashMap::new(), // Initialize the entertainment HashMap
            },
            past_tasks: Vec::new(),
        }
    }
}

fn main() {
    let mut user_settings = load_user_settings();

    let current_time = Utc::now() + Duration::hours(2);

    let formatted_date = current_time.format("%d/%m/%Y").to_string();

    // Check if the date has changed and move the today object to past_tasks if it has
    if user_settings.today.date != formatted_date {
        user_settings.past_tasks.push(user_settings.today.clone());
        user_settings.today = TodaysTasks {
            date: formatted_date.clone(),
            start_time: Some(current_time.format("%H:%M:%S").to_string()),
            todays_tasks: HashMap::new(),
            todays_chores: HashMap::new(),
            todays_entertainment: HashMap::new(), // Initialize the entertainment HashMap
        };
        save_user_settings(&user_settings);
    }

    let default_start_time = "Unknown".to_string();
    let start_time = user_settings
        .today
        .start_time
        .as_ref()
        .unwrap_or(&default_start_time);

    println!(
        "Welcome to a new day. You started today at {}. Current time is {}",
        start_time,
        current_time.format("%H:%M:%S")
    );

    let current_weekday = current_time.date_naive().weekday();

    let start_time = NaiveTime::parse_from_str(
        user_settings
            .today
            .start_time
            .as_deref()
            .unwrap_or("00:00:00"),
        "%H:%M:%S",
    )
    .expect("Invalid start time format");

    println!("Schedule for {}, {}", formatted_date, current_weekday);

    // The hour at which the day closes
    let end_of_day = loop {
        match &user_settings.end_time {
            Some(end_time) => {
                match NaiveDateTime::parse_from_str(
                    &format!("{} {}", current_time.format("%Y-%m-%d"), end_time),
                    "%Y-%m-%d %H:%M:%S",
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
                io::stdin()
                    .read_line(&mut end_time)
                    .expect("Failed to read line");
                let end_time = end_time.trim().to_string();
                match NaiveDateTime::parse_from_str(
                    &format!("{} {}", current_time.format("%Y-%m-%d"), end_time),
                    "%Y-%m-%d %H:%M:%S",
                ) {
                    Ok(datetime) => {
                        user_settings.end_time = Some(end_time.clone());
                        save_user_settings(&user_settings);
                        break datetime.and_utc();
                    }
                    Err(_) => {
                        println!("Invalid end time format. Please try again.");
                        continue;
                    }
                }
            }
        }
    };

    let end_time = end_of_day.time();

    // Calculate the duration between start_time and end_time
    let day_duration = end_time.signed_duration_since(start_time);
    let total_hours = day_duration.num_hours();
    let total_minutes = day_duration.num_minutes() % 60;

    println!(
        "{}",
        format!(
            "You have {} hours and {} minutes to seize the day",
            total_hours, total_minutes
        )
        .green()
    );

    let remaining_hours = end_of_day - current_time;
    let hours_left = remaining_hours.num_hours();
    let minutes_left = remaining_hours.num_minutes() % 60;

    println!(
        "You have {} hours and {} minutes left.",
        hours_left, minutes_left
    );

    // Calculate total productive minutes from saved tasks
    let mut total_productivity_minutes: u64 = user_settings
        .today
        .todays_tasks
        .values()
        .map(|task| task.minutes_spent)
        .sum();

    let hours_productive = total_productivity_minutes / 60;
    let minutes_productive = total_productivity_minutes % 60;

    println!(
        "{}",
        format!(
            "\nYou have been productive for {} hours and {} minutes",
            hours_productive, minutes_productive
        )
        .green()
    );

    for (task, task_data) in &user_settings.today.todays_tasks {
        let task_hours = task_data.minutes_spent / 60;
        let task_minutes = task_data.minutes_spent % 60;
        println!(
            "{}: {} hours and {} minutes",
            task, task_hours, task_minutes
        );
    }

    println!("Here Are Today's and Tomorrow's Deadlines");

    loop {
        println!("\nEnter task code (C for Coding, R for Reading, A for Audio, W for Writing, L for Learning, Ch for Chores, E for Entertainment, or X to exit):");
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
            "ch" => "Chores",
            "e" => "Entertainment", // Add the new case for "Entertainment"
            _ => {
                println!("Invalid task code. Please try again.");
                continue;
            }
        };

        let start_time_prompt = get_time_from_user(&format!(
            "Enter start time for {} (HH:MM) or 'now' for the current time:",
            task_name
        ));

        let end_time_prompt = format!(
            "Started at {} Enter end time for {} (HH:MM) or 'now' for the current time:",
            start_time_prompt.format("%H:%M"),
            task_name
        )
        .red()
        .to_string();

        let end_time = get_time_from_user(&end_time_prompt);

        let duration = end_time.signed_duration_since(start_time_prompt);

        println!(
            "\nYou Spent {} Minutes {}",
            duration.num_minutes(),
            task_name
        );

        let duration_minutes = duration.num_minutes() as u64;

        if task_name == "Chores" {
            let task_entry = user_settings
                .today
                .todays_chores
                .entry(task_name.to_string())
                .or_insert(Task { minutes_spent: 0 });
            task_entry.minutes_spent += duration_minutes;
        } else if task_name == "Entertainment" {
            let task_entry = user_settings
                .today
                .todays_entertainment
                .entry(task_name.to_string())
                .or_insert(Task { minutes_spent: 0 });
            task_entry.minutes_spent += duration_minutes;
        } else {
            let task_entry = user_settings
                .today
                .todays_tasks
                .entry(task_name.to_string())
                .or_insert(Task { minutes_spent: 0 });
            task_entry.minutes_spent += duration_minutes;

            total_productivity_minutes += duration_minutes;
        }

        let hours_productive = total_productivity_minutes / 60;
        let minutes_productive = total_productivity_minutes % 60;

        println!(
            "{}",
            format!(
                "You have been productive for {} hours and {} minutes",
                hours_productive, minutes_productive
            )
            .green()
        );

        for (task, task_data) in &user_settings.today.todays_tasks {
            let task_hours = task_data.minutes_spent / 60;
            let task_minutes = task_data.minutes_spent % 60;
            println!(
                "{}: {} hours and {} minutes",
                task, task_hours, task_minutes
            );
        }

        // Print chores separately
        for (chore, chore_data) in &user_settings.today.todays_chores {
            let chore_hours = chore_data.minutes_spent / 60;
            let chore_minutes = chore_data.minutes_spent % 60;
            println!(
                "{}: {} hours and {} minutes",
                chore, chore_hours, chore_minutes
            );
        }

        // Print entertainment separately
        for (entertainment, entertainment_data) in &user_settings.today.todays_entertainment {
            let entertainment_hours = entertainment_data.minutes_spent / 60;
            let entertainment_minutes = entertainment_data.minutes_spent % 60;
            println!(
                "{}: {} hours and {} minutes",
                entertainment, entertainment_hours, entertainment_minutes
            );
        }

        save_user_settings(&user_settings);
    }
}

fn get_time_from_user(prompt: &str) -> NaiveTime {
    loop {
        println!("{}", prompt);
        let mut time_input = String::new();
        io::stdin()
            .read_line(&mut time_input)
            .expect("Failed to read line");
        let time_input = time_input.trim();
        if time_input.eq_ignore_ascii_case("now") {
            return (Utc::now() + Duration::hours(2)).time();
        } else if time_input.len() == 4 && time_input.chars().all(char::is_numeric) {
            let formatted_time = format!("{}:{}", &time_input[0..2], &time_input[2..4]);
            match NaiveTime::parse_from_str(&formatted_time, "%H:%M") {
                Ok(time) => return time,
                Err(_) => println!("Invalid time format. Please try again."),
            }
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
    serde_json::from_reader(reader)
        .unwrap_or_else(|_| UserSettings::new(Utc::now().format("%d/%m/%Y").to_string()))
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
