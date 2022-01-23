use std::io::{Cursor, Read};

use byteorder::{BigEndian, ReadBytesExt};
use crate::constants::ZIG_ZAG_MAP;

use crate::structs::Header;
use crate::utils::print_header;
use crate::markers::*;

#[allow(dead_code)]
mod markers;
mod constants;
mod structs;
mod utils;

fn main() {
    let header = read_jpg("images/img.jpg");
    print_header(header);
}

fn read_jpg(filename: &str) -> Header {
    let mut file = std::fs::File::open(filename).expect("Cannot open file");
    let mut data = Vec::new();
    file.read_to_end(&mut data).expect("Cannot read file");
    let mut reader = Cursor::new(data);
    let mut header = Header::default();
    let mut last = reader.read_u8().expect("Cannot read marker");
    let mut current = reader.read_u8().expect("Cannot read marker");
    if last != 0xff && current != SOI {
        panic!("Invalid JPEG file");
    }
    last = reader.read_u8().expect("Cannot read marker");
    current = reader.read_u8().expect("Cannot read marker");
    loop {
        if last != 0xff {
            panic!("Expected a marker");
        }
        match current {
            SOF0 => {
                read_start_of_frame(&mut header, &mut reader);
            }
            DRI => {
                read_restart_interval(&mut header, &mut reader);
            }
            DQT => read_quantization_table(&mut header, &mut reader),
            DHT => {
                read_huffman_table(&mut header, &mut reader);
            }
            SOS => {
                read_start_of_scan(&mut header, &mut reader);
                break;
            }
            APP0..=APP15 => read_appn(&mut header, &mut reader, current),
            SOF2 => panic!("Progressive DCT is not supported"),
            COM => read_comment(&mut header, &mut reader),
            TEM => {}
            0xFF => {
                current = reader.read_u8().expect("Cannot read marker");
                continue;
            }
            JPG0..=JPG13 | DNL | DHP | EXP => read_comment(&mut header, &mut reader),
            SOI | EOI | DAC | SOF0..=SOF15 => panic!("Unexpected marker {}", current),
            _ => panic!("Unknown marker : {:#8x}", current)
        }
        last = reader.read_u8().expect("Cannot read marker");
        current = reader.read_u8().expect("Cannot read marker");
    }
    current = reader.read_u8().expect("Cannot read marker");
    // read compressed image data
    loop {
        last = current;
        current = reader.read_u8().expect("Cannot read huffman data");
        if last == 0xFF {
            //end of image
            if current == EOI {
                break;
            } else if current == 0x00 {
                header.huffman_data.push(last);
                current = reader.read_u8().expect("Cannot read huffman data");
            } else if current >= RST0 && current <= RST7 {
                current = reader.read_u8().expect("Cannot read huffman data");
            } else if current == 0xFF {
                continue;
            } else {
                panic!("Invalid marker during compression data scan {:#06x}", current);
            }
        } else {
            header.huffman_data.push(last);
        }
    }
    header
}

fn read_start_of_scan(header: &mut Header, reader: &mut Cursor<Vec<u8>>) {
    println!("Reading SOS Marker");
    if header.color_components.len() == 0 {
        panic!("SOS detected before SOF");
    }
    let length = reader.read_u16::<BigEndian>().expect("Cannot read SOS length");
    for component in &mut header.color_components {
        component.used = false;
    }

    let num_components = reader.read_u8().expect("Cannot read number of components");
    for _ in 0..num_components {
        let mut component_id = reader.read_u8().expect("Cannot read component id");
        if header.zero_based {
            component_id += 1;
        }
        if component_id > header.number_components {
            panic!("Invalid color component ID {}", component_id);
        }
        let component = &mut header.color_components[component_id as usize - 1];
        if component.used {
            panic!("Duplicate color component Id {}", component_id);
        }
        component.used = true;
        let huffman_table_ids = reader.read_u8().expect("Cannot read huffman table ids");
        component.huffman_dc_table_id = huffman_table_ids >> 4;
        component.huffman_ac_table_id = huffman_table_ids & 0x0f;
        if component.huffman_dc_table_id > 3 {
            panic!("Invalid huffman DC table id {}", component.huffman_dc_table_id);
        }
        if component.huffman_ac_table_id > 3 {
            panic!("Invalid huffman AC table id {}", component.huffman_ac_table_id);
        }
    }
    header.start_of_selection = reader.read_u8().expect("Cannot read start of selection");
    header.end_of_selection = reader.read_u8().expect("Cannot read end of selection");
    let successive_approx = reader.read_u8().expect("Cannot read sucessive approx");
    header.successive_approx_high = successive_approx >> 4;
    header.successive_approx_low = successive_approx & 0x0f;
    if header.start_of_selection != 0 || header.end_of_selection != 63 {
        panic!("Invalid spectral selection");
    }
    if header.successive_approx_high != 0 || header.successive_approx_low != 0 {
        panic!("Invalid successive approximation");
    }
    if length - 6 - (2 * num_components as u16) != 0 {
        panic!("SOS invalid");
    }
}

