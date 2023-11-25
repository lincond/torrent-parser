use std::fs::File;
use std::io::Read;
use std::{println, todo};

fn parse_byte_string(buffer: &[u8], cursor: &mut usize) -> String {
    let byte = buffer[*cursor];
    if !byte.is_ascii_digit() {
        panic!("Unexpected byte while parsing dict: {byte}");
    }

    let mut len_bytes = Vec::new();
    while buffer[*cursor] != b':' {
        len_bytes.push(buffer[*cursor]);
        *cursor +=1;
    }
    *cursor +=1; // consume ':'

    let len: usize = len_bytes.iter().fold(0, |acc, &byte| {
        acc * 10 + (byte - b'0') as usize
    });

    let result = String::from_utf8(buffer[*cursor..(*cursor +len)].to_vec()).expect("ERROR: unable to parse byte string");
    *cursor += len;

    result
}

fn parse_dict(buffer: &[u8], cursor: &mut usize) {
    *cursor += 1;

    while buffer[*cursor] != b'e' {
        let key = parse_byte_string(buffer, cursor);
        println!("key: {:?}", key);

        let byte = buffer[*cursor] as char;
        if byte.is_ascii_digit() {
            let value = parse_byte_string(buffer, cursor);
            println!("value: {:?}", value);
        } else {
            println!("cursor: {}", byte);
            todo!();
        }
        println!("cursor: {}", byte);
    }
}


fn main() {
    let file_path = "ubuntu-20.04.6-desktop-amd64.iso.torrent";

    let mut file = File::open(file_path).expect("ERROR: cannot open the torrent file");
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer).expect("ERROR: failed to read the torrent file");

    let mut cursor: usize = 0;

    while cursor < buffer.len() {
        let byte = buffer[cursor];
        if byte == b'd' {
            println!("dict found at position {cursor}:");
            parse_dict(&buffer, &mut cursor);
            break;
        }
        cursor += 1;
    }
}
