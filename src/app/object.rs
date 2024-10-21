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


#[derive(Copy, Clone)]
pub struct Normal {
    pub normal: (f32, f32, f32),
}
glium::implement_vertex!(Normal, normal);


#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: (f32, f32, f32)
    // pub tex_coords: [f32; 2]
}    
glium::implement_vertex!(Vertex, position); 


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
        vertices: &[Vertex],
        normals: &[Normal],
        indices: &[u16],
    ) -> Self {
        Self {
            // texture: None,
            position: glium::VertexBuffer::new(display, vertices).unwrap(),
            normal: glium::VertexBuffer::new(display, normals).unwrap(),
            indice: glium::IndexBuffer::new(
                display,
                glium::index::PrimitiveType::TrianglesList,
                indices,
            ).unwrap(),
            shaders: Shader::default()
        }
    }

    pub fn shaders_switch(&mut self, ctx: &mut Ctx) {
        self.shaders.switch_shading(ctx);
    }

    pub fn draw_obj(&mut self, display: &Display<WindowSurface>, ctx: & mut Ctx) {
        if ctx.rotation == true {
            ctx.rot_speed += 0.015;
        }
        let uniforms = uniform! {
            rotation_matrix: Matrix::new_rotation(ctx).get_4x4_matrix(),
            perspective_matrix: Matrix::new_perspective(ctx).get_4x4_matrix(),
            light: [0.5, 1.0, -0.5f32]
        };
        let program = glium::Program::from_source(display, self.shaders.vertex_shader, self.shaders.fragment_shader, None).expect("Error: \"glium::Program::from_source\" Fail");
        let mut frame = display.draw();
        // ------------->Depth Testing Params + Backface culling
        frame.clear_color_and_depth((0.5, 0.5, 0.5, 1.0), 1.0);
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            backface_culling: if ctx.backface {
                glium::draw_parameters::BackfaceCullingMode::CullClockwise
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
        // ------------->Depth Testing
        frame.draw((&self.position, &self.normal), &self.indice, &program, &uniforms, &params).unwrap();        
        frame.finish().unwrap();
    }
}