use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use crate::server::Storage;
use resp::Value;
use crate::command::command_response::{CommandResponse, CommandError};

pub(crate) fn command_mget(
    commands: Vec<Storage>,
    mut data_map: &mut HashMap<String, Storage>
) -> Result<CommandResponse, CommandError> {
    let mut results = vec![];
    for key in commands {
        let key = storage_string!(key);
        if data_map.contains_key(&key) {
            if let Some(item) = data_map.get(&key).clone() {
                match item {
                    Storage::String { value, created, expire } => {
                        if is_expired!(created, expire) {
                            data_map.remove(&key).expect("Unable to remove key");
                            results.push(Value::Null);
                        } else {
                            results.push(Value::String(value.clone()));
                        }
                    },
                    Storage::List { value, created, expire } => {
                        if is_expired!(created, expire) {
                            data_map.remove(&key).expect("Unable to remove key");
                            results.push(Value::Null);
                        } else {
                            results.push(Value::Array(value.iter().map(|x| Value::String(x.clone())).collect::<Vec<Value>>()))
                        }
                    },
                    _ => results.push(Value::Null)
                }
            }
        } else {
            results.push(Value::Null);
        }
    }
    Ok(CommandResponse::Mget {
        value: Value::Array(results)
    })
}