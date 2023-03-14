use youtube_dl::{Error, SingleVideo, YoutubeDl};

pub async fn download_movie(
    url: &str,
    output_file_name: &str,
) -> Result<Option<SingleVideo>, Error> {
    match YoutubeDl::new(url)
        .download(true)
        .socket_timeout("15")
        .output_directory(".")
        .output_template(output_file_name)
        .run_async()
        .await
    {
        Ok(output) => Ok(output.into_single_video()),
        Err(err) => {
            println!("err: {:?}", err);
            Err(err)
        }
    }
}
