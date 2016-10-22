use std::sync::{Arc, mpsc, RwLock};
use std::sync::atomic::{Ordering, AtomicBool};

#[derive(Debug)]
pub enum Input {
    Up_pressed,
    Down_pressed,
    Left_pressed,
    Right_pressed,
    Up_released,
    Down_released,
    Left_released,
    Right_released,
}

impl Input {
    pub fn from_str(s: &str) -> Option<Input> {
        match s {
            "i:up_pressed" => Some(Input::Up_pressed),
            "i:down_pressed" => Some(Input::Down_pressed),
            "i:left_pressed" => Some(Input::Left_pressed),
            "i:right_pressed" => Some(Input::Right_pressed),
            "i:up_released" => Some(Input::Up_released),
            "i:down_released" => Some(Input::Down_released),
            "i:left_released" => Some(Input::Left_released),
            "i:right_released" => Some(Input::Right_released),
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

