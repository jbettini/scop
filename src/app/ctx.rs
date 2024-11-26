use super::parser::{
    obj_parser,
    Obj
};

pub struct Ctx {
    pub width: u32,
    pub height: u32,
    pub rotation: bool,
    pub x_factor: f32,
    pub y_factor: f32,
    pub z_factor: f32,
    pub rot_speed: f32,
    pub shading: u32,
    pub backface: bool,
    pub polmode: i32,
    pub speed_factor: f32,
    pub obj: Obj,
    pub light_move: bool,
    pub light: [f32; 3],
    pub texture: bool,
    pub mix_factor: f32
}

impl Ctx {
    pub fn new() -> Self {
        Self {
            width: 1080,
            height: 1080,
            rotation: true,
            x_factor: 0.0,
            y_factor: 0.0,
            z_factor: 8.0,
            rot_speed: 0.0,
            shading: 0,
            backface: true,
            polmode: 0,
            speed_factor: 0.015,
            obj: {
                let obj_ret = obj_parser("./obj/42.obj");
                match obj_ret {
                    Ok(obj) => obj,
                    Err(err) => {
                        println!("{:?}", err);
                        std::process::exit(1);
                    }
                }
            },
            light: [0.0, 0.0, -0.5],
            light_move: false,
            mix_factor: 1.0,
            texture: true
        }
    }
}

impl Default for Ctx {
    fn default() -> Self {
        Self::new()
    }
}
