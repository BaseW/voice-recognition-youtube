use youtube_downloader::{download_movie, search_videos};

#[derive(clap::Parser, Debug)]
pub struct Args {
    pub query: Option<String>,
    pub limit: Option<usize>,
    pub id: Option<String>,
}

pub async fn download(
    search_query: Option<String>,
    count: Option<usize>,
    target_video_id: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    // if target_video_id is some and mode is some, recognize video
    if let Some(target_video_id) = target_video_id {
        let url = format!("https://www.youtube.com/watch?v={}", target_video_id);
        let download_file_path = format!("tmp/{}.webm", target_video_id);
        download_movie(&url, &download_file_path).await.unwrap();
        return Ok(());
    }
    // if search_query is some, download and recognize
    if let Some(search_query) = search_query {
        let count = count.unwrap_or(10);
        let videos = search_videos(search_query, count).await;
        // print videos with index
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
        // print prompt to select target video index
        println!("select target video index: ");
        // get target video index from stdin
        let mut target_video_index = String::new();
        std::io::stdin().read_line(&mut target_video_index).unwrap();
        let target_video_index = target_video_index.trim().parse::<usize>().unwrap();
        // check index is valid
        if target_video_index >= videos.len() {
            println!("invalid index");
            std::process::exit(1);
        }
        let target_video_id = (&videos)[target_video_index].id.as_str().to_string();
        let url = format!("https://www.youtube.com/watch?v={}", target_video_id);
        let download_file_path = format!("tmp/{}.webm", target_video_id);
        download_movie(&url, &download_file_path).await.unwrap();
    }
    Ok(())
}
