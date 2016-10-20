#![feature(plugin)]
#![plugin(clippy)]

extern crate websocket;

use std::time;
use std::thread;
use std::vec::Vec;
use std::sync::{mpsc, Arc, RwLock};
use std::sync::atomic::{Ordering, AtomicBool};

mod connections;

struct Config {
    pub delay_between_snapshots_ms: u64,
}

struct World {
    pub x: f64,
    pub y: f64,
    pub elapsed_ms: f64,
}

impl World {
    pub fn advance(&self, next_world: &mut World, dt: f64, inputs: &mut Vec<Input>) {
        next_world.x = self.x + 10.0 * dt;
    }
}

fn model_loop(config: Arc<Config>, curr_world_is_1: Arc<AtomicBool>,
              world1: Arc<RwLock<World>>, world2: Arc<RwLock<World>>,
              inputs1: Arc<RwLock<Vec<Input>>>, inputs2: Arc<RwLock<Vec<Input>>>) {
    println!("looping the model");
    loop {
        let curr_world;
        let mut next_world;
        let mut next_inputs;

        if curr_world_is_1.load(Ordering::Relaxed) {
            next_world = world2.write().unwrap();
            curr_world = world1.read().unwrap();
            next_inputs = inputs2.write().unwrap();
        } else {
            next_world = world1.write().unwrap();
            curr_world = world2.read().unwrap();
            next_inputs = inputs1.write().unwrap();
        }

        curr_world.advance(&mut *next_world, 10.0, &mut *next_inputs);

        next_inputs.clear();

        curr_world_is_1.store(!curr_world_is_1.load(Ordering::Relaxed), Ordering::Relaxed);
        thread::sleep(time::Duration::from_millis(config.delay_between_snapshots_ms));
    };
}

fn merge_inputs(input_rx: mpsc::Receiver<Input>,
                curr_world_is_1: Arc<AtomicBool>,
                inputs1: Arc<RwLock<Vec<Input>>>, inputs2: Arc<RwLock<Vec<Input>>>) {
    println!("merging inputs!");
    for input in input_rx {
        let mut inputs = if curr_world_is_1.load(Ordering::Relaxed) {
            inputs1.write().unwrap()
        } else {
            inputs2.write().unwrap()
        };
        inputs.push(input);
    }
}

enum Input {
    I1,
    I2,
}

fn main() {
    println!("Welcome to maxmech backend!");

    let inputs1 = Arc::new(RwLock::new(Vec::new()));
    let inputs2 = Arc::new(RwLock::new(Vec::new()));
    let (input_tx, input_rx) = mpsc::channel::<Input>();
    let config = Arc::new(Config {
        delay_between_snapshots_ms: 50,
    });
    let curr_world_is_1 = Arc::new(AtomicBool::new(true));
    let world1 = Arc::new(RwLock::new(World {
        x: 0.0,
        y: 0.0,
        elapsed_ms: 0.0,
    }));
    let world2 = Arc::new(RwLock::new(World {
        x: 0.0,
        y: 0.0,
        elapsed_ms: 0.0,
    }));

    let t1 = thread::spawn(|| {
        connections::listen_to_incomming_connections();
    });

    let inputs1_model_loop = inputs1.clone();
    let inputs2_model_loop = inputs2.clone();
    let curr_world_is_1_model_loop = curr_world_is_1.clone();
    let t2 = thread::spawn(|| {
        model_loop(config,
                   curr_world_is_1_model_loop,
                   world1, world2,
                   inputs1_model_loop, inputs2_model_loop);
    });

    merge_inputs(input_rx, curr_world_is_1, inputs1, inputs2);

    t1.join().unwrap();
    t2.join().unwrap();
}
