use std::io::{Cursor, Read};
use byteorder::{BigEndian, ReadBytesExt};

const START: u16 = 0xffd8;
const HEADER: u16 = 0xffe0;
const QUANT_TABLE: u16 = 0xffdb;
const FRAME_START: u16 = 0xffc0;
const DEFINE_HUFFMAN_TABLE: u16 = 0xffc4;
const SCAN_START: u16 = 0xffda;
const END: u16 = 0xffd9;

fn main() {
    let mut file = std::fs::File::open("images/img2.jpg").expect("Cannot open file");
    let mut data = Vec::new();
    file.read_to_end(&mut data).expect("Cannot read file");
    let mut reader = Cursor::new(data);
    loop {
        let bytes = reader.read_u16::<BigEndian>().expect("Cannot read a double byte value");
        match bytes {
            START => println!("START of file {:#04x}", bytes),
            HEADER => println!("HEADER of file {:#04x}", bytes),
            QUANT_TABLE => println!("QUANT_TABLE of file {:#04x}", bytes),
            FRAME_START => println!("FRAME_START of file {:#04x}", bytes),
            DEFINE_HUFFMAN_TABLE => println!("DEFINE_HUFFMAN_TABLE of file {:#04x}", bytes),
            SCAN_START => println!("SCAN_START of file {:#04x}", bytes),
            END => {
                println!("END of file {:#04x}", bytes);
                break;
            }
            _ => {}
        }
    }
}
