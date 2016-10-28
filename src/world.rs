use stash::Stash;

use input::Input;
use sprite::{Hero, Floor1, Box1, SpriteEnum};

pub struct World {
    pub sprites: Stash<SpriteEnum>,
    pub elapsed_ms: f32,
    pub hero_key: usize,
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
        let hero_key = stash.put(SpriteEnum::Hero(Hero {
            pos: (2.0, 2.0),
            up_pressed: false,
            down_pressed: false,
            left_pressed: false,
            right_pressed: false,
        }));
        new_sprite_keys.push(hero_key);
        new_sprite_keys.push(stash.put(SpriteEnum::Box1(Box1 {
            pos: (-3.0, -3.0, 0.2),
            scale: (1.0, 0.7, 1.0),
            rotation: (0.0, 0.0, 0.1),
        })));
        new_sprite_keys.push(stash.put(SpriteEnum::Box1(Box1 {
            pos: (-3.0, -2.0, 0.2),
            scale: (1.0, 0.7, 1.0),
            rotation: (0.0, 0.0, 0.1),
        })));
        new_sprite_keys.push(stash.put(SpriteEnum::Box1(Box1 {
            pos: (-3.0, -1.0, 0.2),
            scale: (1.0, 0.7, 1.0),
            rotation: (0.0, 0.0, 0.1),
        })));
        new_sprite_keys.push(stash.put(SpriteEnum::Box1(Box1 {
            pos: (-3.0, -0.0, 0.2),
            scale: (1.0, 0.7, 1.0),
            rotation: (0.0, 0.0, 0.1),
        })));
        new_sprite_keys.push(stash.put(SpriteEnum::Box1(Box1 {
            pos: (-3.0, 1.0, 0.2),
            scale: (1.0, 0.7, 1.0),
            rotation: (0.0, 0.0, 0.1),
        })));

        new_sprite_keys.push(stash.put(SpriteEnum::Box1(Box1 {
            pos: (1.0, -3.0, 0.2),
            scale: (1.0, 0.7, 1.0),
            rotation: (0.0, 0.0, 0.1),
        })));
        new_sprite_keys.push(stash.put(SpriteEnum::Box1(Box1 {
            pos: (1.0, -2.0, 0.2),
            scale: (1.0, 0.7, 1.0),
            rotation: (0.0, 0.0, 0.1),
        })));
        new_sprite_keys.push(stash.put(SpriteEnum::Box1(Box1 {
            pos: (1.0, -1.0, 0.2),
            scale: (1.0, 0.7, 1.0),
            rotation: (0.0, 0.0, 0.1),
        })));
        new_sprite_keys.push(stash.put(SpriteEnum::Box1(Box1 {
            pos: (1.0, -0.0, 0.2),
            scale: (1.0, 0.7, 1.0),
            rotation: (0.0, 0.0, 0.1),
        })));
        new_sprite_keys.push(stash.put(SpriteEnum::Box1(Box1 {
            pos: (3.0, 1.0, 0.2),
            scale: (1.0, 0.7, 1.0),
            rotation: (0.0, 0.0, 0.1),
        })));
        for x in -3..3 {
            for y in -3..3 {
                let floor1_key = stash.put(SpriteEnum::Floor1(Floor1 {
                    pos: (x as f32 * 5.0f32, y as f32 * 5.0f32),
                }));
                new_sprite_keys.push(floor1_key);
            }
        }

        World {
            sprites: stash,
            elapsed_ms: 0.0,
            hero_key: hero_key,
            new_sprite_keys: Vec::new(),
            removed_sprite_keys: Vec::new(),
            updated_sprite_keys: Vec::new(),
        }
    }
    pub fn copy_into(&self, into: &mut World) {
        into.hero_key = self.hero_key;
        into.elapsed_ms = self.elapsed_ms;
        for (k, v) in &self.sprites {
            if let Some(into_v) = into.sprites.get_mut(k) {
                *into_v = *v;
            }
        }
    }
    pub fn hero(&self) -> &Hero {
        match *self.sprites.get(self.hero_key).unwrap() {
            SpriteEnum::Hero(ref hero) => hero,
            _ => panic!("key {:?} does not belong to a Hero type", self.hero_key),
        }
    }
    pub fn hero_mut(&mut self) -> &mut Hero {
        match *self.sprites.get_mut(self.hero_key).unwrap() {
            SpriteEnum::Hero(ref mut hero) => hero,
            _ => panic!("key {:?} does not belong to a Hero type", self.hero_key),
        }
    }
    pub fn advance(&self, next_world: &mut World, dt_ms: f32, inputs: &mut Vec<Input>) {

        self.copy_into(next_world);
        next_world.new_sprite_keys.clear();
        next_world.removed_sprite_keys.clear();
        next_world.updated_sprite_keys.clear();

        for input in inputs {
            match *input {
                Input::UpPressed => next_world.hero_mut().up_pressed = true,
                Input::DownPressed => next_world.hero_mut().down_pressed = true,
                Input::LeftPressed => next_world.hero_mut().left_pressed = true,
                Input::RightPressed => next_world.hero_mut().right_pressed = true,
                Input::UpReleased => next_world.hero_mut().up_pressed = false,
                Input::DownReleased => next_world.hero_mut().down_pressed = false,
                Input::LeftReleased => next_world.hero_mut().left_pressed = false,
                Input::RightReleased => next_world.hero_mut().right_pressed = false,
            }
        }

        next_world.hero_mut().drift(dt_ms);
        next_world.updated_sprite_keys.push(next_world.hero_key);

        next_world.elapsed_ms += dt_ms
    }
    pub fn as_frontend_msg(&self, msg_type: MsgType) -> String {
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

        format!("{{\"t\":{}, \"new_sprites\":[{}], \"updated_sprites\":[{}],\"removed_sprite_keys\":[{}] }}",
             self.elapsed_ms,
             new_sprite_msgs,
             updated_sprite_msgs,
             removed_sprite_keys,
        )
    }
}
