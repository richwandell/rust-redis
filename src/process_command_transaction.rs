use crate::commands::{COMMAND_SET, COMMAND_GET, COMMAND_PING, COMMAND_COMMAND, COMMAND_QUIT};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use crate::server::Storage;

pub(crate) enum CommandError {
    Error {
        text: String
    },
    Null
}

pub(crate) enum CommandResponse {
    Ping,
    Set,
    Get {
        response: String
    },
    Cmd,
    Quit
}

pub(crate) fn process_command_transaction(
    commands: Vec<String>,
    data_map_mutex: &Arc<Mutex<HashMap<String, Storage>>>
) -> Result<CommandResponse, CommandError> {
    let data_map = &mut*data_map_mutex.lock().unwrap();
    let command = commands[0].to_uppercase();
    if command == COMMAND_QUIT {
        return Ok(CommandResponse::Quit)
    } else if command == COMMAND_COMMAND {
        return Ok(CommandResponse::Cmd)
    } else if command == COMMAND_PING {
        return Ok(CommandResponse::Ping)
    } else if command == COMMAND_SET {
        let key = commands[1].to_string();
        let val = commands[2].to_string();
        data_map.insert(key, Storage::String {value: val});
        return Ok(CommandResponse::Set)
    } else if command == COMMAND_GET {
        let key = commands[1].to_string();
        if data_map.contains_key(&key) {
            match data_map.get(&key).expect("key not found") {
                Storage::String { value } => {
                    return Ok(CommandResponse::Get {
                        response: value.clone()
                    })
                }
                _ => {
                    return Err(CommandError::Null)
                }
            }
        } else {
            return Err(CommandError::Null)
        }
    }
    return Err(CommandError::Error {
        text: format!("-ERR unknown command `{}`, with args beginning with: {}", commands[0], if commands.len() > 1 {
            commands[1].to_string()
        } else {
            "".to_string()
        })
    })
}