use resp::Value;

pub(crate) enum CommandResponse {
    Ping,
    Set,
    Get {
        response: Value
    },
    GetDel {
        response: String
    },
    GetSet {
        response: String
    },
    Cmd,
    Quit,
    Del {
        removed: i64
    },
    Keys {
        keys: Value
    },
    Mset,
    Mget {
        value: Value
    },
    Monitor,
    SetEx
}

pub(crate) enum CommandError {
    Error {
        text: String
    },
    Null
}