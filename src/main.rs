#![allow(clippy::suboptimal_flops)]
use std::{f32::consts::TAU, io::Result};

/// Module for a wave file header.
mod header;
/// Module for a signed 24-bit integer type.
mod signed_24_bit_int;
/// Module for audio sample types.
mod to_sample_range;

use header::WavHeader;
use signed_24_bit_int::i24;
use to_sample_range::ToSampleRange;

// The sample rate of the sine wave and output wav file.
const SAMPLE_RATE: f32 = 44100.0;
const DURATION_SECS: f32 = 0.1;
const OUTPUT_PATH: &str = "output/";

fn main() -> Result<()> {
    let sine_8_bit = create_wav::<u8>();
    std::fs::write(file_with_path("sine8.wav"), sine_8_bit)?;

    // not working
    let sine_16_bit = create_wav::<i16>();
    std::fs::write(file_with_path("sine16.wav"), sine_16_bit)?;

    // not working
    let sine_24_bit = create_wav::<i24>();
    std::fs::write(file_with_path("sine24.wav"), sine_24_bit)?;

    // not working
    let sine_32_bit = create_wav::<i32>();
    std::fs::write(file_with_path("sine32.wav"), sine_32_bit)?;

    Ok(())
}

fn file_with_path(file_name: &str) -> String {
    format!("{OUTPUT_PATH}{file_name}")
}

fn create_wav<T: ToSampleRange>() -> Vec<u8> {
    // our sine wave
    let sine_data = make_sine::<T>(DURATION_SECS, 440.0, 0.25, SAMPLE_RATE);
    let sine_bytes = unsafe { slice_to_bytes(&sine_data) };

    // our wave file header
    let mut header =
        WavHeader::new(SAMPLE_RATE, std::mem::size_of::<T>() as u16 * 8, 1);
    header.set_data_size(sine_bytes.len() as u32);
    let header_bytes = unsafe { header.as_bytes() };

    // the file data
    let mut file_data = Vec::from(header_bytes);
    file_data.extend_from_slice(sine_bytes);

    file_data
}

/// Creates a vector of samples representing an 8-bit sine wave of duration
/// `duration_secs` seconds and at frequency `freq` Hz.
fn make_sine<T: ToSampleRange>(
    duration_secs: f32,
    freq: f32,
    amp: f32,
    sample_rate: f32,
) -> Vec<T> {
    let num_samples = (sample_rate * duration_secs) as usize;

    (0..num_samples)
        .map(|i| {
            let phase = i as f32 * freq / sample_rate;
            let sine = (TAU * phase).sin() * amp;

            T::sine_to_range(sine)
        })
        .collect()
}

/// Converts a slice of `T` to a slice of bytes.
unsafe fn slice_to_bytes<T: Sized>(data: &[T]) -> &[u8] {
    unsafe {
        std::slice::from_raw_parts(
            (data.as_ptr()).cast::<u8>(),
            std::mem::size_of_val(data),
        )
    }
}
