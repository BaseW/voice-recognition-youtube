use youtube_downloader::download_movie;

use crate::{
    ffmpeg::convert_file_by_ffmpeg, vosk::split_and_recognize,
    whisper::recognize as recognize_by_whisper,
};

#[derive(clap::Parser, Debug)]
pub struct Args {
    pub id: String,
    pub mode: String,
}

pub async fn recognize(target_video_id: &str, mode: &str) {
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

    if mode == "vosk" {
        split_and_recognize(&target_video_id, sample_rate);
    } else if mode == "whisper" {
        recognize_by_whisper(&target_video_id);
    }
}
