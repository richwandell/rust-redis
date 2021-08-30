use std::net::{TcpStream, SocketAddr};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use crate::server::Storage;
use std::sync::mpsc::{Sender, Receiver};
use std::io::Write;
use crate::create_commands::{create_commands};
use crate::create_command_response::create_command_respons;
use resp::{Value, encode};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::command::process_command_transaction::process_command_transaction;
use crate::command::command_response::{CommandResponse, CommandError};

extern crate encoding_rs_io;
extern crate encoding_rs;


fn create_log_msg(addr: SocketAddr, commands: Vec<Storage>) -> (String, Vec<Storage>) {
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs_f64();

    let ip = addr.ip();
    let port = addr.port();
    let mut command = "".to_string();
    for item in &commands {
        command += "\"";
        command += &match item {
            Storage::Bytes { value, created: _, expire: _ } => {
                let clone = value.clone();
                let str = String::from_utf8_lossy(&clone);
                str.to_string()
            }
            Storage::String { value, created: _, expire: _ } => value.clone(),
            Storage::List { .. } => "list".to_string(),
            Storage::Set { .. } => "set".to_string(),
            Storage::Command { .. } => "command".to_string()
        };
        command += "\" ";
    }
    command.pop();
    (format!("{:.6} [{}:{}] {}", time, ip, port, command), commands)
}

pub(crate) fn thread_loop(
    mut stream: TcpStream,
    data_map_mutex: Arc<Mutex<HashMap<String, Storage>>>,
    tx: Sender<String>,
    mprx: Receiver<String>,
) {
    let mut monitor = false;
    // let stream_mutex = Arc::new(Mutex::new(stream));
    // let stream_mutex1 = Arc::clone(&stream_mutex);
    // let stream_mutex2 = Arc::clone(&stream_mutex);

    let mut message = vec![];

    loop {
        if message.len() > 0 {
            &stream.write(message.as_ref());
        }

        let commands = create_commands(&stream);

        if commands.len() > 0 {
            // let (msg, commands) = create_log_msg(stream.peer_addr().unwrap(), commands);
            // tx.send(msg).expect("error sending message");
            match process_command_transaction(commands, &data_map_mutex) {
                Ok(result) => {
                    match result {
                        CommandResponse::Set => {
                            message = encode(&Value::String("OK".to_string()));
                        }
                        CommandResponse::Get { response } => {
                            message = encode(&response);
                        }
                        CommandResponse::Ping => {
                            message = encode(&Value::String("PONG".to_string()));
                        }
                        CommandResponse::Cmd => {
                            message = create_command_respons();
                        }
                        CommandResponse::Quit => {
                            break;
                        }
                        CommandResponse::Del { removed } => {
                            message = encode(&Value::Integer(removed));
                        }
                        CommandResponse::Keys { keys } => {
                            message = encode(&keys);
                        }
                        CommandResponse::Mset => {
                            message = encode(&Value::String("OK".to_string()));
                        }
                        CommandResponse::Mget { value } => {
                            message = encode(&value);
                        }
                        CommandResponse::GetDel { response } => {
                            message = encode(&Value::Bulk(response));
                        }
                        CommandResponse::GetSet { response } => {
                            message = encode(&Value::Bulk(response));
                        }
                        CommandResponse::Monitor => {
                            monitor = true;
                            break;
                        }
                        CommandResponse::SetEx => {
                            message = encode(&Value::String("OK".to_string()));
                        }
                    }
                }
                Err(error) => {
                    match error {
                        CommandError::Error { text } => {
                            message = encode(&Value::String(text));
                        }
                        CommandError::Null => {
                            message = encode(&Value::Null);
                        }
                    }
                }
            }
        } else {
            println!("{}", "thread killed");
            break;
        }
    }

    if monitor {
        loop {
            let msg = mprx.recv().unwrap();
            match stream.write(encode(&Value::String(msg)).as_ref()) {
                Ok(sent) => {
                    println!("wrote {}", sent);
                }
                Err(_) => {
                    println!("{}", "closing connection");
                    break;
                }
            }
        }
    }
}