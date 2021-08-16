use crate::server::Storage;
use std::time::{SystemTime, UNIX_EPOCH};

#[macro_use]
macro_rules! storage_string {
    ($e:expr) => {{
        match $e {
            Storage::Bytes { .. } => "".to_string(),
            Storage::String { value, created:_, expire:_ } => value,
            Storage::List { .. } => "".to_string(),
            Storage::Set { .. } => "".to_string()
        }
    }}
}

#[macro_use]
macro_rules! storage_float {
    ($e:expr) => {{
        match $e {
            Storage::Bytes { .. } => -1.0,
            Storage::String { value, created:_, expire:_ } => {
                value.parse::<f64>().unwrap()
            },
            Storage::List { .. } => -1.0,
            Storage::Set { .. } => -1.0
        }
    }}
}

#[macro_use]
macro_rules! is_expired {
    ($created:expr, $expire:expr) => {{
        let time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        $expire > &-1.0 && $created + $expire < time
    }}
}

pub(crate) mod process_command_transaction;
pub(crate) mod command_response;
mod command_get;
mod command_set;
mod command_del;
mod command_keys;
mod command_mset;
mod command_mget;
mod command_getdel;
mod command_getset;
mod commands;
mod command_setex;

pub(crate) fn expire_storage(item: Storage, ttl: f64) -> Storage {
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs_f64();

    match item {
        Storage::String { value, created: _, expire: _ } => {
            Storage::String {
                value,
                created: time,
                expire: ttl,
            }
        }
        Storage::Bytes { value, created:_, expire:_ } => {
            Storage::Bytes {
                value,
                created: time,
                expire: ttl
            }
        }
        _ => item
    }
}
