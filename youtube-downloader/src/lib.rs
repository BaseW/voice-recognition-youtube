use youtube_dl::{Error, SearchOptions, SingleVideo, YoutubeDl};

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

pub async fn search_videos(search_query: String, count: usize) -> Vec<SingleVideo> {
    let search_options = SearchOptions::youtube(search_query).with_count(count);
    let youtube_dl = YoutubeDl::search_for(&search_options);
    match youtube_dl.run() {
        Ok(output) => match output.into_playlist() {
            Some(playlist) => {
                if let Some(entries) = playlist.entries {
                    entries
                } else {
                    Vec::new()
                }
            }
            None => Vec::new(),
        },
        Err(err) => {
            println!("err: {:?}", err);
            Vec::new()
        }
    }
}
