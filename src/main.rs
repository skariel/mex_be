#![feature(plugin)]
#![plugin(clippy)]

extern crate rand;
extern crate stash;
extern crate websocket;
extern crate parking_lot;

use std::time;
use std::thread;
use std::sync::{mpsc, Arc};
use std::collections::BTreeMap;
use std::sync::atomic::{Ordering, AtomicBool};

use parking_lot::RwLock;

mod input;
mod sprite;
mod engine;

use engine::sprites::Sprite;
use engine::sessionid::SessionID;
use engine::world::{World, Data as DataTrait};
use engine::connections::listen_to_incomming_connections;
use engine::input::{Input as InputTrait, SessionInput, merge_inputs};

use sprite::SpriteEnum;

use input::Input;

struct Config {
    pub delay_between_snapshots_ms: u64,
}

struct Data {
    pub hero_keys: BTreeMap<SessionID, usize>,
}

impl Data {
    pub fn empty() -> Data {
        Data {
            hero_keys: BTreeMap::new(),
        }
    }
}

impl DataTrait for Data {
    fn clone_from(&mut self, from: &Self) {
        self.hero_keys.clone_from(&from.hero_keys);
    }
    fn get_session_msg(&self, session_id: &SessionID) -> String {
        format!("\"hero_key\":{}",
                if let Some(key) = self.hero_keys.get(&session_id) {
                    key.to_string()
                } else {"".into()},
        )
    }
    fn is_ready_for_msg(&self, session_id: &SessionID) -> bool {
        if let Some(_) = self.hero_keys.get(session_id) {
            return true;
        };
        false
    }
}

fn model_loop<I: InputTrait, D: DataTrait, S: Sprite<I,D>>(config: Arc<Config>,
              curr_world_is_1: Arc<AtomicBool>,
              world1: Arc<RwLock<World<I,D,S>>>,
              world2: Arc<RwLock<World<I,D,S>>>,
              inputs1: Arc<RwLock<Vec<SessionInput<I>>>>,
              inputs2: Arc<RwLock<Vec<SessionInput<I>>>>) {
    println!("looping the model");

    let time = time::SystemTime::now();
    let elapsed_ms = || -> f64 {
        time.elapsed().unwrap().as_secs() as f64 * 1000.0 +
        time.elapsed().unwrap().subsec_nanos() as f64 / 1000000.0
    };
    let mut frames = 0;

    curr_world_is_1.store(false, Ordering::Release);
    loop {
        curr_world_is_1.store(!curr_world_is_1.load(Ordering::Acquire), Ordering::Release);
        frames += 1;
        if frames % 1000 == 0 {
            println!("frames: {:?}", frames);
        }
        let t1 = elapsed_ms();
        let curr_world;
        let mut next_world;
        let mut next_inputs =
            if curr_world_is_1.load(Ordering::Acquire) {
                next_world = world2.write();
                curr_world = world1.read();
                inputs2.write()
            } else {
                next_world = world1.write();
                curr_world = world2.read();
                inputs1.write()
            };

        let dt: f32 = elapsed_ms() as f32 - curr_world.elapsed_ms;
        curr_world.advance(&mut *next_world, dt, &*next_inputs);

        next_inputs.clear();

        let dt = config.delay_between_snapshots_ms as f64 - (elapsed_ms() - t1);
        thread::sleep(time::Duration::from_millis(dt as u64));
    }
}

fn main() {
    println!("Welcome to maxmech backend!");

    // initialization of variables

    let inputs1 = Arc::new(RwLock::new(Vec::new()));
    let inputs2 = Arc::new(RwLock::new(Vec::new()));
    let (input_tx, input_rx) = mpsc::channel::<SessionInput<Input>>();
    let curr_world_is_1 = Arc::new(AtomicBool::new(true));
    let config = Arc::new(Config { delay_between_snapshots_ms: 30 });
    let world1: Arc<RwLock<World<Input,Data,SpriteEnum>>> = Arc::new(RwLock::new(World::new(Data::empty())));
    let world2: Arc<RwLock<World<Input,Data,SpriteEnum>>> = Arc::new(RwLock::new(World::new(Data::empty())));

    // running the game loop

    let ic1 = inputs1.clone();
    let ic2 = inputs2.clone();
    let w1 = world1.clone();
    let w2 = world2.clone();
    let cw = curr_world_is_1.clone();
    let t2 = thread::spawn(move || {
        model_loop(config, cw, w1, w2, ic1, ic2);
    });

    let w1 = world1.clone();
    let w2 = world2.clone();
    let cw = curr_world_is_1.clone();
    let t1 = thread::spawn(move || {
        listen_to_incomming_connections(input_tx, cw, w1, w2);
    });

    merge_inputs(input_rx, curr_world_is_1, inputs1, inputs2);

    t1.join().unwrap();
    t2.join().unwrap();
}
