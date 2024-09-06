
use std::collections::HashMap;
use std::thread::sleep;
use std::time::{Duration, Instant};
use chrono::Local;

mod get_focus;
mod detect_display_server;








fn main() {
    detect_display_server::detect_display_server();
    // HashMap to store the time spent on each application
    let mut app_times: HashMap<String, Duration> = HashMap::new();
    let mut current_app = get_focus::get_focused_window().unwrap_or_else(|| "Unknown".to_string());
    let mut start_time = Instant::now();

    println!("Starting application tracking...");

    loop {
        // Sleep for 1 second before polling again
        sleep(Duration::from_secs(1));

        // Get the currently focused window
        let new_app: String = get_focus::get_focused_window().unwrap_or_else(|| "Unknown".to_string());

        // If the focused window has changed
        if new_app != current_app {
            // Calculate the time spent on the previous window
            let elapsed = start_time.elapsed();
            println!(
                "{} -> Time spent on '{}': {:.2?}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                current_app,
                elapsed
            );

            // Add the time to the HashMap
            *app_times.entry(current_app.clone()).or_insert(Duration::new(0, 0)) += elapsed;

            // Switch to the new window and reset the timer
            current_app = new_app;
            start_time = Instant::now();
        }

        // Optional: Print application times periodically
        if app_times.len() % 10 == 0 {
            println!("Application times so far:");
            for (app, time_spent) in &app_times {
                println!("{}: {:.2?}", app, time_spent);
            }
        }
    }
}