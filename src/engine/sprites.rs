use stash::Stash;

use engine::input::{Input, SessionInput};
use engine::world::{Data, World};

pub trait Sprite<I: Input, D: Data>: Send+Sync+Clone+Sized {
    fn get_initial_sprites() -> Vec<Self>;
    fn handle_inputs(inputs: &[SessionInput<I>], next_world: &mut World<I,D,Self>);
    fn handle_sprites(sprites: &Stash<Self>, next_world: &mut World<I,D,Self>, dt_ms: f32);
    fn get_sprite_msg(&self, key: usize) -> String;
}
