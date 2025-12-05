use std::path::PathBuf;
use std::time::Duration;
use serde_json::Value;
use reqwest::blocking::Client;

use crate::model::search_item::{ SearchItem, DurationItem };
use crate::utils::file::search_local;
use crate::utils::network::internet_available;

use crate::yt_dlp::YtDlp;

impl YtDlp {
    pub fn search(&self, query: &str) -> Result<Vec<SearchItem>, String> {
        if internet_available() {
            self.search_online(query)
        } else if let Some(dir) = &self.local_dir {
            Ok(search_local(dir, query))
        } else {
            Err("인터넷 없음 & local_dir 미지정".to_string())
        }
    }

    fn search_online(&self, query: &str) -> Result<Vec<SearchItem>, String> {
        let url = format!("https://rpi-player.onrender.com/search?q={}", query);

        println!("[DEBUG] Render API 요청: {}", url);

        let client = Client::builder()
            .timeout(Duration::from_secs(20))
            .build()
            .map_err(|e| e.to_string())?;

        for attempt in 1..=3 {
            println!("[DEBUG] {}번째 시도...", attempt);

            match client.get(&url).send() {
                Ok(response) => {
                    println!("[DEBUG] 응답 코드: {}", response.status());

                    if !response.status().is_success() {
                        return Err(format!("서버 오류: {}", response.status()));
                    }

                    let text = response.text().unwrap_or_default();
                    println!("[DEBUG] 수신 RAW JSON: {}", text);

                    let json: Value =
                        serde_json::from_str(&text).map_err(|e| format!("JSON 파싱 실패: {}", e))?;

                    // 🔥 핵심 수정 부분 → 타입 명시
                    let items: Vec<SearchItem> = json
                        .as_array()
                        .unwrap_or(&Vec::new())
                        .iter()
                        .map(|item| {
                            // duration 처리
                            let duration_obj = &item["duration"];
                            let duration = if duration_obj.is_object() {
                                DurationItem {
                                    seconds: duration_obj["seconds"].as_i64().unwrap_or(0),
                                    timestamp: duration_obj["timestamp"].as_str().unwrap_or("").to_string(),
                                }
                            } else {
                                DurationItem {
                                    seconds: 0,
                                    timestamp: "".to_string(),
                                }
                            };

                            SearchItem {
                                id: item["id"].as_str().unwrap_or("").to_string(),
                                title: item["title"].as_str().unwrap_or("").to_string(),
                                path: PathBuf::from(item["url"].as_str().unwrap_or("").to_string()),
                                duration,
                                thumbnail: item.get("thumbnail").and_then(|v| v.as_str().map(|s| s.to_string())),
                            }
                        })
                        .collect();

                    println!("[DEBUG] 파싱된 검색 결과: {}개", items.len());
                    return Ok(items);
                }
                Err(err) => {
                    println!("[DEBUG] 요청 실패: {}", err);
                    if attempt == 3 {
                        return Err("Render 서버 응답 없음".to_string());
                    }
                }
            }
        }

        Err("알 수 없는 오류".to_string())
    }
}
