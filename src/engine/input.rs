use std::sync::{Arc, mpsc};
use std::sync::atomic::{Ordering, AtomicBool};

use parking_lot::RwLock;

use engine::sessionid::SessionID;


#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct SessionInput<I: Input> {
    pub session_id: SessionID,
    pub input: I,
}

pub trait Input: Clone + Copy + Eq + PartialEq + Send + Sync {
    fn from_str(&str) -> Option<Self>;
    fn connection_created() -> Self;
}

pub fn merge_inputs<T: Input>(input_rx: mpsc::Receiver<SessionInput<T>>,
                              curr_world_is_1: Arc<AtomicBool>,
                              inputs1: Arc<RwLock<Vec<SessionInput<T>>>>,
                              inputs2: Arc<RwLock<Vec<SessionInput<T>>>>) {
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
