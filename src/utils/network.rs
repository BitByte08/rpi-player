use std::process::Command;

pub fn internet_available() -> bool {
    // 1회만 ping 시도
    Command::new("ping")
        .args(&["-c", "1", "8.8.8.8"]) // 구글 DNS
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}