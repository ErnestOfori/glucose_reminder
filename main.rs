use chrono::{Local, Timelike};
use notify_rust::Notification;
use std::{thread, time::Duration};

fn notify() {
    Notification::new()
        .summary("Blutzucker-Erinnerung")
        .body("Zeit, deinen Blutzucker zu prüfen!")
        .show()
        .unwrap();
}

fn main() {
    // Fixed reminder times in 24-hour format: (hour, minute)
    let reminder_times = vec![(7, 0), (12, 0), (20, 0)];

    println!("📅 Blood glucose reminder started!");

    loop {
        let now = Local::now();
        let current_hour = now.hour();
        let current_minute = now.minute();

        for (hour, minute) in &reminder_times {
            if current_hour == *hour && current_minute == *minute {
                notify();
                println!("🔔 Reminder sent at {:02}:{:02}", hour, minute);

                // Wait 60 seconds to avoid duplicate notifications
                thread::sleep(Duration::from_secs(60));
            }
        }

        // Sleep to reduce CPU usage
        thread::sleep(Duration::from_secs(30));
    }
}
