use std::process::Command;
use std::path::{Path, PathBuf};
use std::fs;

/// yt-dlp 초기화 모듈 (Linux/macOS)
pub struct Init;

impl Init {
    /// 설치된 yt-dlp 경로 반환
    pub fn get_path() -> Result<PathBuf, String> {
        let path = dirs::home_dir()
            .ok_or("Cannot determine home directory")?
            .join(".local/bin/yt-dlp");

        if path.exists() {
            Ok(path)
        } else {
            Err("yt-dlp binary not found in ~/.local/bin".into())
        }
    }

    /// yt-dlp 설치 여부 확인
    pub fn check() -> bool {
        Self::get_path().is_ok()
    }

    /// 바이너리 다운로드 및 설치
    pub fn install() -> Result<PathBuf, String> {
        let dir = dirs::home_dir()
            .ok_or("Cannot determine home directory")?
            .join(".local/bin");

        fs::create_dir_all(&dir).map_err(|e| e.to_string())?;

        let out_path = dir.join("yt-dlp");
        if !out_path.exists() {
            println!("Downloading yt-dlp...");
            let url = "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp";
            let response = reqwest::blocking::get(url).map_err(|e| e.to_string())?;
            let bytes = response.bytes().map_err(|e| e.to_string())?;
            fs::write(&out_path, &bytes).map_err(|e| e.to_string())?;

            // 실행 권한 부여
            Command::new("chmod")
                .arg("a+rx")
                .arg(&out_path)
                .status()
                .map_err(|e| e.to_string())?;
            println!("yt-dlp installed at ~/.local/bin/yt-dlp");
        }

        Ok(out_path)
    }

    /// 설치 확인 후 필요 시 설치
    pub fn ensure() -> Result<PathBuf, String> {
        if Self::check() {
            Self::get_path()
        } else {
            println!("yt-dlp not found. Installing...");
            Self::install()
        }
    }
}
