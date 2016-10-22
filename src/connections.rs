use std::thread;
use std::borrow::Cow;
use std::string::String;
use websocket::message::Type;
use std::sync::{Arc, RwLock, mpsc};
use websocket::header::WebSocketProtocol;
use std::sync::atomic::{Ordering, AtomicBool};
use websocket::{Server, Message, Sender, Receiver};

use input::Input;
use world::World;

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

            if let Some(&WebSocketProtocol(ref protocols)) = headers.get() {
                if protocols.contains(&("rust-websocket".to_string())) {
                    // We have a protocol we want to use
                    response.headers.set(WebSocketProtocol(vec!["rust-websocket".to_string()]));
                }
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

            for message in receiver.incoming_messages() {
                let message: Message = message.unwrap();

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
                        println!("{}", &*to_cow_str(&message));
                        if let Some(input) = Input::from_str(&*to_cow_str(&message)) {
                            println!("its a valid input: {:?}", input);
                            input_tx.send(input);
                            continue
                        }
                        if World::is_world_request(&*to_cow_str(&message)) {
                            println!("its a world request!");

                            let world = if curr_world_is_1.load(Ordering::Relaxed) {
                                world1.clone()
                            } else {
                                world2.clone()
                            };

                            let world = world.read().unwrap();

                            sender.send_message(&Message::text(&(*world.to_json()))).unwrap();
                            continue
                        }
                        sender.send_message(&message).unwrap()
                    },
                }
            }
        });
    }
}
