use crate::serialization::save_user_settings; // Import save_user_settings function from serialization module
use crate::task::{Task, UserSettings};
use chrono::Datelike;
use chrono::{Duration, NaiveTime, Utc};
use colored::*;
use std::io::{self};

pub fn get_time_from_user(prompt: &str) -> NaiveTime {
    loop {
        println!("{}", prompt);
        let mut time_input = String::new();
        io::stdin()
            .read_line(&mut time_input)
            .expect("Failed to read line");
        let time_input = time_input.trim();
        if time_input.is_empty() || time_input.eq_ignore_ascii_case("now") {
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

pub fn display_summary(
    user_settings: &mut UserSettings,
    current_time: &chrono::DateTime<Utc>,
    end_of_day: chrono::DateTime<Utc>,
) {
    let start_time = user_settings.get_start_time();
    println!(
        "Welcome to a new day. You started today at {}. Current time is {}",
        start_time.format("%H:%M:%S"),
        current_time.format("%H:%M:%S")
    );

    let current_weekday = current_time.date_naive().weekday();
    println!(
        "Schedule for {}, {}",
        current_time.format("%d/%m/%Y"),
        current_weekday
    );

    let remaining_hours = end_of_day - *current_time;
    let hours_left = remaining_hours.num_hours();
    let minutes_left = remaining_hours.num_minutes() % 60;

    println!(
        "You have {} hours and {} minutes left.",
        hours_left, minutes_left
    );

    let total_productivity_minutes: u64 = user_settings
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
            "You have been productive for {} hours and {} minutes",
            hours_productive, minutes_productive
        )
        .green()
    );

    if hours_productive >= 8 {
        println!("You had a good day")
    } else if hours_productive >= 4 {
        println!("You had a good day. It could get better.")
    } else {
        println!("Keep Going")
    }

    for (task, task_data) in &user_settings.today.todays_tasks {
        let task_hours = task_data.minutes_spent / 60;
        let task_minutes = task_data.minutes_spent % 60;
        println!(
            "{}: {} hours and {} minutes",
            task, task_hours, task_minutes
        );
    }

    for (chore, chore_data) in &user_settings.today.todays_chores {
        let chore_hours = chore_data.minutes_spent / 60;
        let chore_minutes = chore_data.minutes_spent % 60;
        println!(
            "{}: {} hours and {} minutes",
            chore, chore_hours, chore_minutes
        );
    }

    for (entertainment, entertainment_data) in &user_settings.today.todays_entertainment {
        let entertainment_hours = entertainment_data.minutes_spent / 60;
        let entertainment_minutes = entertainment_data.minutes_spent % 60;
        println!(
            "{}: {} hours and {} minutes",
            entertainment, entertainment_hours, entertainment_minutes
        );
    }
}

pub fn prompt_task(user_settings: &mut UserSettings) -> bool {
    let mut total_productivity_minutes: u64 = user_settings
        .today
        .todays_tasks
        .values()
        .map(|task| task.minutes_spent)
        .sum();

    loop {
        // TODO fix the exit command
        println!("\nEnter task code (C for Coding, R for Reading, A for Action, W for Writing, L for Learning, Ch for Chores, E for Entertainment, or X to exit):");
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
            "a" => "Action",
            "w" => "Writing",
            "l" => "Learning",
            "ch" => "Chores",
            "e" => "Entertainment",
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
            "Started at {}. Enter end time for {} (HH:MM) or 'now' for the current time:",
            start_time_prompt.format("%H:%M"),
            task_name
        )
        .red()
        .to_string();

        loop {
            let end_time = get_time_from_user(&end_time_prompt);

            if end_time < start_time_prompt {
                println!("End time cannot be earlier than start time. Please try again.");
            } else {
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

                save_user_settings(user_settings);

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
                for (entertainment, entertainment_data) in &user_settings.today.todays_entertainment
                {
                    let entertainment_hours = entertainment_data.minutes_spent / 60;
                    let entertainment_minutes = entertainment_data.minutes_spent % 60;
                    println!(
                        "{}: {} hours and {} minutes",
                        entertainment, entertainment_hours, entertainment_minutes
                    );
                }

                break;
            }
        }
    }

    true
}
