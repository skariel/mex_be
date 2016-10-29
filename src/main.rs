#![feature(plugin)]
#![plugin(clippy)]

extern crate rand;
extern crate stash;
extern crate websocket;
extern crate parking_lot;

use std::time;
use std::thread;
use std::vec::Vec;
use std::sync::{mpsc, Arc};
use std::collections::BTreeMap;
use std::sync::atomic::{Ordering, AtomicBool};

use parking_lot::RwLock;

mod world;
mod input;
mod sprite;
mod sessionid;
mod connections;

use world::World;
use sessionid::SessionID;
use input::{Input, merge_inputs};
use connections::listen_to_incomming_connections;

struct Config {
    pub delay_between_snapshots_ms: u64,
}

fn model_loop(config: Arc<Config>,
              curr_world_is_1: Arc<AtomicBool>,
              world1: Arc<RwLock<World>>,
              world2: Arc<RwLock<World>>,
              inputs1: Arc<RwLock<BTreeMap<SessionID,Input>>>,
              inputs2: Arc<RwLock<BTreeMap<SessionID,Input>>>) {
    println!("looping the model");

    let time = time::SystemTime::now();
    let elapsed_ms = || -> f64 {
        time.elapsed().unwrap().as_secs() as f64 * 1000.0 +
        time.elapsed().unwrap().subsec_nanos() as f64 / 1000000.0
    };
    let mut frames = 0;

    curr_world_is_1.store(false, Ordering::Relaxed);
    loop {
        curr_world_is_1.store(!curr_world_is_1.load(Ordering::Relaxed), Ordering::Relaxed);
        frames += 1;
        if frames % 100 == 0 {
            println!("frames: {:?}", frames);
        }
        let t1 = elapsed_ms();
        let curr_world;
        let mut next_world;
        let mut next_inputs;

        if curr_world_is_1.load(Ordering::Relaxed) {
            next_world = world2.write();
            curr_world = world1.read();
            next_inputs = inputs2.write();
        } else {
            next_world = world1.write();
            curr_world = world2.read();
            next_inputs = inputs1.write();
        }

        let dt: f32 = elapsed_ms() as f32 - curr_world.elapsed_ms;
        curr_world.advance(&mut *next_world, dt, &mut *next_inputs);

        next_inputs.clear();

        let dt = config.delay_between_snapshots_ms as f64 - (elapsed_ms() - t1);
        thread::sleep(time::Duration::from_millis(dt as u64));
    }
}

fn main() {
    println!("Welcome to maxmech backend!");

    // initialization of variables

    let inputs1 = Arc::new(RwLock::new(BTreeMap::new()));
    let inputs2 = Arc::new(RwLock::new(BTreeMap::new()));
    let (input_tx, input_rx) = mpsc::channel::<(SessionID, Input)>();
    let curr_world_is_1 = Arc::new(AtomicBool::new(true));
    let config = Arc::new(Config { delay_between_snapshots_ms: 30 });
    let world1 = Arc::new(RwLock::new(World::new()));
    let world2 = Arc::new(RwLock::new(World::new()));

    // running the game loop

    let ic1 = inputs1.clone();
    let ic2 = inputs2.clone();
    let w1 = world1.clone();
    let w2 = world2.clone();
    let cw = curr_world_is_1.clone();
    let t2 = thread::spawn(|| {
        model_loop(config, cw, w1, w2, ic1, ic2);
    });

    let w1 = world1.clone();
    let w2 = world2.clone();
    let cw = curr_world_is_1.clone();
    let t1 = thread::spawn(|| {
        listen_to_incomming_connections(input_tx, cw, w1, w2);
    });

    merge_inputs(input_rx, curr_world_is_1, inputs1, inputs2);

    t1.join().unwrap();
    t2.join().unwrap();
}
