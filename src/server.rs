use std::net::{TcpStream};
use std::thread::JoinHandle;
use std::{thread};
use std::sync::{Arc, Mutex, mpsc};
use std::collections::HashMap;
use mpsc::{Sender as MSender, Receiver as MReceiver};
extern crate resp;
use crate::thread_loop::thread_loop;

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
    sender: MSender<String>,
    data_map_mutex: Arc<Mutex<HashMap<String, Storage>>>,
    connections_mutex: Arc<Mutex<Vec<MSender<String>>>>
}

impl Server {

    pub(crate) fn new(log_std: bool) -> Server {
        let (mptx, mprx) = mpsc::channel::<String>();
        let data_map: HashMap<String, Storage> = HashMap::new();
        let data_map_mutex = Arc::new(Mutex::new(data_map));

        let connections = Vec::default();
        let connections_mutex = Arc::new(Mutex::new(connections));

        let mut server = Server {
            thread_join_handles: Default::default(),
            sender: mptx,
            data_map_mutex,
            connections_mutex
        };
        server.start_rx_loop(log_std,  mprx);
        return server;
    }

    fn start_rx_loop(&mut self, log_std: bool, rx: MReceiver<String>) {
        let connections_mutex = Arc::clone(&self.connections_mutex);

        thread::spawn(move || {
            loop {
                let msg = rx.recv().unwrap();
                if log_std {
                    println!("{}", msg);
                }
                let connections_list =  &mut*connections_mutex.lock().unwrap();
                let mut remove = vec![];
                for i in 0..connections_list.len() {
                    match connections_list.get(i).unwrap().send(msg.clone()) {
                        Ok(_) => {}
                        Err(_) => {
                            remove.push(i);
                        }
                    }
                }
                for i in remove {
                    connections_list.remove(i);
                }
            }
        });
    }

    pub(crate) fn add_connection(&mut self, stream: TcpStream) {
        let ip = stream.local_addr().unwrap().ip();
        println!("adding connection {}", ip);

        let (mptx, mprx) = mpsc::channel::<String>();
        self.connections_mutex.lock().unwrap().push(mptx);

        let tx = self.sender.clone();
        let data_map_mutex = Arc::clone(&self.data_map_mutex);
        self.thread_join_handles.push(thread::spawn(move || thread_loop(stream, data_map_mutex, tx, mprx)));
    }

}