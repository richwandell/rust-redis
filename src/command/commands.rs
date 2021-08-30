pub(crate) const COMMAND_QUIT: &str = "QUIT";
pub(crate) const COMMAND_SET: &str = "SET";
pub(crate) const COMMAND_GET: &str = "GET";
pub(crate) const COMMAND_PING: &str = "PING";
pub(crate) const COMMAND_COMMAND: &str = "COMMAND";
pub(crate) const COMMAND_DEL: &str = "DEL";
pub(crate) const COMMAND_KEYS: &str = "KEYS";
pub(crate) const COMMAND_MSET: &str = "MSET";
pub(crate) const COMMAND_MGET: &str = "MGET";
pub(crate) const COMMAND_GETDEL: &str = "GETDEL";
pub(crate) const COMMAND_GETSET: &str = "GETSET";
pub(crate) const COMMAND_MONITOR: &str = "MONITOR";
pub(crate) const COMMAND_SETEX: &str = "SETEX";

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum RedisCommand {
    Quit,
    Set,
    Get,
    Ping,
    Command,
    Del,
    Keys,
    Mset,
    Mget,
    GetDel,
    GetSet,
    Monitor,
    SetEx
}