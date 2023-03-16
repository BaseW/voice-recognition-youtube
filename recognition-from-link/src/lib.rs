use hound::WavReader;
use youtube_downloader::search_videos;

pub fn convert_file_by_ffmpeg(
    input_file_path: &str,
    output_file_path: &str,
    sample_rate: u32,
) -> i32 {
    let mut command = std::process::Command::new("ffmpeg");
    command
        .arg("-y")
        .arg("-i")
        .arg(input_file_path)
        .arg("-vn")
        .arg("-ar")
        .arg(format!("{}", sample_rate))
        .arg("-ac")
        .arg("1")
        .arg("-b:a")
        .arg("192k")
        .arg(output_file_path);
    // print command
    // println!("command: {:?}", command);
    match command.status() {
        Ok(status) => {
            println!("convert command exit status: {}", status);
            status.code().unwrap()
        }
        Err(err) => {
            println!("err: {:?}", err);
            1
        }
    }
}

pub fn split_file_by_ffmpeg(input_file_path: &str, output_file_path: &str) -> i32 {
    let mut command = std::process::Command::new("ffmpeg");
    command
        .arg("-y")
        .arg("-i")
        .arg(input_file_path)
        .arg("-f")
        .arg("segment")
        .arg("-segment_time")
        .arg("10")
        .arg("-c")
        .arg("copy")
        .arg(output_file_path);
    // print command
    // println!("command: {:?}", command);
    match command.status() {
        Ok(status) => {
            println!("split command exit status: {}", status);
            status.code().unwrap()
        }
        Err(err) => {
            println!("err: {:?}", err);
            1
        }
    }
}

pub fn recognize_splitted_files(target_video_id: &str, sample_rate: u32) {
    // read path to vosk model from environment variable
    let vosk_model_path = std::env::var("VOSK_MODEL_PATH").expect("VOSK_MODEL_PATH not set");
    let mut vosk_recognizer =
        voice_recognition_vosk::VoskRecognizer::new(vosk_model_path, sample_rate);
    let mut csv_writer = csv::Writer::from_path(format!("tmp/{}.csv", target_video_id)).unwrap();
    // write header: elapsed_time, recognition_result
    csv_writer
        .write_record(&["elapsed_time", "recognition_result"])
        .expect("Could not write header");

    let mut prev_recognition_result = "".to_string();
    // recognize each files
    // tmp/output001.wav, tmp/output002.wav, ...
    let mut file_count = 0;
    let mut chunk_count = 0;
    let chunk_size = 100;
    loop {
        let wav_path = format!("tmp/{}{:03}.wav", target_video_id, file_count);
        println!("wav_path: {}", wav_path);
        match WavReader::open(wav_path) {
            Ok(mut reader) => {
                let samples = reader
                    .samples()
                    .collect::<hound::Result<Vec<i16>>>()
                    .expect("Could not read WAV file");
                for sample in samples.chunks(chunk_size) {
                    chunk_count = chunk_count + 1;
                    let elapsed_time = ((chunk_size * chunk_count) as f32) / (sample_rate as f32);
                    vosk_recognizer.accept_waveform(sample);
                    let current_result = vosk_recognizer.partial_result();
                    // get last 30 characters
                    let current_result = current_result
                        .chars()
                        .rev()
                        .take(30)
                        .collect::<String>()
                        .chars()
                        .rev()
                        .collect::<String>();
                    // if recognition result is different from previous result
                    if prev_recognition_result != current_result {
                        // println!("{} s: {:#?}", elapsed_time, current_result);
                        match csv_writer
                            .write_record(&[elapsed_time.to_string(), current_result.clone()])
                        {
                            Ok(_) => {}
                            Err(err) => {
                                println!("write to csv error: {:?}", err);
                            }
                        }
                        prev_recognition_result = current_result;
                    }
                }
            }
            Err(err) => {
                println!("failed to open wav file: {:?}", err);
                break;
            }
        }
        file_count = file_count + 1;
    }
}

pub async fn select_target_video_from_search_result(search_query: String, count: usize) -> String {
    println!("searching videos...");
    let videos = search_videos(search_query, count).await;
    // print videos with index
    for (i, video) in videos.iter().enumerate() {
        println!("{}: {}", i, video.title);
    }
    // print prompt to select target video index
    println!("select target video index: ");
    // get target video index from stdin
    let mut target_video_index = String::new();
    std::io::stdin().read_line(&mut target_video_index).unwrap();
    let target_video_index = target_video_index.trim().parse::<usize>().unwrap();
    // check index is valid
    if target_video_index >= videos.len() {
        println!("invalid index");
        std::process::exit(1);
    }
    (&videos)[target_video_index].id.as_str().to_string()
}
