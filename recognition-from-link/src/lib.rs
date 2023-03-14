use hound::WavReader;

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

pub fn recognize_splitted_files(sample_rate: u32) {
    // read path to vosk model from environment variable
    let vosk_model_path = std::env::var("VOSK_MODEL_PATH").expect("VOSK_MODEL_PATH not set");
    let mut vosk_recognizer =
        voice_recognition_vosk::VoskRecognizer::new(vosk_model_path, sample_rate);

    let mut prev_recognition_result = "".to_string();
    // recognize each files
    // tmp/output001.wav, tmp/output002.wav, ...
    let mut file_count = 0;
    let mut chunk_count = 0;
    let chunk_size = 100;
    loop {
        let wav_path = format!("tmp/output{:03}.wav", file_count);
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
                    // if recognition result is different from previous result
                    if prev_recognition_result != current_result {
                        println!("{} s: {:#?}", elapsed_time, current_result);
                        prev_recognition_result = current_result;
                    }
                }
            }
            Err(err) => {
                println!("err: {:?}", err);
                break;
            }
        }
        file_count = file_count + 1;
    }
}
