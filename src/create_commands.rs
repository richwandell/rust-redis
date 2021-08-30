use std::net::TcpStream;
use std::io::{BufReader, BufRead, Read};
use redis_protocol_parser::{RedisProtocolParser, RESP, RError};
use std::str::{from_utf8};
use crate::server::Storage;
use crate::command::commands::RedisCommand;

// GetDel,
// GetSet,
// Monitor,
// SetEx

fn find_command(buffer: &Vec<u8>) -> RedisCommand {
    if buffer == &vec![71, 69, 84] {
        return RedisCommand::Get;
    }
    if buffer == &vec![83, 69, 84] {
        return RedisCommand::Set;
    }
    if buffer == &vec![80, 73, 78, 71] {
        return RedisCommand::Ping;
    }
    if buffer == &vec![68, 69, 76] {
        return RedisCommand::Del;
    }
    if buffer == &vec![75, 69, 89, 83] {
        return RedisCommand::Keys;
    }
    if buffer == &vec![77, 83, 69, 84] {
        return RedisCommand::Mset;
    }
    if buffer == &vec![77, 71, 69, 84] {
        return RedisCommand::Mget;
    }
    if buffer == &vec![67, 79, 77, 77, 65, 78, 68] {
        return RedisCommand::Command;
    }
    if buffer == &vec![81, 85, 73, 84] {
        return RedisCommand::Quit;
    }
    if buffer == &vec![67, 79, 78, 70, 73, 71] {
        return RedisCommand::Config;
    }
    return RedisCommand::Unknown;
}

pub(crate) fn create_commands_new(stream: &TcpStream) -> Vec<Storage> {
    let mut buffer: Vec<u8> = vec![0; 10];
    let mut reader = BufReader::new(stream);
    reader.read_until(b'\n', &mut buffer).expect("Error reading bytes");

    let mut array_length = 0;
    for i in (0..buffer.len()-2).rev() {
        if buffer.get(i).unwrap() == &b'*' {
            let received_data_size_bytes = &buffer[i+1..buffer.len() - 2];
            let received_data_size_string = String::from_utf8_lossy(&received_data_size_bytes);
            array_length = received_data_size_string.parse::<usize>().unwrap();
            break;
        }
    }

    if array_length == 0 {
        return vec![];
    }
    let mut items = vec![];
    for i in 0..array_length {
        let mut buffer: Vec<u8> = vec![0; 10];
        reader.read_until(b'\n', &mut buffer).expect("Error reading bytes");
        let mut data_length = 0;
        for i in (0..buffer.len()-2).rev() {
            if buffer.get(i).unwrap() == &b'$' {
                let received_data_size_bytes = &buffer[i+1..buffer.len() - 2];
                let received_data_size_string = String::from_utf8_lossy(&received_data_size_bytes);
                data_length = received_data_size_string.parse::<usize>().unwrap();
                break;
            }
        }
        let mut item_buffer = vec![0; data_length];
        reader.read_exact(&mut item_buffer).expect("Error reading value");
        if i == 0 {
            let cmd = find_command(&item_buffer);
            items.push(Storage::Command {value: cmd});
        } else {
            items.push(Storage::Bytes {value: item_buffer, expire: -1.0, created: -1.0})
        }
        // discarding the last two bytes because it's just \r\n
        let mut item_buffer = vec![0; 2];
        reader.read_exact(&mut item_buffer).expect("Error reading value");
    }
    return items;
}

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

