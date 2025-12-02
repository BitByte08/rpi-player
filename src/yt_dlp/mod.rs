use std::process::Command;
use std::fs;
use std::path::Path;

/// yt-dlp 설치 여부 체크
pub fn check_yt_dlp() -> bool {
    Command::new("yt-dlp")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// yt-dlp 설치 (간단한 예제)
pub fn install_yt_dlp() -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        let url = "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp.exe";
        let out_path = "yt-dlp.exe";
        if !Path::new(out_path).exists() {
            let response = reqwest::blocking::get(url).map_err(|e| e.to_string())?;
            let bytes = response.bytes().map_err(|e| e.to_string())?;
            fs::write(out_path, &bytes).map_err(|e| e.to_string())?;
        }
    }

    #[cfg(target_os = "linux")]
    {
        // 간단히 pip 사용
        let status = Command::new("sh")
            .arg("-c")
            .arg("python3 -m pip install -U yt-dlp")
            .status()
            .map_err(|e| e.to_string())?;
        if !status.success() {
            return Err("Failed to install yt-dlp via pip".into());
        }
    }

    Ok(())
}

/// 설치 체크 후 다운로드 실행
pub fn ensure_and_download(keyword: &str) -> Result<(), String> {
    if !check_yt_dlp() {
        install_yt_dlp()?;
    }
    // ytsearch: 첫 번째 결과 다운로드
    let search_url = format!("ytsearch1:{}", keyword);
    Command::new("yt-dlp")
        .args(["-x", "--audio-format", "mp3", "--write-thumbnail", &search_url])
        .status()
        .map_err(|e| e.to_string())
        .and_then(|status| {
            if status.success() { Ok(()) } else { Err("Download failed".into()) }
        })
}
