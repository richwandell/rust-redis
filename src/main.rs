mod server;
mod create_commands;
mod create_command_response;
mod connection_loop;
mod command;

// use std::net::{TcpListener, TcpStream};
use clap::{App, Arg};
use crate::server::{Server, Storage};
use async_std::io;
use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;
use std::io::Error;
use std::rc::Rc;
use std::cell::RefCell;
use async_std::{
    io::BufReader,
    task
};
use std::collections::HashMap;
use crate::create_commands::create_commands;
use crate::server::create_log_msg;
use std::collections::hash_map::RandomState;
use async_std::sync::{Arc, Mutex};
use crate::connection_loop::connection_loop;

#[async_std::main]
async fn main() -> std::io::Result<()> {
    let matches = App::new("Rust Redis")
        .version("0.1")
        .author("Rich Wandell <richwandell@gmail.com>")
        .about("Rust Redis")
        .arg(Arg::with_name("host")
            .help("Listen Host")
            .long("host")
            .short("h")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("port")
            .help("Port")
            .long("port")
            .short("p")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("log-std")
            .help("Log to std")
            .long("log-std")
            .short("lstd")
            .takes_value(false)
            .required(false))
        .get_matches();

    let port = matches.value_of("port").unwrap();
    let host = matches.value_of("host").unwrap();
    let log_std = matches.is_present("log-std");


    match TcpListener::bind(format!("{}:{}", host, port)).await {
        Ok(listener) => {
            let connections: HashMap<String, TcpStream> = HashMap::new();
            let data_map: HashMap<String, Storage> = HashMap::new();
            let data_arc = Arc::new(Mutex::new(data_map));

            let mut incoming = listener.incoming();
            while let Some(stream) = incoming.next().await {
                let mut stream = stream.expect("Error unwrapping stream");
                task::spawn(connection_loop(stream, data_arc.clone(), log_std));
            }
            Ok(())
        }
        Err(e) => {
            println!("error {}", e);
            Err(e)
        }
    }

    // match TcpListener::bind(format!("{}:{}", host, port)) {
    //     Ok(listener) => {
    //         let mut server = Server::new(log_std);
    //         fn handle_client(stream: TcpStream, server: &mut Server) {
    //             println!("{}", "thing happened");
    //             server.add_connection(stream);
    //         }
    //         for stream in listener.incoming() {
    //             handle_client(stream.unwrap(), &mut server);
    //         }
    //         Ok(())
    //     }
    //     Err(e) => {
    //         println!("error {}", e);
    //         Err(e)
    //     }
    // }
}
