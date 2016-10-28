
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