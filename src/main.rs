#![allow(clippy::suboptimal_flops)]
use std::{
    f32::consts::TAU,
    io::{prelude::*, Result},
    ops::RangeInclusive,
};

mod unsigned_24_bit_int;
use unsigned_24_bit_int::u24;

// The sample rate of the sine wave and output wav file.
const SAMPLE_RATE: f32 = 44100.0;

/// A representation of the wav file header.
#[repr(C)]
struct WavHeader {}

impl WavHeader {
    /// Creates a new, initialized wav file header.
    pub fn new() -> Self {
        Self {}
    }

    /// Returns `self` as a slice of bytes.
    pub unsafe fn as_bytes(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                (self as *const Self).cast::<u8>(),
                std::mem::size_of::<Self>(),
            )
        }
    }
}

fn main() -> Result<()> {
    // our sine wave
    let sine_data = make_sine::<u8>(2.0, 440.0, SAMPLE_RATE);
    let sine_bytes = slice_to_bytes(&sine_data);

    // our wave file header
    let header = WavHeader::new();
    let header_bytes = unsafe { header.as_bytes() };

    // the file data
    let mut file_data = Vec::from(header_bytes);
    file_data.extend_from_slice(sine_bytes);

    // write the file
    std::fs::write("sine.wav", file_data)?;

    Ok(())
}

trait ToSampleRange {
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

/// Creates a vector of samples representing an 8-bit sine wave of duration
/// `duration_secs` seconds and at frequency `freq` Hz.
fn make_sine<T: ToSampleRange>(
    duration_secs: f32,
    freq: f32,
    sample_rate: f32,
) -> Vec<T> {
    let num_samples = (sample_rate * duration_secs) as usize;

    (0..num_samples)
        .map(|i| {
            let phase = i as f32 * freq / sample_rate;
            let sine = (TAU * phase).sin();

            T::sine_to_range(sine)
        })
        .collect()
}

fn slice_to_bytes<T: Sized>(data: &[T]) -> &[u8] {
    unsafe {
        std::slice::from_raw_parts(
            (data.as_ptr()).cast::<u8>(),
            std::mem::size_of_val(data),
        )
    }
}
