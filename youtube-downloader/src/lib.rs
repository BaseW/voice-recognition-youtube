use youtube_dl::{Error, SingleVideo, YoutubeDl};

pub async fn download_movie(url: &str) -> Result<SingleVideo, Error> {
    let output = YoutubeDl::new(url)
        .download(true)
        .socket_timeout("15")
        .output_directory("tmp")
        .run_async()
        .await
        .expect("Failed to get video information");
    let single_video = output.into_single_video().expect("Failed to download");
    Ok(single_video)
}
