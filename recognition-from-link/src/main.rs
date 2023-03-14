use hound::WavReader;
use youtube_downloader::download_movie;

fn convert_file_by_ffmpeg(input_file_path: &str, output_file_path: &str, sample_rate: u32) -> i32 {
    let mut command = std::process::Command::new("ffmpeg");
    command
        .arg("-y")
        .arg("-i")
        .arg(input_file_path)
        .arg("-vn")
        .arg("-ar")
        .arg(format!("{}", sample_rate))
        .arg("-ac")
        .arg("1")
        .arg("-b:a")
        .arg("192k")
        .arg(output_file_path);
    // print command
    // println!("command: {:?}", command);
    match command.status() {
        Ok(status) => {
            println!("convert command exit status: {}", status);
            status.code().unwrap()
        }
        Err(err) => {
            println!("err: {:?}", err);
            1
        }
    }
}

fn split_file_by_ffmpeg(input_file_path: &str, output_file_path: &str) -> i32 {
    let mut command = std::process::Command::new("ffmpeg");
    command
        .arg("-y")
        .arg("-i")
        .arg(input_file_path)
        .arg("-f")
        .arg("segment")
        .arg("-segment_time")
        .arg("10")
        .arg("-c")
        .arg("copy")
        .arg(output_file_path);
    // print command
    // println!("command: {:?}", command);
    match command.status() {
        Ok(status) => {
            println!("split command exit status: {}", status);
            status.code().unwrap()
        }
        Err(err) => {
            println!("err: {:?}", err);
            1
        }
    }
}

fn recognize_splitted_files(sample_rate: u32) {
    // read path to vosk model from environment variable
    let vosk_model_path = std::env::var("VOSK_MODEL_PATH").expect("VOSK_MODEL_PATH not set");
    let mut vosk_recognizer =
        voice_recognition_vosk::VoskRecognizer::new(vosk_model_path, sample_rate);

    let mut prev_recognition_result = "".to_string();
    // recognize each files
    // tmp/output001.wav, tmp/output002.wav, ...
    let mut file_count = 0;
    let mut chunk_count = 0;
    let chunk_size = 100;
    loop {
        let wav_path = format!("tmp/output{:03}.wav", file_count);
        match WavReader::open(wav_path) {
            Ok(mut reader) => {
                let samples = reader
                    .samples()
                    .collect::<hound::Result<Vec<i16>>>()
                    .expect("Could not read WAV file");
                for sample in samples.chunks(chunk_size) {
                    chunk_count = chunk_count + 1;
                    let elapsed_time = ((chunk_size * chunk_count) as f32) / (sample_rate as f32);
                    vosk_recognizer.accept_waveform(sample);
                    let current_result = vosk_recognizer.partial_result();
                    // if recognition result is different from previous result
                    if prev_recognition_result != current_result {
                        println!("{} s: {:#?}", elapsed_time, current_result);
                        prev_recognition_result = current_result;
                    }
                }
            }
            Err(err) => {
                println!("err: {:?}", err);
                break;
            }
        }
        file_count = file_count + 1;
    }
}

#[tokio::main]
async fn main() {
    let url = "https://www.youtube.com/watch?v=DZcNcQfEgnY";
    let download_file_path = "tmp/video.webm";
    // check if download file exists
    if !std::path::Path::new(download_file_path).exists() {
        // download video
        println!("downloading video...");
        let video = download_movie(url, download_file_path).await.unwrap();
        println!("Video title: {}", video.title);
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
