use chrono::{Datelike, Duration, NaiveDateTime, Utc};

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
    )
}
