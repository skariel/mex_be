use stash::Stash;

use input::Input;
use sprite::{Hero, SpriteEnum};

pub struct World {
    pub sprites: Stash<SpriteEnum>,
    pub elapsed_ms: f32,
    pub hero_key: usize,
}

impl World {
    pub fn new() -> World {
        let mut stash = Stash::new();
        let hero_key = stash.put(SpriteEnum::Hero(Hero {
            pos: (2.0, 2.0),
            up_pressed: false,
            down_pressed: false,
            left_pressed: false,
            right_pressed: false,
        }));
        World {
            sprites: stash,
            elapsed_ms: 0.0,
            hero_key: hero_key,
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

        next_world.elapsed_ms += dt_ms
    }
    pub fn to_json(&self) -> String {
        let hero = self.hero();
        format!("{{\"t\":{}, \"x\":{}, \"y\":{}}}",
                self.elapsed_ms.to_string(),
                hero.pos.0.to_string(),
                hero.pos.1.to_string())
    }
}
