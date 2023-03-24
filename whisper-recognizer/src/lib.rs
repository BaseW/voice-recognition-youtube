use whisper_rs::{convert_integer_to_float_audio, FullParams, SamplingStrategy, WhisperContext};

pub fn convert_integer_to_float_audio_data(audio: &[i16]) -> Vec<f32> {
    convert_integer_to_float_audio(audio)
}

pub struct WhisperRecognizer {
    whisper_context: WhisperContext,
}

impl WhisperRecognizer {
    pub fn new(model_path: String) -> Self {
        let whisper_context = WhisperContext::new(&model_path).expect("Could not create the model");

        Self { whisper_context }
    }

    pub fn start_recognition(&mut self, data: &[f32]) {
        // Create a params object for running the model.
        // Currently, only the Greedy sampling strategy is implemented, with BeamSearch as a WIP.
        // The number of past samples to consider defaults to 0.
        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 0 });

        // Edit params as needed.
        // Set the number of threads to use to 1.
        params.set_n_threads(1);
        // Enable translation.
        params.set_translate(false);
        // Set the language to translate to to English.
        params.set_language(Some("ja"));
        // Disable anything that prints to stdout.
        params.set_print_special(false);
        params.set_print_progress(true);
        params.set_print_realtime(false);
        params.set_print_timestamps(true);

        // Run the model.
        self.whisper_context
            .full(params, &data[..])
            .expect("failed to run model");
    }

    pub fn get_segment_count(&self) -> i32 {
        self.whisper_context.full_n_segments()
    }

    pub fn get_segment_result(&self, index: i32) -> (i64, i64, String) {
        let segment = self
            .whisper_context
            .full_get_segment_text(index)
            .expect("failed to get segment");
        let start_timestamp = self.whisper_context.full_get_segment_t0(index);
        let end_timestamp = self.whisper_context.full_get_segment_t1(index);
        (start_timestamp, end_timestamp, segment)
    }
}
