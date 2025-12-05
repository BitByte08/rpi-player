use std::process::Command;


pub fn internet_available() -> bool {
    let args = if cfg!(target_os = "windows") {
        vec!["-n", "1", "8.8.8.8"]
    } else {
        vec!["-c", "1", "8.8.8.8"]
    };

    Command::new("ping")
        .args(&args)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}