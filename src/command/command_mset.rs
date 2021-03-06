use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use crate::server::Storage;
use crate::command::command_response::{CommandResponse, CommandError};

pub(crate) fn command_mset(
    commands: Vec<Storage>,
    data_map_mutex: &Arc<Mutex<HashMap<String, Storage>>>
) -> Result<CommandResponse, CommandError> {
    let data_map = &mut*data_map_mutex.lock().unwrap();
    let mut on_key = true;
    let mut last_key = "".to_string();
    for key_or_value in commands {
        if on_key {
            last_key = storage_string!(key_or_value);
        }
        else {
            data_map.insert(last_key.clone(), key_or_value);
        }
        on_key = !on_key;
    }
    Ok(CommandResponse::Mset)
}