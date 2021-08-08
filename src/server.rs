use std::net::{TcpStream};
use std::io::{Write, Read, BufReader, BufRead};
use std::thread::JoinHandle;
use std::{thread, io};
use spmc::{Sender, Receiver};
use core::time;
use std::str;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::collections::hash_map::RandomState;
use crate::commands::COMMAND_SET;

pub(crate) enum Storage {
    String {
        location: usize
    },
    Number {
        location: usize
    }
}

pub(crate) struct DataMutex {
    data_map_mutex: Arc<Mutex<HashMap<String, Storage>>>,
    str_data_mutex: Arc<Mutex<Vec<String>>>
}

pub(crate) struct Server {
    pub(crate) connections: Vec<TcpStream>,
    pub(crate) thread_join_handles: Vec<JoinHandle<()>>,
    #[allow(dead_code)]
    sender: Sender<()>,
    #[allow(dead_code)]
    receiver: Receiver<()>,
    data_map_mutex: Arc<Mutex<HashMap<String, Storage>>>,
    str_data_mutex: Arc<Mutex<Vec<String>>>,
}

impl Server {

    pub(crate) fn new() -> Server {
        let (mut sptx, sprx) = spmc::channel();
        let mut data_map: HashMap<String, Storage> = HashMap::new();
        let data_map_mutex = Arc::new(Mutex::new(data_map));
        let mut str_data: Vec<String> = Vec::new();
        let str_data_mutex = Arc::new(Mutex::new(str_data));

        Server {
            connections: Default::default(),
            thread_join_handles: Default::default(),
            sender: sptx,
            receiver: sprx,
            data_map_mutex,
            str_data_mutex
        }
    }

    pub fn thread_loop(mut stream: TcpStream, mut data_mutex: DataMutex, rx: Receiver<()>) {

        fn process_command_transaction(commands: Vec<&str>, mut data_mutex: &DataMutex) {
            let data_map = &mut*data_mutex.data_map_mutex.lock().unwrap();
            if commands[0] == COMMAND_SET {
                if data_map.contains_key(commands[1]) {
                    match data_map.get(commands[1]).expect("key not found") {
                        Storage::String { location } => {
                            let mut str_data = &mut*data_mutex.str_data_mutex.lock().unwrap();
                            str_data[*location] = commands[2].to_string();
                            println!("{:?}", str_data);
                        }
                        Storage::Number { location } => {

                        }
                    }
                } else {
                    let mut str_data = &mut*data_mutex.str_data_mutex.lock().unwrap();
                    data_map.insert(commands[1].to_string(), Storage::String {location: str_data.len()});
                    str_data.push(commands[2].to_string());
                    println!("{:?}", str_data);
                }
            }
        }

        let mut reader = BufReader::new(stream);
        loop {
            let mut buffer = String::new();

            let recieved_data_size = reader.read_line(&mut buffer)
                .expect("Error reading line");

            if recieved_data_size > 0 {
                let commands = buffer.split(" ").collect::<Vec<&str>>();
                if commands.len() > 0{
                    process_command_transaction(commands, &data_mutex);
                }
                println!("{}", buffer);
            } else {
                break;
            }
        }
    }

    pub(crate) fn add_connection(&mut self, mut stream: TcpStream) {
        let ip = stream.local_addr().unwrap().ip();
        println!("adding connection {}", ip);
        if let Ok(_) = stream.write("thing happened".as_ref()) {
            let thread_data_mutex = DataMutex {
                data_map_mutex: Arc::clone(&self.data_map_mutex),
                str_data_mutex: Arc::clone(&self.str_data_mutex)
            };
            let rx = self.receiver.clone();
            self.thread_join_handles.push(thread::spawn(move || Server::thread_loop(stream, thread_data_mutex, rx)));
        }
    }

}