use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use crate::server::Storage;
use crate::command::command_response::{CommandResponse, CommandError};
use resp::Value;


pub(crate) fn command_get(
    mut commands: Vec<Storage>,
    mut data_map: &mut HashMap<String, Storage>
) -> Result<CommandResponse, CommandError> {
    let key = storage_string!(commands.remove(0));
    if data_map.contains_key(&key) {
        match data_map.get(&key).expect("key not found") {
            Storage::String { value, created, expire } => {
                let return_value = value.clone();
                if is_expired!(created, expire) {
                    data_map.remove(&key).expect("Unable to remove key");
                }
                Ok(CommandResponse::Get {
                    response: Value::String (return_value)
                })
            }
            Storage::Bytes { value, created, expire } => {
                let return_value = value.clone();
                if is_expired!(created, expire) {
                    data_map.remove(&key).expect("Unable to remove key");
                }
                Ok(CommandResponse::Get {
                    response: Value::BufBulk (return_value)
                })
            }
            _ => {
                Err(CommandError::Null)
            }
        }
    } else {
        Err(CommandError::Null)
    }
}