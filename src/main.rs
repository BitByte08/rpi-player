mod yt_dlp;
mod utils;
mod model;

use yt_dlp::YtDlp;
fn main() -> Result<(), String> {
    let yt = YtDlp::new()?;
    println!("[DEBUG] search 호출 전");
    let results = yt.search("marigold")?;
    println!("[DEBUG] search 호출 후, 결과 수: {}", results.len());
    for r in results {
        println!("{:?}", r);
    }
    Ok(())
}