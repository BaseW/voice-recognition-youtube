use youtube_dl::{Error, YoutubeDl};

pub async fn download_movie(url: &str) -> Result<(), Error> {
    let output = YoutubeDl::new(url)
        .download(true)
        .socket_timeout("15")
        .output_directory("tmp")
        .run_async()
        .await
        .expect("Failed to get video information");
    let single_video = output.into_single_video().expect("Failed to download");
    let title = single_video.title;
    let duration = single_video.duration;
    println!("Video title: {}", title);
    // if duration is ok, print it
    if let Some(duration) = duration {
        println!("Video duration: {}", duration);
    }
    Ok(())
}
