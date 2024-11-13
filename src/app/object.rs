use glium::{
    self,
    // Texture2d,
    glutin::surface::WindowSurface,
    Surface,
    Display,
    VertexBuffer,
    IndexBuffer,
    uniform
};

use super::{
    ctx::Ctx,
    matrix::Matrix,
    shaders::Shader
};


#[derive(Copy, Clone, Debug)]
pub struct Normal {
    pub normal: (f32, f32, f32),
}
glium::implement_vertex!(Normal, normal);
impl Normal {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            normal: (x, y, z)
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: (f32, f32, f32),
    // pub tex_coords: [f32; 2]
}

glium::implement_vertex!(Vertex, position);

impl Vertex {
    // pub fn cros_product(&self) -> 
    pub fn from_vec(vec: &[f64]) -> Result<Self, &'static str> {
        if vec.len() != 3 {
            return Err("Error: Invalid argument: Vertex::from_vec requires exactly 3 elements.");
        }
        Ok(Self {
            position: (vec[0] as f32, vec[1] as f32, vec[2] as f32),
        })
    }
}


pub struct Object {
    // pub texture: Option<Texture2d>,
    pub indice: IndexBuffer<u16>,
    pub normal: VertexBuffer<Normal>,
    pub position: VertexBuffer<Vertex>,
    pub shaders: Shader

}

impl Object {
    pub fn new(
        display: &Display<WindowSurface>,
        ctx: &Ctx
    ) -> Self {
        Self {
            // texture: None,
            position: glium::VertexBuffer::new(display, &ctx.mesh.v)
                        .expect("Failed to create position buffer"),
            normal: glium::VertexBuffer::new(display, &ctx.mesh.get_normals())
                        .expect("Failed to create normal buffer"),
            indice: glium::IndexBuffer::new(
                display,
                glium::index::PrimitiveType::TrianglesList,
                &ctx.mesh.clone().get_indices(),
            ).expect("Failed to create index buffer"),
            shaders: Shader::default()
        }
    }

    pub fn get_color(r: u8, g: u8, b: u8) -> (f32, f32, f32, f32) {
        return ((r as f32 / 255.0),  (g as f32 / 255.0), (b as f32 / 255.0) , 1.0);
    }
    pub fn shaders_switch(&mut self, ctx: &mut Ctx) {
        self.shaders.switch_shading(ctx);
    }
    pub fn draw_obj(&mut self, display: &Display<WindowSurface>, ctx: & mut Ctx) {
        if ctx.rotation == true {
            ctx.rot_speed += ctx.speed_factor;
        }
        let uniforms = uniform! {
            rotation_matrix: Matrix::new_rotation(ctx).get_4x4_matrix(),
            perspective_matrix: Matrix::new_perspective(ctx).get_4x4_matrix(),
            object_center: ctx.mesh.centroid,
            light: ctx.light
        };
        let program = glium::Program::from_source(display, self.shaders.vertex_shader, self.shaders.fragment_shader, None)
                                        .expect("Error: \"glium::Program::from_source\" Fail");
        let mut frame = display.draw();
        frame.clear_color_and_depth(Object::get_color(0x02, 0x02, 0x02), 1.0);
        // -------------> Depth Testing + WireFrame + BackFaceCulling

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            backface_culling: if ctx.backface {
                glium::draw_parameters::BackfaceCullingMode::CullCounterClockwise
            } else {
                glium::draw_parameters::BackfaceCullingMode::CullingDisabled
            },
            polygon_mode: if !ctx.polmode {
                glium::draw_parameters::PolygonMode::Line
            } else {
                glium::draw_parameters::PolygonMode::Fill
            },
            .. Default::default()
        };
        // -------------> Depth Testing + WireFrame + BackFaceCulling
        frame.draw((&self.position, &self.normal), &self.indice, &program, &uniforms, &params).unwrap();        
        frame.finish().unwrap();
    }
}