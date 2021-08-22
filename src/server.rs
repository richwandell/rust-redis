use std::net::{TcpStream, SocketAddr};
use std::thread::JoinHandle;
use std::{thread};
use std::sync::{Arc, Mutex, mpsc};
use std::collections::HashMap;
use mpsc::{Sender as MSender, Receiver as MReceiver};
extern crate resp;
use crate::thread_loop::thread_loop;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::command::process_command_transaction::process_command_transaction;
use crate::command::command_response::{CommandResponse, CommandError};
use resp::{Value, encode};
use crate::create_command_response::create_command_respons;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Storage {
    Bytes {
        value: Vec<u8>,
        created: f64,
        expire: f64
    },
    String {
        value: String,
        created: f64,
        expire: f64
    },
    #[allow(dead_code)]
    List {
        value: Vec<String>,
        created: f64,
        expire: f64
    },
    #[allow(dead_code)]
    Set {
        value: HashMap<String, String>,
        created: f64,
        expire: f64
    }
}

pub(crate) struct Server {
    thread_join_handles: Vec<JoinHandle<()>>,
    sender: MSender<(Vec<Storage>, SocketAddr, String)>,
    connections_mutex: Arc<Mutex<HashMap<String, MSender<Vec<u8>>>>>
}

fn create_log_msg(log_std: bool, addr: SocketAddr, commands: Vec<Storage>) -> (String, Vec<Storage>) {
    if !log_std {
        return ("".to_string(), commands)
    }
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs_f64();

    let ip = addr.ip();
    let port = addr.port();
    let mut command = "".to_string();
    for item in &commands {
        command += "\"";
        command += &match item {
            Storage::Bytes { value, created: _, expire: _ } => {
                let clone = value.clone();
                let str = String::from_utf8_lossy(&clone);
                str.to_string()
            }
            Storage::String { value, created: _, expire: _ } => value.clone(),
            Storage::List { .. } => "list".to_string(),
            Storage::Set { .. } => "set".to_string()
        };
        command += "\" ";
    }
    command.pop();
    (format!("{:.6} [{}:{}] {}", time, ip, port, command), commands)
}

impl Server {

    pub(crate) fn new(log_std: bool) -> Server {
        let (mptx, mprx) = mpsc::channel::<(Vec<Storage>, SocketAddr, String)>();

        let connections: HashMap<String, MSender<Vec<u8>>> = HashMap::new();
        let connections_mutex = Arc::new(Mutex::new(connections));

        let mut server = Server {
            thread_join_handles: Default::default(),
            sender: mptx,
            connections_mutex
        };
        server.start_rx_loop(log_std,  mprx);
        return server;
    }

    fn start_rx_loop(&mut self, log_std: bool, rx: MReceiver<(Vec<Storage>, SocketAddr, String)>) {
        let connections_mutex = Arc::clone(&self.connections_mutex);

        thread::spawn(move || {
            let mut data_map: HashMap<String, Storage> = HashMap::new();
            let mut monitoring_threads = vec![];
            loop {
                let (commands, socket_addr, thread_uuid) = rx.recv().unwrap();
                let (msg, commands) = create_log_msg(log_std || monitoring_threads.len() > 0, socket_addr, commands);
                if log_std {
                    println!("{}", msg.clone());
                }
                let mut message = vec![];

                match process_command_transaction(commands, &mut data_map) {
                    Ok(result) => {
                        match result {
                            CommandResponse::Set => {
                                message = encode(&Value::String("OK".to_string()));
                            }
                            CommandResponse::Get { response } => {
                                message = encode(&response);
                            }
                            CommandResponse::Ping => {
                                message = encode(&Value::String("PONG".to_string()));
                            }
                            CommandResponse::Cmd => {
                                message = create_command_respons();
                            }
                            CommandResponse::Quit => {
                                break;
                            }
                            CommandResponse::Del { removed } => {
                                message = encode(&Value::Integer(removed));
                            }
                            CommandResponse::Keys { keys } => {
                                message = encode(&keys);
                            }
                            CommandResponse::Mset => {
                                message = encode(&Value::String("OK".to_string()));
                            }
                            CommandResponse::Mget { value } => {
                                message = encode(&value);
                            }
                            CommandResponse::GetDel { response } => {
                                message = encode(&Value::Bulk(response));
                            }
                            CommandResponse::GetSet { response } => {
                                message = encode(&Value::Bulk(response));
                            }
                            CommandResponse::Monitor => {
                                monitoring_threads.push(thread_uuid.clone());
                                message = encode(&Value::String("MONITOR".to_string()));
                            }
                            CommandResponse::SetEx => {
                                message = encode(&Value::String("OK".to_string()));
                            }
                        }
                    }
                    Err(error) => {
                        match error {
                            CommandError::Error { text } => {
                                message = encode(&Value::String(text));
                            }
                            CommandError::Null => {
                                message = encode(&Value::Null);
                            }
                        }
                    }
                }

                for thread_id in &monitoring_threads {
                    let txmx = &*connections_mutex.lock().unwrap();
                    let m = encode(&Value::String(msg.clone()));
                    txmx.get(thread_id).unwrap().send(m);
                }

                let txmx = &*connections_mutex.lock().unwrap();
                txmx.get(&thread_uuid).unwrap().send(message);
            }
        });
    }

    pub(crate) fn add_connection(&mut self, stream: TcpStream) {
        let ip = stream.local_addr().unwrap().ip();
        println!("adding connection {}", ip);

        let (mptx, mprx) = mpsc::channel::<Vec<u8>>();
        let my_uuid = Uuid::new_v4().to_string();
        self.connections_mutex.lock().unwrap().insert(my_uuid.clone(), mptx);

        let tx = self.sender.clone();
        self.thread_join_handles.push(thread::spawn(move || thread_loop(stream, tx, mprx, my_uuid)));
    }

}