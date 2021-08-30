use std::collections::HashMap;
use std::io::Write;
use std::net::{SocketAddr, TcpStream};
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use std::time::{SystemTime, UNIX_EPOCH};

use resp::{encode, Value};

use crate::command::command_response::{CommandError, CommandResponse};
use crate::command::process_command_transaction::process_command_transaction;
use crate::create_command_response::create_command_respons;
use crate::create_commands::{create_commands, create_commands_new};
use crate::server::Storage;
use crate::command::commands::RedisCommand;

pub(crate) fn thread_loop(
    mut stream: TcpStream,
    tx: Sender<(Vec<Storage>, SocketAddr, String)>,
    mprx: Receiver<Vec<u8>>,
    my_uuid: String,
) {
    let mut monitor = false;

    let mut message = vec![];

    loop {
        if message.len() > 0 {
            stream.write(message.as_ref());
        }

        let commands = create_commands_new(&stream);

        if commands.len() > 0 {
            match commands.get(0).unwrap() {
                Storage::Command { value } => {
                    match value {
                        RedisCommand::Monitor => {
                            monitor = true;
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
            tx.send((commands, stream.peer_addr().unwrap(), my_uuid.clone())).expect("unable to send command");
            message = mprx.recv().unwrap();
            if monitor {
                break;
            }
        } else {
            println!("{}", "thread killed");
            break;
        }
    }

    if monitor {
        loop {
            let msg = mprx.recv().unwrap();
            match stream.write(msg.as_ref()) {
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