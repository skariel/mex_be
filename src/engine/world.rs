use std::marker::PhantomData;

use stash::Stash;

use engine::sprites::Sprite;
use engine::sessionid::SessionID;
use engine::input::{Input, SessionInput};

pub trait Data : Send+Sync+Sized {
    fn clone_from(&mut self, from: &Self);
    fn get_session_msg(&self, session_id: &SessionID) -> String;
    fn is_ready_for_msg(&self, session_id: &SessionID) -> bool;
}

pub struct World<I: Input, D: Data, S: Sprite<I, D>> {
    pub sprites: Stash<S>,
    pub elapsed_ms: f32,
    pub new_sprite_keys: Vec<usize>,
    pub removed_sprite_keys: Vec<usize>,
    pub updated_sprite_keys: Vec<usize>,
    pub data: D,
    _phantom:  PhantomData<I>,
}

pub enum MsgType {
    Full,
    Diff,
}

impl<I: Input, D: Data, S: Sprite<I,D>> World<I,D,S> {
    pub fn new(data: D) -> World<I,D,S> {
        let mut stash = Stash::new();
        let mut new_sprite_keys = Vec::new();

        for sprite in S::get_initial_sprites() {
            new_sprite_keys.push(stash.put(sprite));
        }

        World {
            sprites: stash,
            elapsed_ms: 0.0,
            new_sprite_keys: Vec::new(),
            removed_sprite_keys: Vec::new(),
            updated_sprite_keys: Vec::new(),
            data: data,
            _phantom: PhantomData,
        }
    }
    pub fn copy_into(&self, into: &mut World<I,D,S>) {
        // no need to copy the diff vectors (e.g. new_sprite_keys) since these get deleted anyway
        into.elapsed_ms = self.elapsed_ms;
        into.sprites.clone_from(&self.sprites);
        into.data.clone_from(&self.data);
    }
    pub fn advance(&self, next_world: &mut World<I,D,S>, dt_ms: f32, inputs: &[SessionInput<I>]) {
        self.copy_into(next_world);
        next_world.new_sprite_keys.clear();
        next_world.removed_sprite_keys.clear();
        next_world.updated_sprite_keys.clear();
        S::handle_inputs(inputs, next_world);
        S::handle_sprites(&self.sprites, next_world, dt_ms);
        next_world.elapsed_ms += dt_ms
    }
    pub fn as_frontend_msg(&self, msg_type: MsgType, session_id: SessionID) -> String {
        let new_sprite_msgs = match msg_type {
            MsgType::Diff => self.new_sprite_keys
                                .iter()
                                .map(|&key| {self.sprites.get(key).unwrap().get_sprite_msg(key)})
                                .collect::<Vec<String>>()
                                .join(","),
            MsgType::Full => self.sprites
                                .iter()
                                .map(|(key, value)| {value.get_sprite_msg(key)})
                                .collect::<Vec<String>>()
                                .join(","),
        };
        let updated_sprite_msgs = self.updated_sprite_keys
            .iter()
            .map(|&key| {self.sprites.get(key).unwrap().get_sprite_msg(key)})
            .collect::<Vec<String>>()
            .join(",");
        let removed_sprite_keys = self.removed_sprite_keys
            .iter()
            .map(|&key| {key.to_string()})
            .collect::<Vec<String>>()
            .join(",");

        format!("{{\"t\":{}, \"session_id\":\"{}\", {}, \"new_sprites\":[{}], \"updated_sprites\":[{}],\"removed_sprite_keys\":[{}] }}",
             self.elapsed_ms,
             session_id.to_string(),
             self.data.get_session_msg(&session_id),
             new_sprite_msgs,
             updated_sprite_msgs,
             removed_sprite_keys,
        )
    }
}
