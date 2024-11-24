use super::ctx::Ctx;

#[derive(Copy, Clone, Debug)]
pub struct Mesh {
	position: [f32; 3],
	pub normal: [f32; 3],
	pub tex_coords: [f32; 2],
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
    pub fn get_mesh_vector(ctx: & mut Ctx) -> Vec<Self> {
        let mut mesh:  Vec<Mesh> = Vec::new();
        let obj = & mut ctx.obj;
        let vertex_normals = obj.calculate_vertex_normals();
        for face in &obj.faces {
            for i in 0..3 {
                let vertex = obj.vertexs[face.v[i] as usize];
                let normal = if obj.vn.len() <= 1 {
                    vertex_normals[face.v[i] as usize]
                } else {
                    obj.vn[face.vn[i] as usize]
                };
                let texture = if obj.vt.len() <= 1 {
                    // let u = (vertex[0] - obj.min_x) / (obj.max_x - obj.min_x);
                    // let u = (vertex[1] - obj.min_y) / (obj.max_y - obj.min_y);
                    let u = (vertex[2] - obj.min_z) / (obj.max_z - obj.min_z);

                    // let v = (vertex[0] - obj.min_x) / (obj.max_x - obj.min_x);
                    let v = (vertex[1] - obj.min_y) / (obj.max_y - obj.min_y);
                    // let v = (vertex[2] - obj.min_z) / (obj.max_z - obj.min_z);

                    [u, v]
                } else {
                    obj.vt[face.vt[i] as usize]
                };
                mesh.push(Mesh::new(vertex, normal, texture));
            }
        }
        return mesh;
    }
}

