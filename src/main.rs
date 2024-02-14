
use std::{net::TcpListener, thread::spawn};


use rand::prelude::*;

use tungstenite::Message;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Resp {
    direction:String,
    rate:i16,
    text:char
}

fn build_response()-> Result<String, serde_json::Error>{
    let mut rng = rand::thread_rng();
    let rand_tuple: (bool, i16, u8) = rng.gen::<(bool,i16,u8)>();
    let dir = if rand_tuple.0 {
        "in"
    } else {
        "out"
    };
    let r: Resp = Resp{direction: dir.to_owned(),
        rate:rand_tuple.1.to_owned(),
        text:rand_tuple.2.to_owned() as char,
    };
    let j: Result<String, serde_json::Error> = serde_json::to_string(&r);
    return j;
}

fn process_query(msg:&Message)-> Message{
    match msg {
        Message::Text(_) => {
            let text = build_response().unwrap();
            return Message::Text(text.to_owned());
        }
        _ => {
            return Message::Text("No".to_owned());
        }
    }

}

fn main() {
    let bind_addr = "127.0.0.1:9001";
    let server = TcpListener::bind(bind_addr).unwrap();
    eprintln!("Listening on: ws://{bind_addr}");
    for stream in server.incoming() {
        spawn(move || {
            let mut websocket = tungstenite::accept(stream.unwrap()).unwrap();
            eprintln!("New client connected");
            loop {
                let msg = websocket.read().unwrap();
                if msg.is_binary() || msg.is_text() {
                    websocket.send(process_query(&msg)).unwrap();
                }
            }
        });
    }
}
