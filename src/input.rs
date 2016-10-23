use std::sync::{Arc, mpsc, RwLock};
use std::sync::atomic::{Ordering, AtomicBool};

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

pub fn merge_inputs(input_rx: mpsc::Receiver<Input>,
                    curr_world_is_1: Arc<AtomicBool>,
                    inputs1: Arc<RwLock<Vec<Input>>>, inputs2: Arc<RwLock<Vec<Input>>>) {
    println!("merging inputs!");
    for input in input_rx {
        let mut inputs = if curr_world_is_1.load(Ordering::Relaxed) {
            println!("merging input to i1");
            inputs1.write().unwrap()
        } else {
            println!("merging input to i2");
            inputs2.write().unwrap()
        };
        inputs.push(input);
    }
}

