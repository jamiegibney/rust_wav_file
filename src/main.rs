#![allow(clippy::suboptimal_flops)]
use std::{f32::consts::TAU, io::Result};

/// Module for a unsigned 24-bit integer type.
mod unsigned_24_bit_int;
mod to_sample_range;
use unsigned_24_bit_int::u24;
use to_sample_range::ToSampleRange;

// The sample rate of the sine wave and output wav file.
const SAMPLE_RATE: f32 = 44100.0;

/// A representation of the wav file header.
#[repr(C)]
struct WavHeader {
    /// Marks the file as a RIFF file (value: `"RIFF"`).
    riff: [u8; 4],
    /// Size of the overall file in bytes.
    file_size: u32,
    /// File type header (value: `"WAVE"`).
    file_type: [u8; 4],
    /// Format chunk marker (value: `"fmt "`).
    format_marker: [u8; 4],
    /// Length of the format data — the stuff above.
    format_data_len: u32,
    /// Type of format, where `1` is PCM.
    format_type: u16,
    /// The number of signal channels.
    num_channels: u16,
    /// The sample rate, i.e. number of samples per second.
    sample_rate: u32,
    /// The number of bits per second: `(sample_rate * bits_per_sample * channels) / 8`.
    bit_rate: u32,
    /// `(bits_per_sample * channels) / 8`, or `bit_rate / sample_rate`
    bits_for_all_channels: u16,
    /// The bit depth, i.e. number of bits per sample.
    bits_per_sample: u16,
    /// Data chunk marker (value: `"data"`).
    data_header: [u8; 4],
    /// The size of the data section in bytes.
    data_size: u32,
}

impl WavHeader {
    /// Creates a new, initialized wav file header.
    pub fn new(sample_rate: f32, bit_depth: u16, num_channels: u16) -> Self {
        Self {
            riff: *b"RIFF",
            file_size: 0,
            file_type: *b"WAVE",
            format_marker: *b"fmt ",
            format_data_len: 16,
            format_type: 1,
            num_channels,
            sample_rate: sample_rate as u32,
            bit_rate: (sample_rate as u32
                * bit_depth as u32
                * num_channels as u32)
                / 8,
            bits_for_all_channels: (bit_depth * num_channels) / 8,
            bits_per_sample: bit_depth,
            data_header: *b"data",
            data_size: 0,
        }
    }

    pub fn set_data_size(&mut self, size_bytes: u32) {
        self.file_size = std::mem::size_of::<Self>() as u32 + size_bytes;
        self.data_size = size_bytes;
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
    // let sine_8_bit = create_wav::<u8>();
    // let sine_16_bit = create_wav::<u16>();
    // let sine_24_bit = create_wav::<u24>();
    let sine_32_bit = create_wav::<f32>();

    // write the file
    std::fs::write("sine.wav", sine_32_bit)?;

    Ok(())
}

fn create_wav<T: ToSampleRange>() -> Vec<u8> {
    // our sine wave
    let sine_data = make_sine::<T>(2.0, 440.0, SAMPLE_RATE);
    let sine_bytes = slice_to_bytes(&sine_data);

    // our wave file header
    let mut header = WavHeader::new(SAMPLE_RATE, 8, 1);
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
