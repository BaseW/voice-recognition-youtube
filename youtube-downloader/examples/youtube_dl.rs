use youtube_downloader::download_movie;

#[tokio::main]
async fn main() {
    let url = "https://www.youtube.com/watch?v=VFbhKZFzbzk";
    download_movie(url).await.unwrap();
}
