use youtube_dl::{SearchOptions, YoutubeDl};

fn main() {
    // get search query from args
    let search_query = std::env::args().nth(1).expect("search query not provided");
    let search_options = SearchOptions::youtube(search_query).with_count(5);
    let youtube_dl = YoutubeDl::search_for(&search_options);
    // list search result
    match youtube_dl.run() {
        Ok(output) => {
            // get playlist
            match output.into_playlist() {
                Some(playlist) => {
                    // print playlist entries length if entries exist
                    if let Some(entries) = playlist.entries {
                        println!("playlist entries length: {}", entries.len());
                        for entry in entries {
                            // print each video title and uploader if uploader exists
                            if let Some(uploader) = entry.uploader {
                                if let Some(upload_date) = entry.upload_date {
                                    println!(
                                        "video title: {}, uploader: {}, upload date: {}",
                                        entry.title, uploader, upload_date
                                    );
                                } else {
                                    println!(
                                        "video title: {}, uploader: {}",
                                        entry.title, uploader
                                    );
                                }
                            } else {
                                println!("video title: {}", entry.title);
                            }
                        }
                    } else {
                        println!("no playlist entries");
                    }
                }
                None => {
                    println!("no playlist");
                }
            }
        }
        Err(err) => {
            println!("err: {:?}", err);
        }
    }
}
