pub struct QuantizationTable {
    pub table: [u16; 64],
    pub set: bool,
}

#[derive(Default)]
pub struct Header {
    pub quantization_table: [QuantizationTable; 4],
    // pub frame_type: u8,
    // pub height: usize,
    // pub width: usize,
}

pub struct  ColorComponent {
    pub horizontal_sampling_factor: u8,
    pub vertical_sampling_factor: u8,
    pub quantization_table_id: u8,
}

impl Default for QuantizationTable {
    fn default() -> Self {
        QuantizationTable {
            table: [0; 64],
            set: false,
        }
    }
}