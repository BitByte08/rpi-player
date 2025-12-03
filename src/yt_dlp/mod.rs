mod init;


pub struct YtDlp {
    pub path: std::path::PathBuf,
}

impl YtDlp {
    pub fn new() -> Result<Self, String> {
        let path = crate::yt_dlp::init::ensure()?;
        Ok(Self { path })
    }
}