use std::sync::{Arc, mpsc};
use std::collections::BTreeMap;
use std::sync::atomic::{Ordering, AtomicBool};

use parking_lot::RwLock;

use sessionid::SessionID;

#[derive(Debug)]
pub enum Input {
    UpPressed,
    DownPressed,
    LeftPressed,
    RightPressed,
    UpReleased,
    DownReleased,
    LeftReleased,
    RightReleased,
    CreateHero,
}

impl Input {
    pub fn from_str(s: &str) -> Option<Input> {
        match s {
            "up_pressed" => Some(Input::UpPressed),
            "down_pressed" => Some(Input::DownPressed),
            "left_pressed" => Some(Input::LeftPressed),
            "right_pressed" => Some(Input::RightPressed),
            "up_released" => Some(Input::UpReleased),
            "down_released" => Some(Input::DownReleased),
            "left_released" => Some(Input::LeftReleased),
            "right_released" => Some(Input::RightReleased),
            _ => None,
        }
    }
}

pub fn merge_inputs(input_rx: mpsc::Receiver<(SessionID, Input)>,
                    curr_world_is_1: Arc<AtomicBool>,
                    inputs1: Arc<RwLock<BTreeMap<SessionID, Input>>>,
                    inputs2: Arc<RwLock<BTreeMap<SessionID, Input>>>) {
    println!("merging inputs!");
    for (session_id, input) in input_rx {
        let mut inputs = if curr_world_is_1.load(Ordering::Relaxed) {
            inputs1.write()
        } else {
            inputs2.write()
        };
        inputs.insert(session_id, input);
    }
}
