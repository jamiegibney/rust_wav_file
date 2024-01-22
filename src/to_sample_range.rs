use super::u24;

pub trait ToSampleRange {
    /// Assuming `sine_output` is in the range `[-1.0, 1.0]`, this method will
    /// normalise the value to the current type.
    fn sine_to_range(sine_output: f32) -> Self;
}

impl ToSampleRange for u8 {
    fn sine_to_range(sine_output: f32) -> Self {
        (Self::MAX as f32 * (sine_output * 0.5 + 0.5)) as Self
    }
}

impl ToSampleRange for u16 {
    fn sine_to_range(sine_output: f32) -> Self {
        (Self::MAX as f32 * (sine_output * 0.5 + 0.5)) as Self
    }
}

impl ToSampleRange for u24 {
    fn sine_to_range(sine_output: f32) -> Self {
        Self::new((Self::MAX as f32 * (sine_output * 0.5 + 0.5)) as u32)
    }
}

impl ToSampleRange for u32 {
    fn sine_to_range(sine_output: f32) -> Self {
        (Self::MAX as f32 * (sine_output * 0.5 + 0.5)) as Self
    }
}

impl ToSampleRange for f32 {
    fn sine_to_range(sine_output: f32) -> Self {
        sine_output
    }
}
