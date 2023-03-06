use vosk::{Model, Recognizer};

pub struct VoskRecognizer {
    sample_rate: u32,
    recognizer: Recognizer,
}

impl VoskRecognizer {
    pub fn new(model_path: String, sample_rate: u32) -> Self {
        let model = Model::new(model_path).expect("Could not create the model");
        let recognizer =
            Recognizer::new(&model, sample_rate as f32).expect("Could not create the recognizer");
        Self {
            sample_rate,
            recognizer,
        }
    }

    pub fn accept_waveform(&mut self, samples: &[i16]) {
        self.recognizer.accept_waveform(samples);
    }

    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    pub fn partial_result(&mut self) -> String {
        let partial_result = self.recognizer.partial_result();
        partial_result.partial.to_string()
    }
}
