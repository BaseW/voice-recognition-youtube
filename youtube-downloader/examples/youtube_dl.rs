use youtube_downloader::download_movie;

#[tokio::main]
async fn main() {
    let url = "https://www.youtube.com/watch?v=VFbhKZFzbzk";
    let output_file_name = "tmp/video.webm";
    download_movie(url, output_file_name).await.unwrap();
}
