use std::time;
use std::thread;
use std::borrow::Cow;
use std::string::String;
use std::sync::{Arc, mpsc};
use std::sync::atomic::{Ordering, AtomicBool};

use parking_lot::RwLock;

use websocket::message::Type;
use websocket::header::WebSocketProtocol;
use websocket::{Server, Message, Sender, Receiver};

use input::Input;
use sessionid::SessionID;
use world::{World, MsgType};

#[derive(Debug, Clone, Copy)]
enum Protocol {
    Input,
    World,
}

fn to_cow_str<'s>(msg: &'s Message<'s>) -> Cow<'s, str> {
    String::from_utf8_lossy(&*msg.payload)
}

pub fn listen_to_incomming_connections(input_tx: mpsc::Sender<(SessionID, Input)>,
                                       curr_world_is_1: Arc<AtomicBool>,
                                       world1: Arc<RwLock<World>>,
                                       world2: Arc<RwLock<World>>) {
    println!("listening to incomming connections");
    let server = Server::bind("127.0.0.1:2794").unwrap();

    for connection in server {
        let input_tx = input_tx.clone();
        let curr_world_is_1 = curr_world_is_1.clone();
        let world1 = world1.clone();
        let world2 = world2.clone();
        // Spawn a new thread for each connection.
        thread::spawn(move || {
            let request = connection.unwrap().read_request().unwrap();
            let headers = request.headers.clone();

            request.validate().unwrap();

            let mut response = request.accept();

            let protocol;

            if let Some(&WebSocketProtocol(ref protocols)) = headers.get() {
                if protocols.contains(&("input-websocket".to_string())) {
                    protocol = Protocol::Input;
                    response.headers.set(WebSocketProtocol(vec!["input-websocket".to_string()]));
                } else if protocols.contains(&("world-websocket".to_string())) {
                    protocol = Protocol::World;
                    response.headers.set(WebSocketProtocol(vec!["world-websocket".to_string()]));
                } else {
                    return;
                }

                let mut client = response.send().unwrap();

                let ip = client.get_mut_sender()
                    .get_mut()
                    .peer_addr()
                    .unwrap();


                println!("Connection from {}", ip);

                let message: Message = Message::text("Hello".to_string());
                client.send_message(&message).unwrap();

                let (mut sender, mut receiver) = client.split();


                let time = time::SystemTime::now();
                let elapsed_ms = || -> f64 {
                    time.elapsed().unwrap().as_secs() as f64 * 1000.0 +
                    time.elapsed().unwrap().subsec_nanos() as f64 / 1000000.0
                };

                // message to create a hero
                let session_id = SessionID::new();
                input_tx.send((session_id, Input::CreateHero));

                match protocol {
                    Protocol::Input => {
                        for message in receiver.incoming_messages() {
                            let message: Message = message.unwrap();

                            match message.opcode {
                                Type::Close => {
                                    let message = Message::close();
                                    sender.send_message(&message).unwrap();
                                    println!("Client {} disconnected", ip);
                                    return;
                                }
                                Type::Ping => {
                                    let message = Message::pong(message.payload);
                                    sender.send_message(&message).unwrap();
                                }
                                _ => {
                                    // get the message text
                                    if let Some(input) = Input::from_str(&*to_cow_str(&message)) {
                                        input_tx.send((session_id, input)).unwrap();
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                    Protocol::World => {
                        // sending first full message. Later we can just diff
                        let mut msg_type = MsgType::Full;
                        loop {
                            let world = if curr_world_is_1.load(Ordering::Relaxed) {
                                world2.clone()
                            } else {
                                world1.clone()
                            };

                            let msg;
                            {
                                let read_world = world.read();
                                msg = String::from(&(*read_world.as_frontend_msg(msg_type)));
                            }
                            match sender.send_message(&Message::text(msg.as_str())) {
                                Ok(_) => (),
                                Err(_) => return,
                            };
                            msg_type = MsgType::Diff;
                        }
                    }
                }
            }
        });
    }
}
