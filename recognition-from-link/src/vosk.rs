use hound::WavReader;

use crate::ffmpeg::split_file_by_ffmpeg;

fn recognize_splitted_files(target_video_id: &str, sample_rate: u32) {
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

pub fn split_and_recognize(target_video_id: &str, sample_rate: u32) {
    let converted_file_path = format!("tmp/{}.wav", target_video_id);
    // check if converted file exists
    if !std::path::Path::new(&converted_file_path).exists() {
        println!("{} does not exist", converted_file_path);
        std::process::exit(1);
    }

    // check if split files exists
    let first_file_path = format!("tmp/{}001.wav", target_video_id);
    if !std::path::Path::new(&first_file_path).exists() {
        // split wav file into 10 seconds
        println!("splitting {}...", converted_file_path);
        let splitted_file_path = format!("tmp/{}%03d.wav", target_video_id);
        match split_file_by_ffmpeg(&converted_file_path, &splitted_file_path) {
            0 => println!("splitted successfully"),
            _ => {
                println!("failed to split");
                std::process::exit(1);
            }
        }
    }

    recognize_splitted_files(target_video_id, sample_rate);
}
