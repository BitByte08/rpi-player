use std::path::PathBuf;
use crate::model::search_item::{DurationItem, SearchItem};

pub fn search_local(dir: &PathBuf, keyword: &str) -> Vec<SearchItem> {
    let mut results = Vec::new();

    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(name) = path.file_stem().and_then(|n| n.to_str()) {
                if name.to_lowercase().contains(&keyword.to_lowercase()) {
                    let duration = DurationItem {
                        seconds: 0,
                        timestamp: "".to_string(),
                    };
                    results.push(SearchItem {
                        id: "".to_string(),
                        title: name.to_string(),
                        path: PathBuf::from(path.to_string_lossy().to_string()),
                        duration,
                        thumbnail: None,
                    });
                }
            }
        }
    }

    results
}

pub fn get_local_dir() -> Option<PathBuf> {
    dirs::home_dir().map(|h| h.join("music"))
}
