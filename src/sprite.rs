use stash::Stash;

use engine::world::World;
use engine::sprites::Sprite;
use engine::input::SessionInput;

use Data;
use input::Input;

#[derive(Debug, Clone, Copy)]
pub struct Floor1 {
    pub pos: (f32, f32),
}

impl Floor1 {
    pub fn as_frontend_msg(&self, key: usize) -> String {
        format!("{{\"type\":\"floor1\",\"key\":{},\"pos\":[{},{}]}}",
                key,
                self.pos.0,
                self.pos.1)
    }
}


#[derive(Debug, Clone, Copy)]
pub struct Box1 {
    pub pos: (f32, f32, f32),
    pub scale: (f32, f32, f32),
    pub rotation: (f32, f32, f32),
    pub rotates: bool,
}

impl Box1 {
    pub fn as_frontend_msg(&self, key: usize) -> String {
        format!("{{\"type\":\"box1\",\"key\":{},\"pos\":[{},{},{}],\"scale\":[{},{},{}],\
                 \"rotation\":[{},{},{}]}}",
                key,
                self.pos.0,
                self.pos.1,
                self.pos.2,
                self.scale.0,
                self.scale.1,
                self.scale.2,
                self.rotation.0,
                self.rotation.1,
                self.rotation.2)
    }
    pub fn drift(&mut self, dt_ms: f32) {
        if self.rotates {
            self.rotation.2 += 0.001f32 * dt_ms;
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Hero {
    pub pos: (f32, f32),
    pub up_pressed: bool,
    pub down_pressed: bool,
    pub left_pressed: bool,
    pub right_pressed: bool,
}

impl Hero {
    pub fn drift(&mut self, dt_ms: f32) {
        let fact = 0.05 * 0.05 * dt_ms;
        if self.up_pressed {
            self.pos.1 += fact;
        }
        if self.down_pressed {
            self.pos.1 -= fact;
        }
        if self.left_pressed {
            self.pos.0 -= fact;
        }
        if self.right_pressed {
            self.pos.0 += fact;
        }
    }
    pub fn as_frontend_msg(&self, key: usize) -> String {
        format!("{{\"type\":\"hero\",\"key\":{},\"pos\":[{},{}]}}",
                key,
                self.pos.0,
                self.pos.1)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SpriteEnum {
    Hero(Hero),
    Floor1(Floor1),
    Box1(Box1),
}

impl Sprite<Input, Data> for SpriteEnum {
    fn get_initial_sprites() -> Vec<Self> {
        let mut sprites = Vec::new();
        sprites.push(SpriteEnum::Box1(Box1 {
            pos: (-3.0, -3.0, 0.2),
            scale: (1.0, 0.7, 1.0),
            rotation: (0.0, 0.0, 0.1),
            rotates: false,
        }));
        sprites.push(SpriteEnum::Box1(Box1 {
            pos: (-3.0, -2.0, 0.2),
            scale: (1.0, 0.7, 1.0),
            rotation: (0.0, 0.0, 0.1),
            rotates: false,
        }));
        sprites.push(SpriteEnum::Box1(Box1 {
            pos: (-3.0, -1.0, 0.2),
            scale: (1.0, 0.7, 1.0),
            rotation: (0.0, 0.0, 0.1),
            rotates: false,
        }));
        sprites.push(SpriteEnum::Box1(Box1 {
            pos: (-3.0, -0.0, 0.2),
            scale: (1.0, 0.7, 1.0),
            rotation: (0.0, 0.0, 0.1),
            rotates: true,
        }));
        sprites.push(SpriteEnum::Box1(Box1 {
            pos: (-3.0, 1.0, 0.2),
            scale: (1.0, 0.7, 1.0),
            rotation: (0.0, 0.0, 0.1),
            rotates: false,
        }));

        sprites.push(SpriteEnum::Box1(Box1 {
            pos: (1.0, -3.0, 0.2),
            scale: (1.0, 0.7, 1.0),
            rotation: (0.0, 0.0, 0.1),
            rotates: false,
        }));
        sprites.push(SpriteEnum::Box1(Box1 {
            pos: (1.0, -2.0, 0.2),
            scale: (1.0, 0.7, 1.0),
            rotation: (0.0, 0.0, 0.1),
            rotates: false,
        }));
        sprites.push(SpriteEnum::Box1(Box1 {
            pos: (1.0, -1.0, 0.2),
            scale: (1.0, 0.7, 1.0),
            rotation: (0.0, 0.0, 0.1),
            rotates: false,
        }));
        sprites.push(SpriteEnum::Box1(Box1 {
            pos: (1.0, -0.0, 0.2),
            scale: (1.0, 0.7, 1.0),
            rotation: (0.0, 0.0, 0.1),
            rotates: false,
        }));
        sprites.push(SpriteEnum::Box1(Box1 {
            pos: (3.0, 1.0, 0.2),
            scale: (1.0, 0.7, 1.0),
            rotation: (0.0, 0.0, 0.1),
            rotates: false,
        }));
        for x in -7..7 {
            for y in -7..7 {
                sprites.push(SpriteEnum::Floor1(Floor1 {
                    pos: (x as f32 * 5.0f32, y as f32 * 5.0f32),
                }));
            }
        }
        sprites
    }

    fn handle_inputs(inputs: &[SessionInput<Input>], next_world: &mut World<Input, Data, Self>) {
        for input in inputs {
            match input.input {
                Input::CreateHero => {
                    let mut sprites = &mut next_world.sprites;
                    let hero_key = sprites.put(SpriteEnum::Hero(Hero {
                        pos: (2.0, 2.0),
                        up_pressed: false,
                        down_pressed: false,
                        left_pressed: false,
                        right_pressed: false,
                    }));
                    next_world.data.hero_keys.insert(input.session_id, hero_key);
                }
                _ => {
                    let mut hero_mut = |f: &Fn(&mut Hero) -> ()| {
                        if let Some(&hero_key) = next_world.data.hero_keys.get(&input.session_id) {
                            return match *next_world.sprites.get_mut(hero_key).expect(format!("could not find sprite(hero) by key {}", hero_key).as_str()) {
                                SpriteEnum::Hero(ref mut hero) => f(hero),
                                _ => panic!("key {:?} does not belong to a Hero type", hero_key),
                            }
                        }
                        panic!("could not find hero key for session {:?}", input.session_id);
                    };
                    match input.input {
                        Input::UpPressed => hero_mut(&|h| { h.up_pressed = true }),
                        Input::DownPressed => hero_mut(&|h| { h.down_pressed = true }),
                        Input::LeftPressed => hero_mut(&|h| { h.left_pressed = true }),
                        Input::RightPressed => hero_mut(&|h| { h.right_pressed = true }),
                        Input::UpReleased => hero_mut(&|h| { h.up_pressed = false }),
                        Input::DownReleased => hero_mut(&|h| { h.down_pressed = false }),
                        Input::LeftReleased => hero_mut(&|h| { h.left_pressed = false }),
                        Input::RightReleased => hero_mut(&|h| { h.right_pressed = false }),
                        _ => (),
                    }
                },
            }
        }
    }

    fn handle_sprites(sprites: &Stash<Self>, next_world: &mut World<Input, Data, Self>, dt_ms: f32) {
        for (key, _) in sprites {
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
    }

    fn get_sprite_msg(&self, key: usize) -> String {
        match *self {
            SpriteEnum::Hero(hero) => hero.as_frontend_msg(key),
            SpriteEnum::Floor1(floor1) => floor1.as_frontend_msg(key),
            SpriteEnum::Box1(box1) => box1.as_frontend_msg(key),
        }
    }
}