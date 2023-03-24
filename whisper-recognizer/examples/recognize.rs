// This example is not going to build in this folder.
// You need to copy this code into your project and add the whisper_rs dependency in your cargo.toml

use whisper_recognizer::WhisperRecognizer;

/// Loads a context and model, processes an audio file, and prints the resulting transcript to stdout.
fn main() {
    // read whisper model path from args
    let whisper_model_path = std::env::args().nth(1).expect("model path not provided");
    // read wav path from args
    let wav_path = std::env::args().nth(2).expect("wav path not provided");
    let mut recognizer = WhisperRecognizer::new(whisper_model_path);

    // Open the audio file.
    let mut reader = hound::WavReader::open(wav_path).expect("failed to open file");
    let hound::WavSpec {
        channels,
        sample_rate,
        ..
    } = reader.spec();

    // Convert the audio to floating point samples.
    let mut audio = whisper_rs::convert_integer_to_float_audio(
        &reader
            .samples::<i16>()
            .map(|s| s.expect("invalid sample"))
            .collect::<Vec<_>>(),
    );

    // Convert audio to 16KHz mono f32 samples, as required by the model.
    // These utilities are provided for convenience, but can be replaced with custom conversion logic.
    // SIMD variants of these functions are also available on nightly Rust (see the docs).
    if channels == 2 {
        audio = whisper_rs::convert_stereo_to_mono_audio(&audio);
    } else if channels != 1 {
        panic!(">2 channels unsupported");
    }

    if sample_rate != 16000 {
        panic!("sample rate must be 16KHz");
    }

    // start recognition
    recognizer.start_recognition(&audio[..]);

    // Iterate through the segments of the transcript.
    let num_segments = recognizer.get_segment_count();
    for i in 0..num_segments {
        let get_segment_result = recognizer.get_segment_result(i);

        println!(
            "{}, {}, {}",
            get_segment_result.0, get_segment_result.1, get_segment_result.2
        );
    }
}
