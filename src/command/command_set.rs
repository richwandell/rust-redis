use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use crate::server::Storage;
use crate::command::command_response::{CommandResponse, CommandError};
use crate::command::expire_storage;

const EX: &str = "EX";
const PX: &str = "PX";
const EXAT: &str = "EXAT";
const PXAT: &str = "PXAT";
const NX: &str = "NX";
const XX: &str = "XX";
const KEEPTTL: &str = "KEEPTTL";
const GET: &str = "GET";

pub(crate) fn command_set(
    mut commands: Vec<Storage>,
    mut data_map: &mut HashMap<String, Storage>
) -> Result<CommandResponse, CommandError> {
    let key = storage_string!(commands.remove(0));
    let mut val = commands.remove(0);

    while commands.len() > 0 {
        let next = storage_string!(commands.remove(0));
        if next.to_uppercase() == EX {
            if commands.len() == 0 {
                return Err(CommandError::Error {text: "(error) ERR syntax error".to_string()})
            }
            let seconds = storage_string!(commands.remove(0));
            if let Ok(ttl) = seconds.parse::<f64>() {
                val = expire_storage(val, ttl);
            } else {
                return Err(CommandError::Error {text: "(error) ERR value is not an integer or out of range".to_string()})
            }
        }
    }

    data_map.insert(key, val);
    Ok(CommandResponse::Set)
}