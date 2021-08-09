use std::net::TcpStream;
use std::io::{BufReader, BufRead};
use redis_protocol_parser::{RedisProtocolParser, RESP};

pub fn create_commands(stream: &TcpStream) -> Vec<String> {

    let mut buffer = String::new();
    let mut reader = BufReader::new(stream);
    let recieved_data_size = reader.read_line(&mut buffer)
        .expect("Error reading line");
    if recieved_data_size == 0 {
        return vec![];
    }

    let mut full_command: String = "".to_string();
    full_command += &*buffer;

    if buffer.ends_with("\r\n") {
        buffer.pop();
        buffer.pop();
    }
    buffer.remove(0);
    let length = buffer.parse::<i64>().unwrap();

    for _ in 0..length {
        buffer = String::new();
        reader.read_line(&mut buffer)
            .expect("Error reading line");
        full_command += &*buffer;

        buffer = String::new();
        reader.read_line(&mut buffer)
            .expect("Error reading line");
        full_command += &*buffer;
    }


    match RedisProtocolParser::parse_resp(full_command.as_ref()) {
        Ok((resp, _left)) => {
            let mut return_vec = vec![];
            match resp {
                RESP::String(string) => {
                    return_vec.push(std::str::from_utf8(string).unwrap().to_string());
                }
                RESP::Error(_) => {}
                RESP::Integer(_) => {}
                RESP::BulkString(string) => {
                    return_vec.push(std::str::from_utf8(string).unwrap().to_string());
                }
                RESP::Nil => {}
                RESP::Array(array) => {
                    for item in array {
                        match item {
                            RESP::BulkString(string) => {
                                return_vec.push(std::str::from_utf8(string).unwrap().to_string());
                            }
                            RESP::String(string) => {
                                return_vec.push(std::str::from_utf8(string).unwrap().to_string());
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