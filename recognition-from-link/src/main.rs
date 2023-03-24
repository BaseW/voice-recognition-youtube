use recognition_from_link::{
    ffmpeg::convert_file_by_ffmpeg, vosk::recognize as recognize_by_vosk,
    whisper::recognize as recognize_by_whisper, youtube::select_target_video_from_search_result,
};
use youtube_downloader::download_movie;

#[tokio::main]
async fn main() {
    // search videos by query that was provided from args
    let search_query = std::env::args().nth(1).expect("search query not provided");
    let count = std::env::args().nth(2).expect("count not provided");
    let count = count.parse::<usize>().unwrap();
    // select mode from args: vosk or whisper
    let mode = std::env::args().nth(3).expect("mode not provided");
    // validate mode
    if mode != "vosk" && mode != "whisper" {
        println!("mode must be vosk or whisper");
        std::process::exit(1);
    }

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

    println!("start recognition...");
    let started_at = std::time::Instant::now();

    if mode == "vosk" {
        recognize_by_vosk(&target_video_id, sample_rate);
    } else if mode == "whisper" {
        recognize_by_whisper(&target_video_id);
    }

    println!("done");
    let elapsed = started_at.elapsed().as_secs();
    println!("elapsed time: {} s", elapsed);
}
