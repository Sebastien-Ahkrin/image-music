use crate::sound::Sound;
use rodio::source::SineWave;
use rodio::{OutputStream, OutputStreamHandle, Sink, Source};
use std::time::Duration;

pub struct Player {
    stream: OutputStream,
    stream_handle: OutputStreamHandle,
    sink: Sink,
}

impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}

impl Player {
    pub fn new() -> Self {
        let (stream, handle) = OutputStream::try_default().expect("Failed to initialize stream");
        let sink = Sink::try_new(&handle).expect("Failed to create sink");

        Self {
            stream,
            stream_handle: handle,
            sink,
        }
    }

    pub fn play_note(&self, sound: &Sound, duration: u64) {
        let source = SineWave::new(sound.frequency)
            .take_duration(Duration::from_millis(duration))
            .amplify(sound.gain);

        self.sink.append(source);
    }
}
