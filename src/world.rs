use input::Input;

pub struct World {
    pub x: f64,
    pub y: f64,
    pub elapsed_ms: f64,
    up_pressed: bool,
    down_pressed: bool,
    left_pressed: bool,
    right_pressed: bool,
}

impl World {
    pub fn new() -> World {
        World {
            x: 0.0,
            y: 0.0,
            elapsed_ms: 0.0,
            up_pressed: false,
            down_pressed: false,
            left_pressed: false,
            right_pressed: false,
        }
    }
    pub fn advance(&self, next_world: &mut World, dt_ms: f64, inputs: &mut Vec<Input>) {
        next_world.up_pressed = self.up_pressed;
        next_world.down_pressed = self.down_pressed;
        next_world.left_pressed = self.left_pressed;
        next_world.right_pressed = self.right_pressed;
        next_world.x = self.x;
        next_world.y = self.y;
        next_world.elapsed_ms = self.elapsed_ms;

        for input in inputs {
            match *input {
                Input::UpPressed => next_world.up_pressed = true,
                Input::DownPressed => next_world.down_pressed = true,
                Input::LeftPressed => next_world.left_pressed = true,
                Input::RightPressed => next_world.right_pressed = true,
                Input::UpReleased => next_world.up_pressed = false,
                Input::DownReleased => next_world.down_pressed = false,
                Input::LeftReleased => next_world.left_pressed = false,
                Input::RightReleased => next_world.right_pressed = false,
            }
        }


        if next_world.up_pressed {
            next_world.y = self.y + 0.01 * dt_ms;
        }
        if next_world.down_pressed {
            next_world.y = self.y - 0.01 * dt_ms;
        }
        if next_world.left_pressed {
            next_world.x = self.x - 0.01 * dt_ms;
        }
        if next_world.right_pressed {
            next_world.x = self.x + 0.01 * dt_ms;
        }
        next_world.elapsed_ms += dt_ms
    }
    pub fn to_json(&self) -> String {
        format!("{{t:{}, x:{}, y:{}}}", self.elapsed_ms.to_string(), self.x.to_string(), self.y.to_string())
    }
    pub fn is_world_request(s: &str) -> bool {
        s.eq("w")
    }
}
