use input::Input;

pub struct World {
    pub x: f64,
    pub y: f64,
    pub elapsed_ms: f64,
}

impl World {
    pub fn advance(&self, next_world: &mut World, dt_ms: f64, inputs: &mut Vec<Input>) {
        next_world.x = self.x + 10.0 * dt_ms;
        next_world.elapsed_ms = self.elapsed_ms + dt_ms
    }
}
