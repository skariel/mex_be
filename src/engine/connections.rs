use std::thread;
use std::borrow::Cow;
use std::string::String;
use std::sync::{Arc, mpsc};
use std::sync::atomic::{Ordering, AtomicBool};

use parking_lot::RwLock;

use websocket::header::WebSocketProtocol;
use websocket::{Server, Message, Sender, Receiver};

use engine::sprites::Sprite;
use engine::sessionid::SessionID;
use engine::input::{Input, SessionInput};
use engine::world::{World, MsgType, Data};


fn to_cow_str<'s>(msg: &'s Message<'s>) -> Cow<'s, str> {
    String::from_utf8_lossy(&*msg.payload)
}

pub fn listen_to_incomming_connections<I: 'static + Input, D: 'static + Data, S: 'static + Sprite<I, D>>(input_tx: mpsc::Sender<SessionInput<I>>,
                                                                                                         curr_world_is_1: Arc<AtomicBool>,
                                                                                                         world1: Arc<RwLock<World<I, D, S>>>,
                                                                                                         world2: Arc<RwLock<World<I, D, S>>>) {
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

            if let Some(&WebSocketProtocol(ref protocols)) = headers.get() {
                if protocols.contains(&("mex-websocket".to_string())) {
                    response.headers.set(WebSocketProtocol(vec!["mex-websocket".to_string()]));
                } else {
                    return;
                }
            } else {
                return
            }

            let session_id = SessionID::new();

            let mut client = response.send().unwrap();

            let ip = client.get_mut_sender()
                .get_mut()
                .peer_addr()
                .unwrap();


            println!("Connection from {}", ip);

            let message: Message = Message::text("Hello".to_string());
            client.send_message(&message).unwrap();

            let (mut sender, mut receiver) = client.split();

            // message to create a hero
            input_tx.send(SessionInput { session_id: session_id, input: I::connection_created() }).unwrap();

            thread::spawn(move || {
                for message in receiver.incoming_messages() {
                    let message: Message = message.unwrap();

                    // get the message text
                    if let Some(input) = Input::from_str(&*to_cow_str(&message)) {
                        input_tx.send(SessionInput { session_id: session_id, input: input }).unwrap();
                        continue;
                    }
                }
            });
            // sending first full message. Later we can just diff
            let mut msg_type = MsgType::Full;
            loop {
                let world = if curr_world_is_1.load(Ordering::Acquire) {
                    world2.clone()
                } else {
                    world1.clone()
                };

                let msg;
                {
                    let read_world = world.read();
                    if !read_world.data.is_ready_for_msg(&session_id) {
                        continue
                    }
                    msg = String::from(&(*read_world.as_frontend_msg(msg_type, session_id)));
                }
                match sender.send_message(&Message::text(msg.as_str())) {
                    Ok(_) => (),
                    Err(_) => return,
                };
                msg_type = MsgType::Diff;
            }
        });
    }
}
