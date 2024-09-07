
use std::collections::HashMap;
use std::thread::sleep;
use std::time::{Duration, Instant};
use chrono::Local;


mod get_focus;
mod detect_display_server;
mod terminal_formatting;








fn main() {
    detect_display_server::detect_display_server();
    // HashMap to store the time spent on each window
    let mut app_times: HashMap<String, Duration> = HashMap::new();
    let mut current_window = get_focus::get_focused_window().unwrap_or_else(|| "Unknown".to_string());
    let mut start_time = Instant::now();
    let mut app_total_times: HashMap<String, Duration> = HashMap::new();
    
    println!("Starting application tracking...");

    loop {
        // Sleep for 1 second before polling again
        sleep(Duration::from_secs(1));

        // let serialized_data = serde_json::to_string(&app_times).unwrap();
        // println!("{serialized_data}");

        // Get the currently focused window
        let new_app: String = get_focus::get_focused_window().unwrap_or_else(|| "Unknown".to_string());

        // If the focused window has changed
        if new_app != current_window {
            // Calculate the time spent on the previous window
            let elapsed = start_time.elapsed();
            terminal_formatting::formatting();
            println!("{} ->",Local::now().format("%Y-%m-%d %H:%M:%S"));
            println!(
                "Time spent on The Window : '{}': on this session: {:.2?}  ",
                current_window,
                elapsed
            );
            
            let splited: Vec<&str> = current_window.split("- ").collect();
            //let ser_split = serde_json::to_string(&splited).unwrap();
            let current_app = String::from(splited[splited.len()-1]);

            // Add the time to the windows HashMap
            *app_times.entry(current_window.clone()).or_insert(Duration::new(0, 0)) += elapsed;
            println!("Total Time Spent on The Window '{}': {:.2?}", current_window, app_times[&current_window]);

            // Add the time to the Apps HashMap
            *app_total_times.entry(current_app.clone()).or_insert(Duration::new(0, 0)) += elapsed;
            println!("Total Time Spent on The App '{}': {:.2?}", current_app, app_total_times[&current_app]);
            

           
            

            // Switch to the new window and reset the timer
            current_window = new_app;
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