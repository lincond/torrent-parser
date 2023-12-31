use core::panic;
use std::env;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::process::exit;
use std::{println, todo};

#[derive(Debug)]
enum BencodeType {
    Integer(i32),
    List(Vec<BencodeType>),
    Dict(HashMap<String, BencodeType>),
    ByteString(Vec<u8>)
}

fn byte_string_to_string(byte_string: &[u8]) -> String {
    String::from_utf8(byte_string.to_vec()).expect("ERROR: unable to transform byte_string to string")
}

fn parse_byte_string(buffer: &[u8], cursor: &mut usize) -> Vec<u8> {
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

    let result: Vec<u8> = buffer[*cursor..(*cursor + len)].to_vec();
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
                let value = parse_int(buffer, cursor);
                list.push(BencodeType::Integer(value));
            }, 
            b'l' => {
                let value = parse_list(buffer, cursor);
                list.push(BencodeType::List(value));
            },
            b'd' => {
                let value = parse_dict(buffer, cursor);
                list.push(BencodeType::Dict(value));
            },
            _ => {
                if byte.is_ascii_digit() {
                    let value = parse_byte_string(buffer, cursor);
                    list.push(BencodeType::ByteString(value.clone()));
                } else {
                    println!("cursor: {}", byte);
                    panic!("ERROR: unexpected byte while parse_list");
                }
            }
        }
    }

    *cursor += 1; // consume 'e'
    list
}

fn parse_dict(buffer: &[u8], cursor: &mut usize) -> HashMap<String, BencodeType> {
    *cursor += 1;

    let mut dict: HashMap<String, BencodeType> = HashMap::new();
    while buffer[*cursor] != b'e' {
        let byte_string = parse_byte_string(buffer, cursor); 
        let key = byte_string_to_string(&byte_string);

        let byte = buffer[*cursor];
        match byte {
            b'i' => {
                let value = parse_int(buffer, cursor);
                dict.insert(key, BencodeType::Integer(value));
            },
            b'l' => {
                let value = parse_list(buffer, cursor);
                dict.insert(key, BencodeType::List(value));
            },
            b'd' => {
                let value = parse_dict(buffer, cursor);
                dict.insert(key, BencodeType::Dict(value));
            },
            _ => {
                if byte.is_ascii_digit() {
                    let value = parse_byte_string(buffer, cursor);
                    dict.insert(key, BencodeType::ByteString(value));
                } else {
                    println!("cursor: {}", byte as char);
                    todo!();
                }
            }
        }
    }

    *cursor += 1; // consume 'e'
    dict
}

fn print_bencode_item(key: &String, item: &BencodeType) {
    match item {
        BencodeType::ByteString(byte_string) => {
            // pieces = SHA1 hashes not parsable into UTF-8 string
            if key != "pieces" {
                print!("{key}: ");
                let item = byte_string_to_string(byte_string);
                println!("{}", item);
            }
        }, 
        BencodeType::Integer(i) => {
            print!("{key}: ");
            println!("{}", i);
        },
        BencodeType::List(list) => {
            for list_item in list {
                print_bencode_item(key, list_item);
            }
        },
        BencodeType::Dict(dict) => {
            for (key, value) in dict.iter() {
                print_bencode_item(key, value);
            }
        }
    }
}

fn parse_torrent_file(buffer: &[u8]) {
    let mut cursor: usize = 0;
    let mut metainfo: HashMap<String, BencodeType>;

    while cursor < buffer.len() {
        let byte = buffer[cursor];
        match byte {
            b'd' => {
                metainfo = parse_dict(buffer, &mut cursor);
                for (key, item) in metainfo.iter() {
                    print_bencode_item(key, item);
                }
            },
            b'\n' => {
                println!("File parsed.");
                break;
            },
            _ => {
                panic!("ERROR: unknown byte {byte} found at pos {cursor}");
            }
        }
    }

}


fn main() {
    let args: Vec<_> = env::args().collect();
    if args.is_empty() {
        println!("usage: torrent-parser <torrent file path>");
        exit(1)
    }

    let file_path = &args[1];
    let mut file = File::open(file_path).expect("ERROR: cannot open the torrent file");
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer).expect("ERROR: failed to read the torrent file");
    parse_torrent_file(&buffer)
}
