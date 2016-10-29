use std::sync::{Arc, mpsc};
use std::sync::atomic::{Ordering, AtomicBool};

use parking_lot::RwLock;

use sessionid::SessionID;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
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

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct SessionInput {
    pub session_id: SessionID,
    pub input: Input,
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

pub fn merge_inputs(input_rx: mpsc::Receiver<SessionInput>,
                    curr_world_is_1: Arc<AtomicBool>,
                    inputs1: Arc<RwLock<Vec<SessionInput>>>,
                    inputs2: Arc<RwLock<Vec<SessionInput>>>) {
    println!("merging inputs!");
    for session_input in input_rx {
        let mut inputs = if curr_world_is_1.load(Ordering::Acquire) {
            inputs1.write()
        } else {
            inputs2.write()
        };
        inputs.push(session_input);
    }
}
