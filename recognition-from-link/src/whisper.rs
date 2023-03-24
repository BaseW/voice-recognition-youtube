use hound::WavReader;
use whisper_recognizer::convert_integer_to_float_audio_data;

pub fn recognize(target_video_id: &str) {
    let converted_file_path = format!("tmp/{}.wav", target_video_id);
    // check if converted file exists
    if !std::path::Path::new(&converted_file_path).exists() {
        println!("{} does not exist", converted_file_path);
        std::process::exit(1);
    }

    let whisper_model_path = std::env::var("WHISPER_MODEL_PATH").unwrap();
    let mut recognizer = whisper_recognizer::WhisperRecognizer::new(whisper_model_path);
    let mut csv_writer = csv::Writer::from_path(format!("tmp/{}.csv", target_video_id)).unwrap();
    // write header: elapsed_time, recognition_result
    csv_writer
        .write_record(&["start_timestamp", "end_timestamp", "recognition_result"])
        .expect("Could not write header");

    match WavReader::open(converted_file_path) {
        Ok(mut reader) => {
            let samples = reader
                .samples()
                .collect::<hound::Result<Vec<i16>>>()
                .expect("Could not read WAV file");
            let audio = convert_integer_to_float_audio_data(&samples);
            recognizer.start_recognition(&audio[..]);
            // Iterate through the segments of the transcript.
            let num_segments = recognizer.get_segment_count();
            for i in 0..num_segments {
                let get_segment_result = recognizer.get_segment_result(i);

                csv_writer
                    .write_record(&[
                        get_segment_result.0.to_string(),
                        get_segment_result.1.to_string(),
                        get_segment_result.2.clone(),
                    ])
                    .expect("Could not write record");
            }
        }
        Err(err) => {
            println!("failed to open wav file: {:?}", err);
            std::process::exit(1);
        }
    };
}
