use std::path::PathBuf;
use crate::model::search_item::SearchItem;

pub fn search_local(dir: &PathBuf, keyword: &str) -> Vec<SearchItem> {
    let mut results = Vec::new();

    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(name) = path.file_stem().and_then(|n| n.to_str()) {
                if name.to_lowercase().contains(&keyword.to_lowercase()) {
                    results.push(SearchItem {
                        id: "".to_string(),
                        title: name.to_string(),
                        url: path.to_string_lossy().to_string(),
                        duration: None,
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
