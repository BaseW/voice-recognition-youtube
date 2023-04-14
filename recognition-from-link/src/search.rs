use youtube_downloader::{search_videos, SingleVideo};

#[derive(clap::Parser, Debug)]
pub struct Args {
    pub query: String,
    pub limit: Option<usize>,
}

pub async fn search(query: String, limit: Option<usize>) -> Vec<SingleVideo> {
    let limit = limit.unwrap_or(10);
    let videos = search_videos(query, limit).await;
    for (i, video) in videos.iter().enumerate() {
        // print index, title
        // if duration exists, print duration
        // if upload_date exists, print upload_date
        println!(
            "{}: {}{}{}",
            i,
            video.title,
            match &video.duration_string {
                Some(duration_string) => format!(" (再生時間: {})", duration_string),
                None => "".to_string(),
            },
            match &video.upload_date {
                Some(upload_date) => format!(" (アップロード日: {})", upload_date),
                None => "".to_string(),
            }
        );
    }
    videos
}
