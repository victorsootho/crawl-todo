mod serialization;
mod task;
mod user_interaction;

use crate::serialization::{load_user_settings, save_user_settings};
use crate::task::TodaysTasks;
use crate::user_interaction::{display_summary, prompt_task};
use chrono::{Duration, Utc};

fn main() {
    let mut user_settings = load_user_settings();

    let current_time = Utc::now() + Duration::hours(2);
    let formatted_date = current_time.format("%d/%m/%Y").to_string();

    if user_settings.today.date != formatted_date {
        user_settings.past_tasks.push(user_settings.today.clone());
        user_settings.today = TodaysTasks::new(
            formatted_date.clone(),
            Some(current_time.format("%H:%M:%S").to_string()),
        );
        save_user_settings(&user_settings);
    }

    let start_time = user_settings.get_start_time();
    let end_of_day = user_settings.get_end_time(&current_time);

    let day_duration = end_of_day.time().signed_duration_since(start_time);
    let total_hours = day_duration.num_hours();
    let total_minutes = day_duration.num_minutes() % 60;

    println!(
        "{} hours and {} minutes to seize the day",
        total_hours, total_minutes
    );

    display_summary(&mut user_settings, &current_time, end_of_day); // Passed as mutable reference

    loop {
        if !prompt_task(&mut user_settings) {
            break;
        }
        save_user_settings(&user_settings);
    }
}
