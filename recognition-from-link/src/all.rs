use youtube_downloader::search_videos;

use crate::{download::download, recognize::recognize};

#[derive(clap::Parser, Debug)]
pub struct Args {
    pub query: Option<String>,
    pub limit: Option<usize>,
    pub id: Option<String>,
    pub mode: String,
}

pub async fn all(
    search_query: Option<String>,
    count: Option<usize>,
    target_video_id: Option<String>,
    mode: String,
) {
    // if target_video_id is some and mode is some, recognize video
    if let Some(target_video_id) = target_video_id {
        download(
            search_query,
            count,
            Some(target_video_id.as_str().to_string()),
        )
        .await
        .unwrap();
        recognize(&target_video_id, &mode).await;
        return;
    }
    // if search_query is some, download and recognize
    if let Some(search_query) = search_query {
        let count = count.unwrap_or(10);
        let videos = search_videos(search_query, count).await;
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
        match download(None, None, Some(target_video_id.as_str().to_string())).await {
            Ok(_) => {
                recognize(&target_video_id, &mode).await;
            }
            Err(e) => {
                println!("error: {}", e);
                std::process::exit(1);
            }
        }
    }
}
