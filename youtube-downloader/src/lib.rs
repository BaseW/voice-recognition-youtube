use youtube_dl::{Error, SingleVideo, YoutubeDl};

pub async fn download_movie(url: &str, output_file_name: &str) -> Result<SingleVideo, Error> {
    let output = YoutubeDl::new(url)
        .download(true)
        .socket_timeout("15")
        .output_directory("tmp")
        .output_template(output_file_name)
        .run_async()
        .await
        .expect("Failed to get video information");
    let single_video = output.into_single_video().expect("Failed to download");
    Ok(single_video)
}
