use std::io;
use std::io::{Cursor, Error, Read};

use byteorder::{BigEndian, ReadBytesExt};

use crate::structs::Header;

mod markers;
mod structs;

fn main() {
    let header = read_jpg("images/test.jpg");
}

fn read_jpg(filename: &str) -> Header {
    let mut file = std::fs::File::open(filename).expect("Cannot open file");
    let mut data = Vec::new();
    file.read_to_end(&mut data).expect("Cannot read file");
    let mut reader = Cursor::new(data);
    let mut header = Header::default();
    let last = reader.read_u8().expect("Cannot read marker");
    let current = reader.read_u8().expect("Cannot read marker");
    if last != 0xff && current != markers::SOI {
        panic!("Invalid JPEG file");
    }
    let last = reader.read_u8().expect("Cannot read marker");
    let current = reader.read_u8().expect("Cannot read marker");
    while header.valid {
        if last != 0xff {
            panic!("Expected a marker");
        }
        if current >= markers::APP0 && current <= markers::APP15 {
            read_appn(&mut header, &mut reader);
            break;
        }
        let last = reader.read_u8().expect("Cannot read marker");
        let current = reader.read_u8().expect("Cannot read marker");
    }
    header
}

fn read_appn(header: &mut Header, reader: &mut Cursor<Vec<u8>>) {
    println!("Reading APPN Marker");
    let length: u16 = reader.read_u16::<BigEndian>().expect("Cannot read APPN length");
    println!("Length is {}", length);
    for i in 0..length - 2 {
        reader.read_u8().expect("Cannot read APPNs");
        ;
    }
}
