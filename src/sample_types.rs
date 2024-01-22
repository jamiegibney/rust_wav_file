use super::i24;

pub trait SampleType {
    /// Assuming `sine_output` is in the range `[-1.0, 1.0]`, this method will
    /// normalize the value to the current type.
    fn sine_to_range(sine_output: f32) -> Self;
}

impl SampleType for u8 {
    fn sine_to_range(sine_output: f32) -> Self {
        (Self::MAX as f32 * (sine_output * 0.5 + 0.5)) as Self
    }
}

impl SampleType for i16 {
    fn sine_to_range(sine_output: f32) -> Self {
        let min = Self::MIN as f32;
        let max = Self::MAX as f32;
        let diff = max - min;

        (diff * (sine_output * 0.5 + 0.5) + min) as Self
    }
}

impl SampleType for i24 {
    fn sine_to_range(sine_output: f32) -> Self {
        let min = Self::MIN as f32;
        let max = Self::MAX as f32;
        let diff = max - min;

        let val = (diff * (sine_output * 0.5 + 0.5) + min) as i32;

        Self::new(val)
    }
}

impl SampleType for i32 {
    fn sine_to_range(sine_output: f32) -> Self {
        let min = Self::MIN as f32;
        let max = Self::MAX as f32;
        let diff = max - min;

        (diff * (sine_output * 0.5 + 0.5) + min) as Self
    }
}