fn read_huffman_table(header: &mut Header, reader: &mut Cursor<Vec<u8>>) {
    println!("Reading DHT Marker");
    let mut length: i32 = reader.read_u16::<BigEndian>().expect("Cannot read SOF length") as i32;
    length -= 2;

    while length > 0 {
        let table_info = reader.read_u8().expect("Cannot read table info");
        let table_id = table_info & 0x0f;
        let ac_table = (table_info >> 4) != 0;
        if table_id > 3 {
            panic!("Invalid Huffman table ID {}", table_id);
        }
        let table;
        if ac_table {
            table = &mut header.huffman_ac_tables[table_id as usize];
        } else {
            table = &mut header.huffman_dc_tables[table_id as usize];
        }
        table.set = true;
        let mut symbol_sum = 0;
        for i in 1..=16 {
            symbol_sum += reader.read_u8().expect("cannot read symbol");
            table.offsets[i] = symbol_sum;
        }
        if symbol_sum > 162 {
            panic!("Too many symbols in the huffman table");
        }
        for i in 0..symbol_sum {
            table.symbols[i as usize] = reader.read_u8().expect("cannot read symbol");
        }
        length -= 17 + symbol_sum as i32;
    }
    if length != 0 {
        panic!("Invalid Huffman table");
    }
}

fn read_restart_interval(header: &mut Header, reader: &mut Cursor<Vec<u8>>) {
    println!("Reading DRI marker");
    let length = reader.read_u16::<BigEndian>().expect("Cannot read SOF length");
    header.restart_interval = reader.read_u16::<BigEndian>().expect("Cannot read SOF length");
    if length - 4 != 0 {
        panic!("DRI invalid");
    }
}

fn read_start_of_frame(header: &mut Header, reader: &mut Cursor<Vec<u8>>) {
    println!("Reading SOF marker");
    if header.number_components != 0 {
        panic!("Multiple SOFs detected");
    }
    let length: i32 = reader.read_u16::<BigEndian>().expect("Cannot read SOF length") as i32;
    let precision = reader.read_u8().expect("Cannot read SOF precision");
    if precision != 8 {
        panic!("Invalid precision {}", precision);
    }
    header.height = reader.read_u16::<BigEndian>().expect("Cannot read height");
    header.width = reader.read_u16::<BigEndian>().expect("Cannot read width");
    if header.height == 0 || header.width == 0 {
        panic!("Invalid JPEG dimensions");
    }
    header.number_components = reader.read_u8().expect("Cannot read number of components");
    if header.number_components == 0 {
        panic!("Number of color components must not be zero");
    }
    for _ in 0..header.number_components {
        let mut component_id = reader.read_u8().expect("Cannot read component Id");
        if component_id == 0 {
            header.zero_based = true;
        }
        if header.zero_based {
            component_id += 1;
        }
        if component_id == 4 || component_id == 5 {
            panic!("YIQ color mode not supported");
        }
        if component_id == 0 || component_id > 3 {
            panic!("Invalid component ID: {}", component_id);
        }
        let component = &mut header.color_components[component_id as usize - 1];
        if component.used {
            panic!("Duplicate color component Id");
        }
        component.used = true;
        let sampling_factor = reader.read_u8().expect("Cannot read sampling factor");
        component.horizontal_sampling_factor = sampling_factor >> 4;
        component.vertical_sampling_factor = sampling_factor & 0x0f;
        if component.horizontal_sampling_factor != 1 || component.vertical_sampling_factor != 1 {
            panic!("Invalid quantization table Id in the frame components");
        }
        component.quantization_table_id = reader.read_u8().expect("Cannot read quantization table ID");
        if component.quantization_table_id > 3 {
            panic!("Invalid quantization table Id in the frame component");
        }
        if length - 8 - (3 * header.number_components as i32) != 0 {
            panic!("SOF invalid");
        }
    }
}

fn read_quantization_table(header: &mut Header, reader: &mut Cursor<Vec<u8>>) {
    println!("Reading DQT");
    let mut length: i32 = reader.read_u16::<BigEndian>().expect("Cannot read APPN length") as i32;
    length -= 2;

    while length > 0 {
        let table_info = reader.read_u8().expect("Cannot read byte");
        length -= 1;
        let table_id = table_info & 0x0f;
        if table_id > 3 {
            panic!("Invalid Quantization Table Id = {}", table_id);
        }
        header.quantization_table[table_id as usize].set = true;
        if table_info >> 4 != 0 {
            for i in 0..64 {
                header.quantization_table[table_id as usize].table[ZIG_ZAG_MAP[i]] = reader.read_u16::<BigEndian>().expect("Unable to read big endian");
            }
            length -= 128;
        } else {
            for i in 0..64 {
                header.quantization_table[table_id as usize].table[ZIG_ZAG_MAP[i]] = reader.read_u8().expect("Unable to read big endian") as u16;
            }
            length -= 64;
        }
    }

    if length != 0 {
        panic!("DQT is invalid");
    }
}

fn read_appn(_: &mut Header, reader: &mut Cursor<Vec<u8>>, current: u8) {
    println!("Reading APPN Marker {:#06x}", current);
    let length: u16 = reader.read_u16::<BigEndian>().expect("Cannot read APPN length");
    for _ in 0..length - 2 {
        reader.read_u8().expect("Cannot read APPNs");
    }
}

fn read_comment(_: &mut Header, reader: &mut Cursor<Vec<u8>>) {
    println!("Reading Comment Marker ");
    let length: u16 = reader.read_u16::<BigEndian>().expect("Cannot read APPN length");
    for _ in 0..length - 2 {
        reader.read_u8().expect("Cannot read APPNs");
    }
}
