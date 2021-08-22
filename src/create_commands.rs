use std::io::{BufReader, BufRead, Read};
use redis_protocol_parser::{RedisProtocolParser, RESP};
use std::str::{from_utf8};
use crate::server::Storage;
use async_std::net::TcpStream;

pub(crate) fn create_commands(stream: &TcpStream) -> Vec<Storage> {

    let mut buffer = String::new();
    let mut reader = BufReader::new(stream);
    let recieved_data_size = reader.read_line(&mut buffer)
        .expect("Error reading line");
    if recieved_data_size == 0 {
        return vec![];
    }
    let mut full_command = vec![];
    full_command.append(&mut buffer.clone().as_bytes().to_vec());

    if buffer.starts_with("*") {
        // array
        let mut array_length_string = buffer.clone();
        array_length_string.pop();
        array_length_string.pop();
        array_length_string.remove(0);
        let array_length = array_length_string.parse::<usize>().unwrap();

        for _ in 0..array_length {
            let mut length_string = String::new();
            reader.read_line(&mut length_string)
                .expect("Error reading line");
            full_command.append(&mut length_string.clone().as_bytes().to_vec());
            if length_string.starts_with("$") {
                length_string.pop();
                length_string.pop();
                length_string.remove(0);
                let item_length = length_string.parse::<usize>().unwrap();
                let mut item_buffer = vec![0; item_length+2];
                reader.read_exact(&mut item_buffer).expect("Error reading value");
                full_command.append(&mut item_buffer);
            }
        }
    }

    macro_rules! string_match {
        ($v:ident, $s:expr) => {
            match from_utf8($s) {
                Ok(item) => {
                    $v.push(Storage::String {
                        value: item.to_string(),
                        created: -1.0,
                        expire: -1.0
                    })
                }
                Err(_) => {
                    $v.push(Storage::Bytes {
                        value: $s.to_vec(),
                        created: -1.0,
                        expire: -1.0
                    })
                }
            }
        }
    }

    match RedisProtocolParser::parse_resp(full_command.as_ref()) {
        Ok((resp, _left)) => {
            let mut return_vec: Vec<Storage> = vec![];
            match resp {
                RESP::String(string) => {
                    string_match!(return_vec, string)
                }
                RESP::Error(_) => {}
                RESP::Integer(_) => {}
                RESP::BulkString(string) => {
                    string_match!(return_vec, string)
                }
                RESP::Nil => {}
                RESP::Array(array) => {
                    for item in array {
                        match item {
                            RESP::BulkString(string) => {
                                string_match!(return_vec, string)
                            }
                            RESP::String(string) => {
                                string_match!(return_vec, string)
                            }
                            _ => {}
                        }
                    }
                }
            }

            return return_vec;
        }
        Err(_error) => {
            return vec![];
        }
    }
}

