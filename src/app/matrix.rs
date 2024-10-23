use super::ctx::Ctx;

pub struct Matrix {
    mx: [f32; 4],
    my: [f32; 4],
    mz: [f32; 4],
    mw: [f32; 4]
}

impl Matrix {
    pub fn new_rotation(ctx: &Ctx) -> Self {
        let cos: f32 = ctx.rot_speed.cos() / 10.0;
        let sin: f32 = ctx.rot_speed.sin() / 10.0;
        // let cos = 1.0;
        // let sin = 1.0;
        Self {
            mx: [   cos          ,       0.0       ,       -sin,         0.0],
            my: [   0.0          ,       0.1       ,        0.0,         0.0],
            mz: [   sin          ,       0.0       ,        cos,         0.0],
            mw: [ ctx.x_factor   ,  ctx.y_factor   ,   ctx.z_factor,     1.0f32]
        }
    }
    pub  fn new_perspective(ctx: &Ctx) -> Self {
        let aspect_ratio: f32 = ctx.height as f32 / ctx.width as f32;

        let fov: f32 = std::f32::consts::PI / 3.0;
        let zfar: f32 = 1024.0;
        let znear: f32 = 0.1;
        let f: f32 = 1.0 / (fov / 2.0).tan();

        Self {
            mx: [f *   aspect_ratio   ,    0.0,              0.0              ,   0.0],
            my: [         0.0         ,     f,               0.0              ,   0.0],
            mz: [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
            mw: [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0]
        }
    }
    pub fn get_4x4_matrix(self) -> [[f32; 4]; 4] {
        return [
            self.mx,
            self.my,
            self.mz,
            self.mw
        ]
    }
}