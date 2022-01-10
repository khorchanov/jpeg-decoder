use std::io::{Cursor, Read, Error};
use std::sync::PoisonError;
use byteorder::{ReadBytesExt};

const MARKER_START: u8 = 0xff;
const START: u8 = 0xd8;
const HEADER: u8 = 0xe0;
const QUANTIFICATION_TABLE: u8 = 0xdb;
const FRAME_START: u8 = 0xc0;
const DEFINE_HUFFMAN_TABLE: u8 = 0xc4;
const SCAN_START: u8 = 0xda;
const END: u8 = 0xd9;

#[derive(Debug)]
struct Node {
    value: u8,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(value: u8) -> Node {
        Node {
            value,
            left: Option::None,
            right: Option::None,
        }
    }

    fn add_left(&mut self, value : u8) {
        self.left = Some(Box::new(Node::new(value)));
    }

    fn add_right(&mut self, value : u8) {
        self.right = Some(Box::new(Node::new(value)));
    }
}

fn decode_huffman(reader: &mut Cursor<Vec<u8>>) -> Result<(u8, [u8; 16], Vec<u8>), Error> {
    println!("******** Decoding huffman Table ********");
    // length of HT table
    for _ in 0..2 {
        reader.read_u8()?;
    }
    let header = reader.read_u8()?;
    let mut lengths = [0; 16];
    reader.read_exact(&mut lengths)?;
    let mut elements = Vec::new();
    for l in lengths {
        if l > 0 {
            let mut data = vec![0; l as usize];
            reader.read_exact(&mut data)?;
            elements.append(&mut data);
        }
    }
    println!("Header {:#8b} ({})", header, header);
    println!("Lengths {:?}", lengths);
    println!("Elements {:?}", elements);
    println!("******** END Decoding huffman Table ********");
    let root = get_huffman_bits(lengths, &elements);
    println!("Root {:?}", root);
    Ok((header, lengths, elements))
}

fn get_huffman_bits(lengths: [u8; 16], elements: &Vec<u8>) -> Vec<Box<[u8]>> {
    let mut root = Vec::new();
    let mut index = 0;
    for i in 0..lengths.len() {
        for j in 0..lengths[i] {
            bits_from_length(&mut root, elements[index], i);
            index += 1;
        }
    }
    root
}

fn bits_from_length(root: &mut Vec<Box<[u8]>>, element: u8, position: usize) -> bool {
    if root.len() != 1 {
        if position == 0 {
            if root.len() < 2 {
                root.push(Box::new([element]));
                return true;
            }
            return false;
        }
        for i in 0..2 {
            if root.len() == i {
                root.push(Box::new([]));
            }
            if bits_from_length(&mut vec![root[i].clone()], element, position - 1) == true {
                return true;
            }
        }
    }
    false
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
