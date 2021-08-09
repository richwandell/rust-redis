use resp::{Value, encode};

pub(crate) fn create_command_respons() -> String {
    let values = Value::Array(vec![
        Value::Array(vec![
            Value::String("get".to_string()),
            Value::Integer(2),
            Value::Array(vec![
                Value::String("readonly".to_string())
            ]),
            Value::Integer(1),
            Value::Integer(1),
            Value::Integer(1)
        ]),
        Value::Array(vec![
            Value::String("set".to_string()),
            Value::Integer(-3),
            Value::Array(vec![
                Value::String("write".to_string()),
                Value::String("denyoom".to_string())
            ]),
            Value::Integer(1),
            Value::Integer(1),
            Value::Integer(1),
            Value::Array(vec![
                Value::String("@write".to_string()),
                Value::String("@string".to_string()),
                Value::String("@slow".to_string())
            ])
        ])
    ]);

    std::str::from_utf8(&encode(&values)).unwrap().parse().unwrap()
}