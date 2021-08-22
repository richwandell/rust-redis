use std::collections::HashMap;
use std::io::Write;
use std::sync::mpsc::{Receiver, Sender};
use std::time::{SystemTime, UNIX_EPOCH};
use resp::{encode, Value};
use crate::command::command_response::{CommandError, CommandResponse};
use crate::command::process_command_transaction::process_command_transaction;
use crate::create_command_response::create_command_respons;
use crate::create_commands::create_commands;
use crate::server::{Storage, create_log_msg};
use async_std::net::TcpStream;
use async_std::sync::{Arc, Mutex};
use async_std::prelude::*;

fn storage_string(storage: &Storage) -> String {
    match storage {
        Storage::Bytes { .. } => "".to_string(),
        Storage::String { value, created: _, expire: _ } => value.clone(),
        Storage::List { .. } => "".to_string(),
        Storage::Set { .. } => "".to_string()
    }
}

pub(crate) async fn connection_loop(
    mut stream: TcpStream,
    data_arc: Arc<Mutex<HashMap<String, Storage>>>,
    log_std: bool
) {
    let mut message = vec![];
    let mut monitor = false;
    loop {
        if message.len() > 0 {
            stream.write(message.as_ref()).await;
        }
        let commands = create_commands(&stream).await;
        let (msg, commands) = create_log_msg(log_std, stream.peer_addr().expect("Cannot get peer address"), commands);
        if log_std {
            println!("{}", msg.clone());
        }

        if commands.len() > 0 {
            if storage_string(commands.get(0).unwrap()).to_uppercase() == "MONITOR" {
                monitor = true;
            }
            let mut data_map = &mut*data_arc.lock().await;
            match process_command_transaction(commands, &mut data_map) {
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
                            // monitoring_threads.push(thread_uuid.clone());
                            message = encode(&Value::String("MONITOR".to_string()));
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

            if monitor {
                break;
            }
        }
        else {
            println!("{}", "thread killed");
            break;
        }
    }
    // let mut monitor = false;
    //
    // let mut message = vec![];
    //
    // loop {
    //     let mut stream_clone = stream.try_clone().unwrap();
    //     if message.len() > 0 {
    //         stream_clone.write(message.as_ref());
    //     }
    //
    //     let commands = create_commands(&stream_clone);
    //
    //     if commands.len() > 0 {
    //         if storage_string(commands.get(0).unwrap()).to_uppercase() == "MONITOR" {
    //             monitor = true;
    //         }
    //         tx.send((commands, stream_clone.peer_addr().unwrap(), my_uuid.clone())).expect("unable to send command");
    //         message = mprx.recv().unwrap();
    //         if monitor {
    //             break;
    //         }
    //     } else {
    //         println!("{}", "thread killed");
    //         break;
    //     }
    // }
    //
    // if monitor {
    //     loop {
    //         let msg = mprx.recv().unwrap();
    //         match stream.write(msg.as_ref()) {
    //             Ok(sent) => {
    //                 println!("wrote {}", sent);
    //             }
    //             Err(_) => {
    //                 println!("{}", "closing connection");
    //                 break;
    //             }
    //         }
    //     }
    // }
}