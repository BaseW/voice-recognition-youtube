use youtube_downloader::download_movie;

#[tokio::main]
async fn main() {
    let url = "https://www.youtube.com/watch?v=VFbhKZFzbzk";
    // download
    let video = download_movie(url).await.unwrap();
    println!("Video title: {}", video.title);

    // convert to mp3 from webm
    let mut command = std::process::Command::new("ffmpeg");
    command
        .arg("-i")
        .arg("tmp/video.webm")
        .arg("-vn")
        .arg("-ar")
        .arg("44100")
        .arg("-ac")
        .arg("2")
        .arg("-b:a")
        .arg("192k")
        .arg("tmp/output.mp3");
    // print command
    println!("command: {:?}", command);
    // execute command
    let output = command.spawn().expect("Failed to execute ffmpeg");
    println!("output: {:?}", output);
}
