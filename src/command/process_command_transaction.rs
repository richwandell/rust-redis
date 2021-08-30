
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use crate::server::Storage;
use crate::command::command_response::{CommandResponse, CommandError};
use crate::command::commands::*;
use crate::command::command_getset::command_getset;
use crate::command::command_getdel::command_getdel;
use crate::command::command_mget::command_mget;
use crate::command::command_mset::command_mset;
use crate::command::command_keys::command_keys;
use crate::command::command_del::command_del;
use crate::command::command_set::command_set;
use crate::command::command_get::command_get;
use crate::command::command_setex::command_setex;

pub(crate) fn process_command_transaction(
    mut commands: Vec<Storage>,
    mut data_map: &mut HashMap<Vec<u8>, Storage>
) -> Result<CommandResponse, CommandError> {
    let command = commands.remove(0);

    match command {
        Storage::Command { value } => {
            match value {
                RedisCommand::Get => {
                    command_get(commands, data_map)
                }
                RedisCommand::Set => {
                    command_set(commands, data_map)
                },
                RedisCommand::Command => {
                    Ok(CommandResponse::Cmd)
                },
                _ => Err(CommandError::Error {
                    text: "".to_string()
                })
            }
        },
        _ => Err(CommandError::Error {
            text: format!("-ERR unknown command `{}`, with args beginning with: {}", storage_string!(command), if commands.len() > 0 {
                storage_string!(commands.remove(0))
            } else {
                "".to_string()
            })
        })
    }

    // if command == COMMAND_SETEX {
    //     return command_setex(commands, data_map);
    // }
    // else if command == COMMAND_MONITOR {
    //     return Ok(CommandResponse::Monitor)
    // }
    // else if command == COMMAND_GETSET {
    //     return command_getset(commands, data_map);
    // }
    // else if command == COMMAND_GETDEL {
    //     return command_getdel(commands, data_map);
    // }
    // else if command == COMMAND_MGET {
    //     return command_mget(commands, data_map);
    // }
    // else if command == COMMAND_MSET {
    //     return command_mset(commands, data_map);
    // }
    // else if command == COMMAND_KEYS {
    //     return command_keys(commands, data_map);
    // }
    // else if command == COMMAND_DEL {
    //     return command_del(commands, data_map);
    // }
    // else if command == COMMAND_QUIT {
    //     return Ok(CommandResponse::Quit)
    // }
    // else if command == COMMAND_COMMAND {
    //     return Ok(CommandResponse::Cmd)
    // }
    // else if command == COMMAND_PING {
    //     return Ok(CommandResponse::Ping)
    // }
    // else if command == COMMAND_SET {
    //     return command_set(commands, data_map);
    // }
    // else if command == COMMAND_GET {
    //     return command_get(commands, data_map);
    // }
    // return Err(CommandError::Error {
    //     text: format!("-ERR unknown command `{}`, with args beginning with: {}", command, if commands.len() > 0 {
    //         storage_string!(commands.remove(0))
    //     } else {
    //         "".to_string()
    //     })
    // })
}