use std::fmt::{Display, Formatter, Result};

pub struct Sound {
    pub name: String,
    pub frequency: f32,
    pub gain: f32,
    pub octave: i32,
}

pub const NAMES: [&str; 7] = ["Do", "Ré", "Mi", "Fa", "Sol", "La", "Si"];
pub const SEMITONES: [i32; 7] = [0, 2, 4, 5, 7, 9, 11];

impl Sound {
    pub fn new(name: String, frequency: f32, gain: f32, octave: i32) -> Self {
        Self {
            name,
            frequency,
            gain,
            octave,
        }
    }
}

impl Display for Sound {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result {
        formatter.write_fmt(format_args!(
            "name:{}, f:{}, g:{}, o:{}",
            self.name, self.frequency, self.gain, self.octave
        ))
    }
}
