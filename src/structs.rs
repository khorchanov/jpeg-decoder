pub struct QuantizationTable {
    pub table: [u16; 64],
    pub set: bool,
}

pub struct Header {
    pub quantization_table: [QuantizationTable; 4],
    pub valid: bool,
}

impl Default for Header {
    fn default() -> Self {
        Header {
            quantization_table: Default::default(),
            valid: true,
        }
    }
}

impl Default for QuantizationTable {
    fn default() -> Self {
        QuantizationTable {
            table: [0; 64],
            set: false,
        }
    }
}