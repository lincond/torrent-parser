use core::panic;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::{println, todo};

#[derive(Debug)]
enum BencodeType {
    Integer(i32),
    List(Vec<BencodeType>),
    Dict(HashMap<String, BencodeType>),
    ByteString(String)
}

fn parse_byte_string(buffer: &[u8], cursor: &mut usize) -> String {
    let byte = buffer[*cursor];
    if !byte.is_ascii_digit() {
        println!("last cursor pos: {cursor}");
        panic!("Unexpected byte while parsing byte_string: {byte}");
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

    let byte = buffer[*cursor] as char;
    let result = String::from_utf8(buffer[*cursor..(*cursor +len)].to_vec()).unwrap_or_default();
    *cursor += len;

    result
}

fn parse_int(buffer: &[u8], cursor: &mut usize) -> i32 {
    *cursor += 1; // consume 'i'

    let mut digits_bytes = Vec::new();
    let mut negative = false;

    while buffer[*cursor]!= b'e' {
        if buffer[*cursor] == b'-' {
            negative = true;
            *cursor += 1;
            continue;
        }
        digits_bytes.push(buffer[*cursor]);
        *cursor += 1;
    }
    *cursor += 1; // consume 'e'

    let result = digits_bytes.iter().fold(0, |acc, &byte| {
        acc * 10 + (byte - b'0') as usize
    });
    if negative {
        return -(result as i32);
    }

    result as i32
}

fn parse_list(buffer: &[u8], cursor: &mut usize) -> Vec<BencodeType> {
    *cursor += 1; // consume 'l'

    let mut list: Vec<BencodeType> = Vec::new();
    let byte = buffer[*cursor];
    while buffer[*cursor] != b'e' {
        match byte {
            b'i' => {
                println!("parse_list: integer found at position {cursor}");
                let value = parse_int(buffer, cursor);
                list.push(BencodeType::Integer(value));
            }, 
            b'l' => {
                println!("parse_list: list found at position {cursor}");
                let value = parse_list(buffer, cursor);
                list.push(BencodeType::List(value));
            },
            b'd' => {
                println!("parse_list: dict found at position {cursor}");
                let value = parse_dict(buffer, cursor);
                list.push(BencodeType::Dict(value));
            },
            _ => {
                if byte.is_ascii_digit() {
                    println!("parse_list: byte_string found at position {cursor}");
                    let value = parse_byte_string(buffer, cursor);
                    list.push(BencodeType::ByteString(value.clone()));
                    println!("value: {:?}", value);
                } else {
                    println!("cursor: {}", byte);
                    panic!("ERROR: unexpected byte while parse_list");
                }
            }
        }
    }

    println!("consuming {}", buffer[*cursor] as char);
    *cursor += 1; // consume 'e'
    list
}

fn parse_dict(buffer: &[u8], cursor: &mut usize) -> HashMap<String, BencodeType> {
    *cursor += 1;

    let mut dict: HashMap<String, BencodeType> = HashMap::new();
    while buffer[*cursor] != b'e' {
        let key = parse_byte_string(buffer, cursor);
        // println!("key: {:?}", key);

        let byte = buffer[*cursor];
        match byte {
            b'i' => {
                let value = parse_int(buffer, cursor);
                // println!("value: {:?}", value);
                dict.insert(key, BencodeType::Integer(value));
            },
            b'l' => {
                let value = parse_list(buffer, cursor);
                // println!("value: {:?}", value);
                dict.insert(key, BencodeType::List(value));
            },
            b'd' => {
                let value = parse_dict(buffer, cursor);
                dict.insert(key, BencodeType::Dict(value));
            },
            _ => {
                if byte.is_ascii_digit() {
                    let value = parse_byte_string(buffer, cursor);
                    // println!("value: {:?}", value);
                    dict.insert(key, BencodeType::ByteString(value));
                } else {
                    println!("cursor: {}", byte as char);
                    todo!();
                }
            }
        }
    }

    println!("dict consuming {}", buffer[*cursor] as char);
    *cursor += 1; // consume 'e'
    dict
}


fn main() {
    let file_path = "ubuntu-20.04.6-desktop-amd64.iso.torrent";

    let mut file = File::open(file_path).expect("ERROR: cannot open the torrent file");
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer).expect("ERROR: failed to read the torrent file");

    let mut cursor: usize = 0;

    while cursor < buffer.len() {
        let byte = buffer[cursor];
        match byte {
            b'd' => {
                println!("dict found at position {cursor}:");
                let dict = parse_dict(&buffer, &mut cursor);
                println!("dict parsed: {:?}", dict);
                println!("last pos: {}", buffer[cursor] as char);
            },
            _ => {
                println!("byte {} found at pos {}", buffer[cursor] as char, cursor);
                todo!();
            }
        }
    }
}
