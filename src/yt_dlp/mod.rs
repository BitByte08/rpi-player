mod init;
mod search;

use crate::utils::file;

pub struct YtDlp {
    pub path: std::path::PathBuf,
    pub local_dir: Option<std::path::PathBuf>,
}

impl YtDlp {
    pub fn new() -> Result<Self, String> {
        let path = YtDlp::ensure()?;
        let local_dir = file::get_local_dir();
        Ok(Self { path, local_dir })
    }
}