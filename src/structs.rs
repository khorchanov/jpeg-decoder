pub struct QuantizationTable {
    pub table: [u16; 64],
    pub set: bool,
}

pub struct Header {
    pub quantization_table: [QuantizationTable; 4],
    pub color_components: [ColorComponent; 3],
    pub huffman_dc_tables: [HuffmanTable; 4],
    pub huffman_ac_tables: [HuffmanTable; 4],
    pub frame_type: u8,
    pub number_components: u8,
    pub height: u16,
    pub width: u16,

    pub start_of_selection: u8,
    pub end_of_selection: u8,
    pub successive_approx_high: u8,
    pub successive_approx_low: u8,

    pub restart_interval: u16,
    pub zero_based: bool,
    pub huffman_data: Vec<u8>,
}

pub struct ColorComponent {
    pub horizontal_sampling_factor: u8,
    pub vertical_sampling_factor: u8,
    pub quantization_table_id: u8,
    pub huffman_dc_table_id: u8,
    pub huffman_ac_table_id: u8,
    pub used: bool,
}

pub struct HuffmanTable {
    pub symbols: [u8; 162],
    pub offsets: [u8; 17],
    pub set: bool,
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
            huffman_ac_table_id: 0,
            huffman_dc_table_id: 0,
            used: false,
        }
    }
}

impl Default for HuffmanTable {
    fn default() -> Self {
        HuffmanTable {
            symbols: [0; 162],
            offsets: [0; 17],
            set: false,
        }
    }
}

impl Default for Header {
    fn default() -> Self {
        Header {
            quantization_table: Default::default(),
            start_of_selection: 0,
            end_of_selection: 63,
            successive_approx_high: 0,
            huffman_data: Vec::new(),
            huffman_ac_tables: Default::default(),
            frame_type: 0,
            huffman_dc_tables: Default::default(),
            zero_based: false,
            restart_interval: 0,
            width: 0,
            height: 0,
            color_components: Default::default(),
            number_components: 0,
            successive_approx_low: 0,
        }
    }
}