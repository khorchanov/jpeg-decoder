use crate::Header;

pub fn print_header(header: Header) {
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
    println!("SOF==================================");
    println!("Frame Type : {:#8b}", header.frame_type);
    println!("Height : {}", header.height);
    println!("Width : {}", header.width);
    println!("Color Components:");
    for i in 0..header.number_components {
        let component = &header.color_components[i as usize];
        println!("Component ID: {}", i + 1);
        println!("Horizontal Sampling Factor : {}", component.horizontal_sampling_factor);
        println!("Vertical Sampling Factor : {}", component.vertical_sampling_factor);
        println!("Quantization Table ID : {}", component.quantization_table_id);
        println!();
    }
    println!("DHT==================================");
    println!("DC Tables :");
    for i in 0..4 {
        if header.huffman_dc_tables[i].set {
            println!("Table ID: {}", i);
            println!("Symbols:");
            for j in 0..16 {
                print!("{}: ", j + 1);
                for k in header.huffman_dc_tables[i].offsets[j]..header.huffman_dc_tables[i].offsets[j + 1] {
                    print!("{:#02x} ", header.huffman_dc_tables[i].symbols[k as usize]);
                }
                println!();
            }
        }
    }
    println!("AC Tables :");
    for i in 0..4 {
        if header.huffman_ac_tables[i].set {
            println!("Table ID: {}", i);
            println!("Symbols:");
            for j in 0..16 {
                print!("{}: ", j + 1);
                for k in header.huffman_ac_tables[i].offsets[j]..header.huffman_ac_tables[i].offsets[j + 1] {
                    print!("{:#02x} ", header.huffman_ac_tables[i].symbols[k as usize]);
                }
                println!();
            }
        }
    }
    println!("DRI==================================");
    println!("Restart Interval : {}", header.restart_interval);
}