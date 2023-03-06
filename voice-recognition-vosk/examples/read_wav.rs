use hound::WavReader;
use std::env;
use voice_recognition_vosk::VoskRecognizer;

fn main() {
    // read path to WAV file from arguments
    let wav_path = env::args().nth(1).expect("No WAV file provided");

    // read path to vosk model from environment variable
    let vosk_model_path = env::var("VOSK_MODEL_PATH").expect("VOSK_MODEL_PATH not set");

    let mut reader = WavReader::open(wav_path).expect("Could not create the WAV reader");
    let samples = reader
        .samples()
        .collect::<hound::Result<Vec<i16>>>()
        .expect("Could not read WAV file");

    let mut vosk_recognizer = VoskRecognizer::new(vosk_model_path, reader.spec().sample_rate);

    for sample in samples.chunks(100) {
        vosk_recognizer.accept_waveform(sample);
        println!("{:#?}", vosk_recognizer.partial_result());
    }

    // エラーになるのでコメントアウト
    // println!("{:#?}", vosk_recognizer.final_result().multiple().unwrap());
}
