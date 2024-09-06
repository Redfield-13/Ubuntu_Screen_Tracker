use std::process::Command;

pub fn get_focused_window() -> Option<String> {
    // Run the xdotool command to get the name of the currently focused window
    let output = Command::new("xdotool")
        .arg("getwindowfocus")
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                // Convert output from bytes to string
                let window_name = String::from_utf8_lossy(&output.stdout).trim().to_string();
                Some(window_name)
            } else {
                eprintln!("Error: xdotool command failed");
                None
            }
        }
        Err(e) => {
            eprintln!("Error: Failed to execute xdotool: {}", e);
            None
        }
    }
}