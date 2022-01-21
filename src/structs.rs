pub struct QuantizationTable {
    pub table: [u16; 64],
    pub set: bool,
}

#[derive(Default)]
pub struct Header {
    pub quantization_table: [QuantizationTable; 4],
    pub color_components: [ColorComponent; 3],
    pub frame_type: u8,
    pub number_components: u8,
    pub height: u16,
    pub width: u16,
    pub restart_interval: u16,
    pub zero_based: bool
}

pub struct ColorComponent {
    pub horizontal_sampling_factor: u8,
    pub vertical_sampling_factor: u8,
    pub quantization_table_id: u8,
    pub used: bool,
}

impl Default for QuantizationTable {
    fn default() -> Self {
        QuantizationTable {
            table: [0; 64],
            set: false,
        }
    }
}

impl Default for ColorComponent {
    fn default() -> Self {
        ColorComponent {
            horizontal_sampling_factor: 1,
            vertical_sampling_factor: 1,
            quantization_table_id: 0,
            used: false,
        }
    }
}