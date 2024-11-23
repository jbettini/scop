#[derive(Copy, Clone, Debug)]
pub struct Mesh {
	position: [f32; 3],
	normal: [f32; 3],
	tex_coords: [f32; 2],
}

glium::implement_vertex!(Mesh, position, normal, tex_coords);

impl Mesh {
    pub fn new(
        position: [f32; 3],
        normal: [f32; 3],
        tex_coords: [f32; 2],
    ) -> Self {
        Self {
            position,
            normal,
            tex_coords
        }
    }
}

