
#[derive(Debug, Clone, Copy)]
pub struct Floor1 {
    pub pos: (f32, f32),
}

#[derive(Debug, Clone, Copy)]
pub struct Box1 {
    pub pos: (f32, f32, f32),
    pub scale: (f32, f32, f32),
    pub rotation: (f32, f32, f32),
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
        let fact = 0.05 * dt_ms;
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
}

#[derive(Debug, Clone, Copy)]
pub enum SpriteEnum {
    Hero(Hero),
    Floor1(Floor1),
    Box1(Box1),
}