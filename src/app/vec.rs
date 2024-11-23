

pub trait Normal {
    fn cross_product(&self, v: [f32; 3]) -> [f32; 3];
    fn calc_face_normal(&self, b: [f32; 3], c: [f32; 3]) -> [f32; 3];
}

impl Normal for [f32; 3] {
    fn cross_product(&self, v: [f32; 3]) -> [f32; 3] {
        [
            self[1] * v[2] - self[2] * v[1],
            self[2] * v[0] - self[0] * v[2],
            self[0] * v[1] - self[1] * v[0]
        ]
    }
    fn calc_face_normal(&self, b: [f32; 3], c: [f32; 3]) -> [f32; 3] {
        let u = [
            b[0] - self[0],
            b[1] - self[1], 
            b[2] - self[2]
        ];
        let v = [
            c[0] - self[0],
            c[1] - self[1], 
            c[2] - self[2]
            ];
        u.cross_product(v)
    }
}

pub trait Normalize {
    fn normalize(& mut self);
}

impl Normalize for Vec<[f32; 3]> {
    fn normalize(& mut self) {
        for normal in self {
            let length = (normal[0] * normal[0] + 
                            normal[1] * normal[1] + 
                            normal[2] * normal[2])
                            .sqrt();
            if length == 0.00 {
                normal[0] = 1.0;
                normal[1] = length;
                normal[2] = length;
            }
            else {
            }
                normal[0] /= length;
                normal[1] /= length;
                normal[2] /= length;
        }
    }
}