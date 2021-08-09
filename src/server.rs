use std::net::{TcpStream};
use std::thread::JoinHandle;
use std::{thread};
use spmc::{Sender, Receiver};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
extern crate resp;
use crate::thread_loop::thread_loop;

pub(crate) enum Storage {
    String {
        value: String
    },
    #[allow(dead_code)]
    List {
        value: Vec<String>
    },
    #[allow(dead_code)]
    Set {
        value: HashMap<String, String>
    }
}

pub(crate) struct Server {
    #[allow(dead_code)]
    pub(crate) connections: Vec<TcpStream>,
    pub(crate) thread_join_handles: Vec<JoinHandle<()>>,
    #[allow(dead_code)]
    sender: Sender<()>,
    #[allow(dead_code)]
    receiver: Receiver<()>,
    data_map_mutex: Arc<Mutex<HashMap<String, Storage>>>
}



impl Server {

    pub(crate) fn new() -> Server {
        let (sptx, sprx) = spmc::channel();
        let data_map: HashMap<String, Storage> = HashMap::new();
        let data_map_mutex = Arc::new(Mutex::new(data_map));

        Server {
            connections: Default::default(),
            thread_join_handles: Default::default(),
            sender: sptx,
            receiver: sprx,
            data_map_mutex
        }
    }

    pub(crate) fn add_connection(&mut self, stream: TcpStream) {
        let ip = stream.local_addr().unwrap().ip();
        println!("adding connection {}", ip);
        let rx = self.receiver.clone();
        let data_map_mutex = Arc::clone(&self.data_map_mutex);
        self.thread_join_handles.push(thread::spawn(move || thread_loop(stream, data_map_mutex, rx)));
    }

}