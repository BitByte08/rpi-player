use std::path::PathBuf;
use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct DurationItem {
    pub seconds: i64,
    pub timestamp: String,
}

#[derive(Debug, Deserialize)]
pub struct SearchItem {
    pub id: String,
    pub title: String,
    pub path: PathBuf,
    pub thumbnail: Option<String>,
    pub duration: DurationItem,
}