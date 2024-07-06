use crate::task::UserSettings;
use chrono::{Datelike, NaiveDate};
use std::collections::HashMap;

pub fn display_stats(user_settings: &UserSettings, period: &str) {
    match period {
        "weekly" => display_weekly_stats(user_settings),
        "monthly" => display_monthly_stats(user_settings),
        _ => println!("Invalid period. Use 'daily', 'weekly', or 'monthly'."),
    }
}

fn display_weekly_stats(user_settings: &UserSettings) {
    let current_date = NaiveDate::parse_from_str(&user_settings.today.date, "%d/%m/%Y")
        .expect("Invalid date format in user settings");
    let start_of_week = current_date
        .checked_sub_signed(chrono::Duration::days(
            current_date.weekday().num_days_from_monday() as i64,
        ))
        .expect("Invalid start of week");

    let mut total_minutes_spent = 0;
    let mut task_summary: HashMap<String, u64> = HashMap::new();

    for day in &user_settings.past_tasks {
        let date = NaiveDate::parse_from_str(&day.date, "%d/%m/%Y")
            .expect("Invalid date format in past tasks");
        if date >= start_of_week && date <= current_date {
            total_minutes_spent += day.total_minutes_spent();
            update_task_summary(&mut task_summary, day);
        }
    }

    // Include today's tasks in the summary
    total_minutes_spent += user_settings.today.total_minutes_spent();
    update_task_summary(&mut task_summary, &user_settings.today);

    println!(
        "Total time spent in the past week: {} hours and {} minutes",
        total_minutes_spent / 60,
        total_minutes_spent % 60
    );

    display_task_summary("Weekly Task Summary", &task_summary);
}

fn display_monthly_stats(user_settings: &UserSettings) {
    let current_date = NaiveDate::parse_from_str(&user_settings.today.date, "%d/%m/%Y")
        .expect("Invalid date format in user settings");

    let mut total_minutes_spent = 0;
    let mut task_summary: HashMap<String, u64> = HashMap::new();

    for day in &user_settings.past_tasks {
        let date = NaiveDate::parse_from_str(&day.date, "%d/%m/%Y")
            .expect("Invalid date format in past tasks");
        if date.month() == current_date.month() && date.year() == current_date.year() {
            total_minutes_spent += day.total_minutes_spent();
            update_task_summary(&mut task_summary, day);
        }
    }

    // Include today's tasks in the summary if it's in the current month
    if current_date.month()
        == NaiveDate::parse_from_str(&user_settings.today.date, "%d/%m/%Y")
            .unwrap()
            .month()
    {
        total_minutes_spent += user_settings.today.total_minutes_spent();
        update_task_summary(&mut task_summary, &user_settings.today);
    }

    println!(
        "Total time spent in the past month: {} hours and {} minutes",
        total_minutes_spent / 60,
        total_minutes_spent % 60
    );

    display_task_summary("Monthly Task Summary", &task_summary);
}

fn update_task_summary(summary: &mut HashMap<String, u64>, day: &crate::task::TodaysTasks) {
    for (task_name, task) in &day.todays_tasks {
        *summary.entry(task_name.clone()).or_insert(0) += task.minutes_spent;
    }
    for (task_name, task) in &day.todays_chores {
        *summary.entry(task_name.clone()).or_insert(0) += task.minutes_spent;
    }
    for (task_name, task) in &day.todays_entertainment {
        *summary.entry(task_name.clone()).or_insert(0) += task.minutes_spent;
    }
}

fn display_task_summary(title: &str, tasks: &HashMap<String, u64>) {
    println!("\n{}", title);
    for (task_name, minutes_spent) in tasks {
        println!(
            "{}: {} hours and {} minutes",
            task_name,
            minutes_spent / 60,
            minutes_spent % 60
        );
    }
}

trait TaskSummary {
    fn total_minutes_spent(&self) -> u64;
}

impl TaskSummary for crate::task::TodaysTasks {
    fn total_minutes_spent(&self) -> u64 {
        self.todays_tasks
            .values()
            .map(|task| task.minutes_spent)
            .sum::<u64>()
            + self
                .todays_chores
                .values()
                .map(|task| task.minutes_spent)
                .sum::<u64>()
            + self
                .todays_entertainment
                .values()
                .map(|task| task.minutes_spent)
                .sum::<u64>()
    }
}
