use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use crate::server::Storage;
use crate::command::command_response::{CommandResponse, CommandError};

pub(crate) fn command_del(
    commands: Vec<Storage>,
    mut data_map: &mut HashMap<String, Storage>
) -> Result<CommandResponse, CommandError> {
    let mut removed = 0;
    for key in commands {
        let key = storage_string!(key);
        if let Some(_) = data_map.remove(&key) {
            removed += 1;
        }
    }
    Ok(CommandResponse::Del {removed})
}