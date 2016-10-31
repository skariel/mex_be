use std::collections::BTreeMap;

use stash::Stash;

use sessionid::SessionID;
use input::{Input, SessionInput};
use sprite::{Hero, Floor1, Box1, SpriteEnum};

pub struct World {
    pub sprites: Stash<SpriteEnum>,
    pub elapsed_ms: f32,
    pub hero_keys: BTreeMap<SessionID, usize>,
    pub new_sprite_keys: Vec<usize>,
    pub removed_sprite_keys: Vec<usize>,
    pub updated_sprite_keys: Vec<usize>,
}

pub enum MsgType {
    Full,
    Diff,
}

impl World {
    pub fn new() -> World {
        let mut stash = Stash::new();
        let mut new_sprite_keys = Vec::new();

        new_sprite_keys.push(stash.put(SpriteEnum::Box1(Box1 {
            pos: (-3.0, -3.0, 0.2),
            scale: (1.0, 0.7, 1.0),
            rotation: (0.0, 0.0, 0.1),
            rotates: false,
        })));
        new_sprite_keys.push(stash.put(SpriteEnum::Box1(Box1 {
            pos: (-3.0, -2.0, 0.2),
            scale: (1.0, 0.7, 1.0),
            rotation: (0.0, 0.0, 0.1),
            rotates: false,
        })));
        new_sprite_keys.push(stash.put(SpriteEnum::Box1(Box1 {
            pos: (-3.0, -1.0, 0.2),
            scale: (1.0, 0.7, 1.0),
            rotation: (0.0, 0.0, 0.1),
            rotates: false,
        })));
        new_sprite_keys.push(stash.put(SpriteEnum::Box1(Box1 {
            pos: (-3.0, -0.0, 0.2),
            scale: (1.0, 0.7, 1.0),
            rotation: (0.0, 0.0, 0.1),
            rotates: true,
        })));
        new_sprite_keys.push(stash.put(SpriteEnum::Box1(Box1 {
            pos: (-3.0, 1.0, 0.2),
            scale: (1.0, 0.7, 1.0),
            rotation: (0.0, 0.0, 0.1),
            rotates: false,
        })));

        new_sprite_keys.push(stash.put(SpriteEnum::Box1(Box1 {
            pos: (1.0, -3.0, 0.2),
            scale: (1.0, 0.7, 1.0),
            rotation: (0.0, 0.0, 0.1),
            rotates: false,
        })));
        new_sprite_keys.push(stash.put(SpriteEnum::Box1(Box1 {
            pos: (1.0, -2.0, 0.2),
            scale: (1.0, 0.7, 1.0),
            rotation: (0.0, 0.0, 0.1),
            rotates: false,
        })));
        new_sprite_keys.push(stash.put(SpriteEnum::Box1(Box1 {
            pos: (1.0, -1.0, 0.2),
            scale: (1.0, 0.7, 1.0),
            rotation: (0.0, 0.0, 0.1),
            rotates: false,
        })));
        new_sprite_keys.push(stash.put(SpriteEnum::Box1(Box1 {
            pos: (1.0, -0.0, 0.2),
            scale: (1.0, 0.7, 1.0),
            rotation: (0.0, 0.0, 0.1),
            rotates: false,
        })));
        new_sprite_keys.push(stash.put(SpriteEnum::Box1(Box1 {
            pos: (3.0, 1.0, 0.2),
            scale: (1.0, 0.7, 1.0),
            rotation: (0.0, 0.0, 0.1),
            rotates: false,
        })));
        for x in -7..7 {
            for y in -7..7 {
                let floor1_key = stash.put(SpriteEnum::Floor1(Floor1 {
                    pos: (x as f32 * 5.0f32, y as f32 * 5.0f32),
                }));
                new_sprite_keys.push(floor1_key);
            }
        }

        World {
            sprites: stash,
            elapsed_ms: 0.0,
            hero_keys: BTreeMap::new(),
            new_sprite_keys: Vec::new(),
            removed_sprite_keys: Vec::new(),
            updated_sprite_keys: Vec::new(),
        }
    }
    pub fn copy_into(&self, into: &mut World) {
        // no need to copy the diff vectors (e.g. new_sprite_keys) since these get deleted anyway
        into.elapsed_ms = self.elapsed_ms;
        into.sprites.clone_from(&self.sprites);
        into.hero_keys.clone_from(&self.hero_keys);
    }
    pub fn hero_mut(&mut self, session_id: &SessionID) -> &mut Hero {
        if let Some(&hero_key) = self.hero_keys.get(session_id) {
            return match *self.sprites.get_mut(hero_key).expect(format!("could not find sprite(hero) by key {}", hero_key).as_str()) {
                SpriteEnum::Hero(ref mut hero) => hero,
                _ => panic!("key {:?} does not belong to a Hero type", hero_key),
            }
        }
        panic!("could not find hero key for session {:?}", session_id);

    }
    pub fn advance(&self, next_world: &mut World, dt_ms: f32, inputs: &[SessionInput]) {

        self.copy_into(next_world);
        next_world.new_sprite_keys.clear();
        next_world.removed_sprite_keys.clear();
        next_world.updated_sprite_keys.clear();

        for input in inputs {
            match input.input {
                Input::UpPressed => next_world.hero_mut(&input.session_id).up_pressed = true,
                Input::DownPressed => next_world.hero_mut(&input.session_id).down_pressed = true,
                Input::LeftPressed => next_world.hero_mut(&input.session_id).left_pressed = true,
                Input::RightPressed => next_world.hero_mut(&input.session_id).right_pressed = true,
                Input::UpReleased => next_world.hero_mut(&input.session_id).up_pressed = false,
                Input::DownReleased => next_world.hero_mut(&input.session_id).down_pressed = false,
                Input::LeftReleased => next_world.hero_mut(&input.session_id).left_pressed = false,
                Input::RightReleased => next_world.hero_mut(&input.session_id).right_pressed = false,
                Input::CreateHero => {
                    let hero_key = next_world.sprites.put(SpriteEnum::Hero(Hero {
                        pos: (2.0, 2.0),
                        up_pressed: false,
                        down_pressed: false,
                        left_pressed: false,
                        right_pressed: false,
                    }));
                    next_world.hero_keys.insert(input.session_id, hero_key);
                }
            }
        }

        for (key, _) in &self.sprites {
            // here next_world can be used to insert, delete or update sprites
            match next_world.sprites.get_mut(key) {
                Some(&mut SpriteEnum::Hero(ref mut hero)) => {
                    hero.drift(dt_ms);
                    next_world.updated_sprite_keys.push(key);
                },
                Some(&mut SpriteEnum::Box1(ref mut box1)) => {
                    box1.drift(dt_ms);
                    next_world.updated_sprite_keys.push(key);
                },
                _ => (),
            };
        }

        next_world.elapsed_ms += dt_ms
    }
    pub fn as_frontend_msg(&self, msg_type: MsgType, session_id: SessionID) -> String {
        let get_sprite_msg_by_value = |key, value: &SpriteEnum| {
            match *value {
                SpriteEnum::Hero(hero) => hero.as_frontend_msg(key),
                SpriteEnum::Floor1(floor1) => floor1.as_frontend_msg(key),
                SpriteEnum::Box1(box1) => box1.as_frontend_msg(key),
            }
        };
        let get_sprite_msg_by_key = |key| {
            if let Some(sprite) = self.sprites.get(key) {
                return get_sprite_msg_by_value(key, sprite);
            }
            panic!("get_sprite_msg: key {} not in sprite list", key.to_string());
        };

        let new_sprite_msgs = match msg_type {
            MsgType::Diff => self.new_sprite_keys
                                .iter()
                                .map(|&key| {get_sprite_msg_by_key(key)})
                                .collect::<Vec<String>>()
                                .join(","),
            MsgType::Full => self.sprites
                                .iter()
                                .map(|(key, value)| {get_sprite_msg_by_value(key, value)})
                                .collect::<Vec<String>>()
                                .join(","),
        };
        let updated_sprite_msgs = self.updated_sprite_keys
            .iter()
            .map(|&key| {get_sprite_msg_by_key(key)})
            .collect::<Vec<String>>()
            .join(",");
        let removed_sprite_keys = self.removed_sprite_keys
            .iter()
            .map(|&key| {key.to_string()})
            .collect::<Vec<String>>()
            .join(",");

        format!("{{\"t\":{}, \"session_id\":\"{}\", \"hero_key\":{}, \"new_sprites\":[{}], \"updated_sprites\":[{}],\"removed_sprite_keys\":[{}] }}",
             self.elapsed_ms,
             session_id.to_string(),
             if let Some(key) = self.hero_keys.get(&session_id) {
                 key.to_string()
             } else {"".into()},
             new_sprite_msgs,
             updated_sprite_msgs,
             removed_sprite_keys,
        )
    }
}
