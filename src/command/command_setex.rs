use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use crate::server::Storage;
use crate::command::command_response::{CommandResponse, CommandError};
use crate::command::expire_storage;


pub(crate) fn command_setex(
    mut commands: Vec<Storage>,
    data_map_mutex: &Arc<Mutex<HashMap<String, Storage>>>
) -> Result<CommandResponse, CommandError> {
    let data_map = &mut*data_map_mutex.lock().unwrap();
    let key = storage_string!(commands.remove(0));
    let ttl = storage_float!(commands.remove(0));
    let val = expire_storage(commands.remove(0), ttl);
    data_map.insert(key, val);
    Ok(CommandResponse::SetEx)
}