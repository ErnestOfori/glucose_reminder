Blood Glucose Reminder – Automated Notification Tool in Rust
This Rust program is a simple automated reminder system designed to help people with diabetes remember to check their blood glucose levels at specific times of the day.

It uses:

chrono to get the current time.

notify_rust to show desktop notifications.

🔧 How it works:
It checks the current time in a loop.

If the time matches one of the preset reminder times (e.g., 07:00, 12:00, 20:00), it sends a desktop notification and prints a message in the terminal.

It then waits 60 seconds to avoid sending the same reminder multiple times.

Between checks, it sleeps for 30 seconds to reduce CPU usage.

✅ What it's used for:
This tool is helpful for automating health routines—especially for people who need regular reminders to monitor their blood sugar levels throughout the day. It runs in the background and ensures important health checks are not forgotten.
