use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use crate::server::Storage;
use spmc::{Receiver};
use std::io::Write;
use crate::create_commands::create_commands;
use crate::process_command_transaction::{process_command_transaction, CommandResponse, CommandError};
use crate::create_command_response::create_command_respons;
use resp::{Value, encode};
use std::str;



pub(crate) fn thread_loop(mut stream: TcpStream, data_map_mutex: Arc<Mutex<HashMap<String, Storage>>>, _rx: Receiver<()>) {
    let mut message = "".to_string();

    macro_rules! response {
        ($value:expr) => {
            let value = encode(&$value);
            message = str::from_utf8(&value).unwrap().parse().unwrap();
        }
    }

    macro_rules! string_response {
            ($string:expr) => {
                let value = encode(&Value::String($string));
                message = str::from_utf8(&value).unwrap().parse().unwrap();
            }
        }

    macro_rules! null_response {
            () => {
                let value = encode(&Value::Null);
                message = std::str::from_utf8(&value).unwrap().parse().unwrap();
            }
        }

    loop {
        if message != "" {
            &stream.write(message.as_ref());
        }

        let commands = create_commands(&stream);

        if commands.len() > 0 {
            if commands.len() > 0 {
                match process_command_transaction(commands, &data_map_mutex) {
                    Ok(result) => {
                        match result {
                            CommandResponse::Set => {
                                string_response!("OK".to_string());
                            }
                            CommandResponse::Get { response } => {
                                string_response!(response);
                            }
                            CommandResponse::Ping => {
                                string_response!("PONG".to_string());
                            }
                            CommandResponse::Cmd => {
                                message = create_command_respons();
                            }
                            CommandResponse::Quit => {
                                break;
                            }
                            CommandResponse::Del { removed } => {
                                response!(Value::Integer(removed));
                            }
                            CommandResponse::Keys { keys } => {
                                response!(keys);
                            }
                        }
                    }
                    Err(error) => {
                        match error {
                            CommandError::Error { text } => {
                                string_response!(text);
                            }
                            CommandError::Null => {
                                null_response!();
                            }
                        }
                    }
                }
            } else {
                message = "".to_string();
            }
        } else {
            println!("{}", "thread killed");
            break;
        }
    }
}