/// A representation of the wav file header.
#[repr(C)]
pub struct WavHeader {
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

