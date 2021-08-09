use crate::commands::{COMMAND_SET, COMMAND_GET, COMMAND_PING, COMMAND_COMMAND, COMMAND_QUIT, COMMAND_DEL, COMMAND_KEYS, COMMAND_MSET, COMMAND_MGET};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use crate::server::Storage;
use resp::Value;
use glob::Pattern;

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
    Quit,
    Del {
        removed: i64
    },
    Keys {
        keys: Value
    },
    Mset,
    Mget {
        value: Value
    }
}

fn item_to_value(item: &Storage) -> Value {
    match item {
        Storage::String { value } => Value::String(value.clone()),
        Storage::List { value } => Value::Array(value.iter().map(|x| Value::String(x.clone())).collect::<Vec<Value>>()),
        _ => Value::Null
    }
}

pub(crate) fn process_command_transaction(
    commands: Vec<String>,
    data_map_mutex: &Arc<Mutex<HashMap<String, Storage>>>
) -> Result<CommandResponse, CommandError> {
    let data_map = &mut*data_map_mutex.lock().unwrap();
    let command = commands[0].to_uppercase();
    if command == COMMAND_MGET {
        let mut results = vec![];
        let mut i = 0;
        for key in commands {
            if i > 0 {
                if data_map.contains_key(&key) {
                    if let Some(item) = data_map.get(&key).clone() {
                        results.push(item_to_value(item));
                    }
                } else {
                    results.push(Value::Null);
                }
            }
            i += 1;
        }
        return Ok(CommandResponse::Mget {
            value: Value::Array(results)
        })
    } else if command == COMMAND_MSET {
        let mut i = 0;
        let mut on_key = true;
        let mut last_key = "".to_string();
        for key_or_value in commands {
            if i > 0 {
                if !on_key {
                    data_map.insert(last_key.clone(), Storage::String {value: key_or_value.clone()});
                }
                on_key = !on_key;
                last_key = key_or_value;
            }
            i += 1;
        }
        return Ok(CommandResponse::Mset)
    } else if command == COMMAND_KEYS {
        let mut matched_keys = vec![];
        let pattern = Pattern::new(&commands[1]).unwrap();
        for key in data_map.keys() {
            if pattern.matches(key) {
                matched_keys.push(Value::String(key.clone()));
            }
        }
        return Ok(CommandResponse::Keys {
            keys: Value::Array(matched_keys)
        })
    } else if command == COMMAND_DEL {
        let mut i = 0;
        let mut removed = 0;
        for key in commands {
            if i > 0 {
                if let Some(_) = data_map.remove(&key) {
                    removed += 1;
                }
            }
            i += 1;
        }
        return Ok(CommandResponse::Del {removed})
    } else if command == COMMAND_QUIT {
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