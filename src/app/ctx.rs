pub struct Ctx {
    pub width: u32,
    pub height: u32,
    pub rotation: bool,
    pub x_factor: f32,
    pub y_factor: f32,
    pub z_factor: f32,
    pub rot_speed: f32,
    pub shading: bool,
    pub backface: bool,
    pub polmode: bool
}

impl Ctx {
    pub fn new() -> Self {
        Self {
            width: 1080,
            height: 1080,
            rotation: true,
            x_factor: 0.0,
            y_factor: 0.0,
            z_factor: 2.5,
            rot_speed: 0.0,
            shading: true,
            backface: true,
            polmode: true
        }
    }
}

impl Default for Ctx {
    fn default() -> Self {
        Self::new()
    }
}
