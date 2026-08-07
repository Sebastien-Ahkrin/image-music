use crate::sound::{NAMES, SEMITONES, Sound};

pub struct HslPixel {
    pub hue: f32,
    pub saturation: f32,
    pub lightness: f32,
}

impl HslPixel {
    pub fn new(hue: f32, saturation: f32, lightness: f32) -> Self {
        HslPixel {
            hue,
            saturation,
            lightness,
        }
    }

    pub fn to_sound(&self) -> Sound {
        let index = (((self.hue / 360.0) * 7.0) % 7.0).floor() as usize;
        let octave = (3.0 + self.lightness * 3.0).round() as i32;

        let semitone = (octave - 4) * 12 + SEMITONES[index] - 9;
        let frequency = 440.0 * 2f32.powf(semitone as f32 / 12.0);

        let gain = 0.05 + self.saturation * 0.25;

        Sound::new(NAMES[index].to_string(), frequency, gain, octave)
    }
}
