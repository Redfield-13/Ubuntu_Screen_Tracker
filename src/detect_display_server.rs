use std::process::Command;

pub fn detect_display_server()  {
    let output = Command::new("sh")
        .arg("-c")
        .arg("echo $XDG_SESSION_TYPE")
        .output()
        .expect("Failed to execute command");

    let dis = String::from_utf8_lossy(&output.stdout).trim().to_string();
    println!("{}", dis);
    
}