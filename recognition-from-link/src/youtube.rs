use youtube_downloader::search_videos;

pub async fn select_target_video_from_search_result(search_query: String, count: usize) -> String {
    println!("searching videos...");
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
    (&videos)[target_video_index].id.as_str().to_string()
}
