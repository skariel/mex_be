#![feature(plugin)]
#![plugin(clippy)]

extern crate rand;
extern crate stash;
extern crate websocket;
extern crate parking_lot;

use std::collections::BTreeMap;

mod input;
mod sprite;
mod engine;

use engine::sessionid::SessionID;
use engine::world::{Data as DataTrait};
use engine::mainloop::{Config as ConfigTrait, run_game};

use sprite::SpriteEnum;

use input::Input;

struct Config {
    pub delay_between_snapshots_ms: u64,
}

impl ConfigTrait for Config {
    fn get_delay_between_snapshots_ms(&self) -> u64 {
        self.delay_between_snapshots_ms
    }
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
                } else { "".into() },
        )
    }
    fn is_ready_for_msg(&self, session_id: &SessionID) -> bool {
        if let Some(_) = self.hero_keys.get(session_id) {
            return true;
        };
        false
    }
    fn duplicate(&self) -> Self {
        let mut dut = Data::empty();
        dut.clone_from(self);
        dut
    }
}

fn main() {
    println!("Welcome to maxmech backend!");
    let config = Config {
        delay_between_snapshots_ms: 25,
    };
    run_game::<Config, Input, Data, SpriteEnum>(config, Data::empty());
}
