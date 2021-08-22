use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use crate::server::Storage;
use resp::Value;
use glob::Pattern;
use crate::command::command_response::{CommandResponse, CommandError};

pub(crate) fn command_keys(
    mut commands: Vec<Storage>,
    mut data_map: &HashMap<String, Storage>
) -> Result<CommandResponse, CommandError> {
    if commands.len() == 0 {
        return Err(CommandError::Error {
            text: "(error) ERR wrong number of arguments for 'keys' command".to_string()
        });
    }
    let mut matched_keys = vec![];
    let pattern_string = storage_string!(commands.remove(0));
    let pattern = Pattern::new(&pattern_string).unwrap();
    for key in data_map.keys() {
        if pattern.matches(key) {
            matched_keys.push(Value::String(key.clone()));
        }
    }
    Ok(CommandResponse::Keys {
        keys: Value::Array(matched_keys)
    })
}