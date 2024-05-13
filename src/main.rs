use chrono::{Datelike, Duration, NaiveDateTime, Utc};
use std::collections::HashMap;
use std::io;

fn main() {
    let current_time = Utc::now() + Duration::hours(2);

    println!(
        "Welcome to a new day. Time is {}",
        current_time.format("%H:%M:%S")
    );

    let current_weekday = current_time.date_naive().weekday();
    let formatted_date = current_time.format("%d/%m/%Y").to_string();

    println!("Schedule for {}, {}", formatted_date, current_weekday);

    let end_of_day = NaiveDateTime::parse_from_str(
        &format!("{} 23:00:00", current_time.format("%Y-%m-%d")),
        "%Y-%m-%d %H:%M:%S",
    )
    .expect("Invalid date format")
    .and_utc();

    let remaining_hours = end_of_day - current_time;
    let hours_left = remaining_hours.num_hours();
    let minutes_left = remaining_hours.num_minutes() % 60;

    println!(
        "You have {} hours and {} minutes left.",
        hours_left, minutes_left
    );

    println!("Here Are Today's and Tomorrow's Deadlines");

    let mut task_durations: HashMap<String, u64> = HashMap::new();
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

        println!("Enter duration for {} (in minutes):", task_name);
        let mut duration_input = String::new();
        io::stdin()
            .read_line(&mut duration_input)
            .expect("Failed to read line");
        let duration: u64 = duration_input.trim().parse().expect("Invalid duration");

        *task_durations.entry(task_name.to_string()).or_insert(0) += duration;
        total_productivity_minutes += duration;

        let hours_productive = total_productivity_minutes / 60;
        let minutes_productive = total_productivity_minutes % 60;

        println!(
            "You have been productive for {} hours and {} minutes",
            hours_productive, minutes_productive
        );

        for (task, duration) in &task_durations {
            let task_hours = *duration / 60;
            let task_minutes = *duration % 60;
            println!(
                "{}: {} hours and {} minutes",
                task, task_hours, task_minutes
            );
        }
    }
}
