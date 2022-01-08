use std::io::{Cursor, Read, Error};
use byteorder::{ReadBytesExt};

const MARKER_START: u8 = 0xff;
const START: u8 = 0xd8;
const HEADER: u8 = 0xe0;
const QUANTIFICATION_TABLE: u8 = 0xdb;
const FRAME_START: u8 = 0xc0;
const DEFINE_HUFFMAN_TABLE: u8 = 0xc4;
const SCAN_START: u8 = 0xda;
const END: u8 = 0xd9;

fn decode_huffman(reader: &mut Cursor<Vec<u8>>) -> Result<(u8, [u8; 16], Vec<u8>), Error> {
    println!("******** Decoding huffman Table ********");
    // length of HT table
    for _ in 0..1 {
        reader.read_u8().expect("Cannot HT length");
    }
    let header = reader.read_u8()?;
    let mut lengths = [0; 16];
    reader.read_exact(&mut lengths)?;
    let mut elements = Vec::new();
    for l in lengths {
        if l > 0 {
            let mut data = vec![0;l as usize];
            reader.read_exact(&mut data)?;
            elements.append(& mut data);
        }
    }
    println!("Header {:#8b} ({})", header, header);
    println!("Lengths {:?}", lengths);
    println!("Elements {:?}", elements.len());
    println!("******** END Decoding huffman Table ********");
    Ok((header, lengths, elements))
}

fn main() {
    let mut file = std::fs::File::open("images/test.jpg").expect("Cannot open file");
    let mut data = Vec::new();
    file.read_to_end(&mut data).expect("Cannot read file");
    let mut reader = Cursor::new(data);
    let mut i = 0;
    loop {
        let byte = reader.read_u8().expect("Cannot read a byte value");
        if byte == MARKER_START {
            let bb = reader.read_u8().expect("Cannot read a byte value");
            match bb {
                START => println!("START at {}", i),
                HEADER => println!("HEADER at {}", i),
                QUANTIFICATION_TABLE => println!("QUANTIFICATION_TABLE at {}", i),
                FRAME_START => println!("FRAME_START at {}", i),
                DEFINE_HUFFMAN_TABLE => {
                    println!("DEFINE_HUFFMAN_TABLE at {}", i);
                    decode_huffman(&mut reader).expect("Cannot decode huffman table");
                }
                SCAN_START => println!("SCAN_START at {}", i),
                END => {
                    println!("END at {}", i);
                    break;
                }
                _ => {}
            }
        }
        i += 1;
    }
}
