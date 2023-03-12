use hound::WavReader;
use youtube_downloader::download_movie;

#[tokio::main]
async fn main() {
    println!("downloading video...");
    let url = "https://www.youtube.com/watch?v=DZcNcQfEgnY";
    // download
    let video = download_movie(url).await.unwrap();
    println!("Video title: {}", video.title);

    println!("converting to wav...");
    let wav_path = "tmp/output.wav";
    let sample_rate = 44100;
    // convert to wav from webm
    let mut command = std::process::Command::new("ffmpeg");
    command
        .arg("-y")
        .arg("-i")
        .arg("tmp/video.webm")
        .arg("-vn")
        .arg("-ar")
        .arg(format!("{}", sample_rate))
        .arg("-ac")
        .arg("1")
        .arg("-b:a")
        .arg("192k")
        .arg(wav_path);
    // print command
    // println!("command: {:?}", command);
    // execute command
    // check status until done
    let status = command.status().expect("Failed to execute ffmpeg");
    println!("convert command exit status: {}", status);

    // split wav file into 10 seconds
    let mut command = std::process::Command::new("ffmpeg");
    command
        .arg("-y")
        .arg("-i")
        .arg(wav_path)
        .arg("-f")
        .arg("segment")
        .arg("-segment_time")
        .arg("10")
        .arg("-c")
        .arg("copy")
        .arg("tmp/output%03d.wav");
    // print command
    // println!("command: {:?}", command);
    // execute command
    // check status until done
    let status = command.status().expect("Failed to execute ffmpeg");
    println!("split command exit status: {}", status);

    // read path to vosk model from environment variable
    let vosk_model_path = std::env::var("VOSK_MODEL_PATH").expect("VOSK_MODEL_PATH not set");
    let mut vosk_recognizer =
        voice_recognition_vosk::VoskRecognizer::new(vosk_model_path, sample_rate);

    // recognize each files
    // tmp/output001.wav, tmp/output002.wav, ...
    for i in 1..=10 {
        let wav_path = format!("tmp/output{:03}.wav", i);
        println!("recognizing {}...", wav_path);
        let mut reader = WavReader::open(wav_path).expect("Could not create the WAV reader");
        let samples = reader
            .samples()
            .collect::<hound::Result<Vec<i16>>>()
            .expect("Could not read WAV file");
        for sample in samples.chunks(100) {
            vosk_recognizer.accept_waveform(sample);
            println!("{:#?}", vosk_recognizer.partial_result());
        }
    }
}
