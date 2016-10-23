use std::thread;
use std::time;
use std::borrow::Cow;
use std::string::String;
use websocket::message::Type;
use std::sync::{Arc, RwLock, mpsc};
use websocket::header::WebSocketProtocol;
use std::sync::atomic::{Ordering, AtomicBool};
use websocket::{Server, Message, Sender, Receiver};

use input::Input;
use world::World;

#[derive(Debug, Clone, Copy)]
enum Protocol {
    Input,
    World,
}

fn to_cow_str<'s>(msg: &'s Message<'s>) -> Cow<'s, str> {
    String::from_utf8_lossy(&*msg.payload)
}

pub fn listen_to_incomming_connections(input_tx: mpsc::Sender<Input>,
                                       curr_world_is_1: Arc<AtomicBool>,
                                       world1: Arc<RwLock<World>>, world2: Arc<RwLock<World>>) {
    println!("listening to incomming connections");
    let server = Server::bind("127.0.0.1:2794").unwrap();

    for connection in server {
        let input_tx = input_tx.clone();
        let curr_world_is_1 = curr_world_is_1.clone();
        let world1 = world1.clone();
        let world2 = world2.clone();
        // Spawn a new thread for each connection.
        thread::spawn(move || {
            let request = connection.unwrap().read_request().unwrap(); // Get the request
            let headers = request.headers.clone(); // Keep the headers so we can check them

            request.validate().unwrap(); // Validate the request

            let mut response = request.accept(); // Form a response

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

                let mut client = response.send().unwrap(); // Send the response

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
                let mut message_count = 0;

                match protocol {
                    Protocol::Input => {
                        for message in receiver.incoming_messages() {
                            let message: Message = message.unwrap();

                            message_count += 1;
                            if message_count % 100 == 0 {
                                println!("rate: {:?}", message_count as f64 / elapsed_ms() * 1000.0);
                            }

                            match message.opcode {
                                Type::Close => {
                                    let message = Message::close();
                                    sender.send_message(&message).unwrap();
                                    println!("Client {} disconnected", ip);
                                    return;
                                },
                                Type::Ping => {
                                    let message = Message::pong(message.payload);
                                    sender.send_message(&message).unwrap();
                                },
                                _ => {
                                    // get the message text
                                    //println!("{}", &*to_cow_str(&message));
                                    if let Some(input) = Input::from_str(&*to_cow_str(&message)) {
                                        println!("its a valid input: {:?}", input);
                                        input_tx.send(input).unwrap();
                                        continue
                                    }
                                },
                            }
                        }
                    },
                    Protocol::World => {
                        loop {
                            let world = if curr_world_is_1.load(Ordering::Relaxed) {
                                world2.clone()
                            } else {
                                world1.clone()
                            };

                            let mut msg = "".into();
                            {
                                let read_world = world.read().unwrap();
                                msg = String::from(&(*read_world.to_json()));
                            }
                            sender.send_message(&Message::text(msg.as_str())).unwrap();
                        }
                    }
                }
            }
        });
    }
}
