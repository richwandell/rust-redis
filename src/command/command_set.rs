use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use crate::server::Storage;
use crate::command::command_response::{CommandResponse, CommandError};

pub(crate) fn command_set(
    mut commands: Vec<Storage>,
    data_map_mutex: &Arc<Mutex<HashMap<String, Storage>>>
) -> Result<CommandResponse, CommandError> {
    let data_map = &mut*data_map_mutex.lock().unwrap();
    let key = storage_string!(commands.remove(0));
    let val = commands.remove(0);
    data_map.insert(key, val);
    Ok(CommandResponse::Set)
}