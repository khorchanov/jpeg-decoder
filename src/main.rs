use std::io::{Cursor, Read};

use byteorder::{BigEndian, ReadBytesExt};

use crate::structs::Header;

#[allow(dead_code)]
mod markers;
mod structs;

fn main() {
    let header = read_jpg("images/test.jpg");
    print_header(header);
}

fn print_header(header: Header) {
    println!("DQT==================================");
    for i in 0..4 {
        if header.quantization_table[i].set {
            println!("Table ID: {}", i);
            print!("Table Data :");
            for j in 0..64 {
                if j % 8 == 0 {
                    println!();
                }
                print!("{:<3}\t", header.quantization_table[i].table[j]);
            }
            println!();
            println!();
        }
    }
}

fn read_jpg(filename: &str) -> Header {
    let mut file = std::fs::File::open(filename).expect("Cannot open file");
    let mut data = Vec::new();
    file.read_to_end(&mut data).expect("Cannot read file");
    let mut reader = Cursor::new(data);
    let mut header = Header::default();
    let mut last = reader.read_u8().expect("Cannot read marker");
    let mut current = reader.read_u8().expect("Cannot read marker");
    if last != 0xff && current != markers::SOI {
        panic!("Invalid JPEG file");
    }
    last = reader.read_u8().expect("Cannot read marker");
    current = reader.read_u8().expect("Cannot read marker");
    while header.valid {
        if last != 0xff {
            panic!("Expected a marker");
        }
        if current == markers::DQT {
            read_quantization_table(&mut header, &mut reader);
            break;
        } else if current >= markers::APP0 && current <= markers::APP15 {
            read_appn(&mut header, &mut reader, current);
        }
        last = reader.read_u8().expect("Cannot read marker");
        current = reader.read_u8().expect("Cannot read marker");
    }
    header
}

fn read_quantization_table(header: &mut Header, reader: &mut Cursor<Vec<u8>>) {
    println!("Reading DQT");
    let mut length: isize = reader.read_u16::<BigEndian>().expect("Cannot read APPN length") as isize;
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
                header.quantization_table[table_id as usize].table[i] = reader.read_u16::<BigEndian>().expect("Unable to read big endian");
            }
            length -= 128;
        } else {
            for i in 0..64 {
                header.quantization_table[table_id as usize].table[i] = reader.read_u8().expect("Unable to read big endian") as u16;
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
    println!("Length is {}", length);
    for _ in 0..length - 2 {
        reader.read_u8().expect("Cannot read APPNs");
    }
}
