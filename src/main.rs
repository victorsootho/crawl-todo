use chrono::{Datelike, Duration, NaiveDateTime, Utc};
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

    println!("Add Today's Tasks");

    let mut tasks = vec![
        ("Coding", 0),
        ("Reading", 0),
        ("Audio", 0),
        ("Writing", 0),
        ("Learning", 0),
    ];
    let mut total_productivity_minutes: u64 = 0;

    println!("Enter durations for Today's Tasks");
    for (_i, (task, duration)) in tasks.iter_mut().enumerate() {
        println!("Enter duration for {} (in minutes): ", task);
        let mut crawl_input = String::new();
        io::stdin()
            .read_line(&mut crawl_input)
            .expect("Failed to read line");
        let input: u64 = crawl_input.trim().parse().expect("Invalid duration");

        *duration = input;
        total_productivity_minutes += input;
    }

    let hours_productive = total_productivity_minutes / 60;
    let minutes_productive = total_productivity_minutes % 60;
    println!(
        "You have been productive for {} hours and {} minutes",
        hours_productive, minutes_productive
    );
    for (i, (task, duration)) in tasks.iter().enumerate() {
        println!("Task {}: {} - {} minutes", i + 1, task, duration);
    }
}
