use recognition_from_link::{
    convert_file_by_ffmpeg, recognize_splitted_files, split_file_by_ffmpeg,
};
use youtube_downloader::{download_movie, search_videos};

async fn select_target_video_from_search_result(search_query: String, count: usize) -> String {
    println!("searching videos...");
    let videos = search_videos(search_query, count).await;
    // print videos with index
    for (i, video) in videos.iter().enumerate() {
        println!("{}: {}", i, video.title);
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

#[tokio::main]
async fn main() {
    // search videos by query that was provided from args
    let search_query = std::env::args().nth(1).expect("search query not provided");
    let count = std::env::args().nth(2).expect("count not provided");
    let count = count.parse::<usize>().unwrap();

    // select target video from search result
    let target_video_id = select_target_video_from_search_result(search_query, count).await;

    let url = format!("https://www.youtube.com/watch?v={}", target_video_id);
    let download_file_path = "tmp/video.webm";
    // check if download file exists
    if !std::path::Path::new(download_file_path).exists() {
        // download video
        println!("downloading video...");
        let video = download_movie(&url, download_file_path).await.unwrap();
        // if video is None, exit
        if video.is_none() {
            println!("failed to download video");
            std::process::exit(1);
        }
    }

    let converted_file_path = "tmp/output.wav";
    let sample_rate = 44100;
    // check if converted file exists
    if !std::path::Path::new("tmp/output.wav").exists() {
        // convert to wav from webm
        println!(
            "converting {} to {}...",
            download_file_path, converted_file_path
        );
        match convert_file_by_ffmpeg(download_file_path, converted_file_path, sample_rate) {
            0 => println!("converted successfully"),
            _ => {
                println!("failed to convert");
                std::process::exit(1);
            }
        }
    }

    // check if split files exists
    if !std::path::Path::new("tmp/output001.wav").exists() {
        // split wav file into 10 seconds
        println!("splitting {}...", converted_file_path);
        match split_file_by_ffmpeg(converted_file_path, "tmp/output%03d.wav") {
            0 => println!("splitted successfully"),
            _ => {
                println!("failed to split");
                std::process::exit(1);
            }
        }
    }

    // recognize splitted wav files
    println!("recognizing {}...", converted_file_path);
    recognize_splitted_files(sample_rate);
}
