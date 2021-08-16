use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use crate::server::Storage;
use crate::command::command_response::{CommandResponse, CommandError};

pub(crate) fn command_getdel(
    mut commands: Vec<Storage>,
    data_map_mutex: &Arc<Mutex<HashMap<String, Storage>>>
) -> Result<CommandResponse, CommandError>{
    let data_map = &mut*data_map_mutex.lock().unwrap();
    let key = storage_string!(commands.remove(0));
    if data_map.contains_key(&key) {
        match data_map.remove(&key).expect("key not found") {
            Storage::String { value, created, expire } => {
                if is_expired!(&created, &expire) {
                    Err(CommandError::Null)
                } else {
                    Ok(CommandResponse::GetDel {
                        response: value
                    })
                }
            }
            _ => {
                Err(CommandError::Null)
            }
        }
    } else {
        Err(CommandError::Null)
    }
}