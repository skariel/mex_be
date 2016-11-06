use engine::input;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Input {
    UpPressed,
    DownPressed,
    LeftPressed,
    RightPressed,
    UpReleased,
    DownReleased,
    LeftReleased,
    RightReleased,
    CreateHero,
}

impl input::Input for Input {
    fn from_str(s: &str) -> Option<Input> {
        match s {
            "up_pressed" => Some(Input::UpPressed),
            "down_pressed" => Some(Input::DownPressed),
            "left_pressed" => Some(Input::LeftPressed),
            "right_pressed" => Some(Input::RightPressed),
            "up_released" => Some(Input::UpReleased),
            "down_released" => Some(Input::DownReleased),
            "left_released" => Some(Input::LeftReleased),
            "right_released" => Some(Input::RightReleased),
            _ => None,
        }
    }
    fn connection_created() -> Input {
        Input::CreateHero
    }
}

