use std::{ net::TcpListener, thread::spawn };

use rand::prelude::*;

use tungstenite::Message;
use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize)]
struct Resp {
    direction: String,
    rate: i16,
    text: char,
}

impl Resp {
    // Custom constructor to create a default Resp instance
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let rand_tuple: (bool, i16, u8) = rng.gen::<(bool, i16, u8)>();
        let dir: &str = if rand_tuple.0 { "in" } else { "out" };
        Self {
            direction: dir.to_owned(),
            rate: rand_tuple.1.to_owned(),
            text: rand_tuple.2.to_owned() as char,
        }
    }
}

fn build_response() -> Result<String, serde_json::Error> {
    let r: Resp = Resp::new();
    return serde_json::to_string(&r);
}

fn process_query() -> Message {
    let text = build_response().unwrap();
    return Message::Text(text.to_owned());
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <ip> <Port>", args[0]);
        std::process::exit(1);
    }

    let ip = &args[1];
    let port = &args[2];

    let bind_addr = format!("{}:{}", ip, port);
    let server = TcpListener::bind(bind_addr.to_owned()).unwrap();
    eprintln!("Listening on: ws://{bind_addr}");
    for stream in server.incoming() {
        spawn(move || {
            let mut websocket = tungstenite::accept(stream.unwrap()).unwrap();
            eprintln!("New client connected");
            loop {
                let msg_result: Result<Message, tungstenite::Error> = websocket.read();
                match msg_result {
                    Ok(msg) => {
                        if msg.is_binary() || msg.is_text() {
                            websocket.send(process_query()).unwrap();
                        }
                    }
                    Err(_) => {
                        eprintln!("Closing the connection");
                        break; //Finishes the thread
                    }
                }
            }
        });
    }
}
