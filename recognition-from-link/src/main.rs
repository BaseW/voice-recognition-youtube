use recognition_from_link::{
    convert_file_by_ffmpeg, recognize_splitted_files, select_target_video_from_search_result,
    split_file_by_ffmpeg,
};
use youtube_downloader::download_movie;

#[tokio::main]
async fn main() {
    // search videos by query that was provided from args
    let search_query = std::env::args().nth(1).expect("search query not provided");
    let count = std::env::args().nth(2).expect("count not provided");
    let count = count.parse::<usize>().unwrap();

    // select target video from search result
    let target_video_id = select_target_video_from_search_result(search_query, count).await;

    let url = format!("https://www.youtube.com/watch?v={}", target_video_id);
    let download_file_path = format!("tmp/{}.webm", target_video_id);
    // check if download file exists
    if !std::path::Path::new(&download_file_path).exists() {
        // download video
        println!("downloading video...");
        let video = download_movie(&url, &download_file_path).await.unwrap();
        // if video is None, exit
        if video.is_none() {
            println!("failed to download video");
            std::process::exit(1);
        }
    }

    let converted_file_path = format!("tmp/{}.wav", target_video_id);
    let sample_rate = 44100;
    // check if converted file exists
    if !std::path::Path::new(&converted_file_path).exists() {
        // convert to wav from webm
        println!(
            "converting {} to {}...",
            download_file_path, converted_file_path
        );
        match convert_file_by_ffmpeg(&download_file_path, &converted_file_path, sample_rate) {
            0 => println!("converted successfully"),
            _ => {
                println!("failed to convert");
                std::process::exit(1);
            }
        }
    }

    // check if split files exists
    let first_file_path = format!("tmp/{}001.wav", target_video_id);
    if !std::path::Path::new(&first_file_path).exists() {
        // split wav file into 10 seconds
        println!("splitting {}...", converted_file_path);
        let splitted_file_path = format!("tmp/{}%03d.wav", target_video_id);
        match split_file_by_ffmpeg(&converted_file_path, &splitted_file_path) {
            0 => println!("splitted successfully"),
            _ => {
                println!("failed to split");
                std::process::exit(1);
            }
        }
    }

    // recognize splitted wav files
    println!("recognizing {}...", converted_file_path);
    recognize_splitted_files(&target_video_id, sample_rate);
}
