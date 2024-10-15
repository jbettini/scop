use glium::{
    self,
    Texture2d,
    glutin::surface::WindowSurface,
    Surface,
    Display,
    VertexBuffer,
    IndexBuffer,
    uniform
};

use std::time::Instant;

// use super::teapot::{self, Normal};

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: (f32, f32, f32)
    // pub tex_coords: [f32; 2]
}
glium::implement_vertex!(Vertex, position); 

#[derive(Copy, Clone)]

pub struct Normal {
    pub normal: (f32, f32, f32),
}

glium::implement_vertex!(Normal, normal);



pub struct Object {
    pub t: Instant,
    // pub texture: Option<Texture2d>,
    pub indices: IndexBuffer<u16>,
    pub normals: VertexBuffer<Normal>,
    pub positions: VertexBuffer<Vertex>

}

impl Object {
    pub fn new(
        display: &Display<WindowSurface>,
        vertices: &[Vertex],
        normals: &[Normal],
        indices: &[u16],
    ) -> Self {
        Self {
            t: Instant::now(),
            // texture: None,
            positions: glium::VertexBuffer::new(display, vertices).unwrap(),
            normals: glium::VertexBuffer::new(display, normals).unwrap(),
            indices: glium::IndexBuffer::new(
                display,
                glium::index::PrimitiveType::TrianglesList,
                indices,
            ).unwrap(),
        }
    }

    pub fn draw_obj(&mut self, display: &Display<WindowSurface>) {

        let uniforms = uniform! {
            matrix: [
                [0.01, 0.0, 0.0, 0.0],
                [0.0, 0.01, 0.0, 0.0],
                [0.0, 0.0, 0.01, 0.0],
                [0.0, 0.0, 0.0, 1.0f32], 
            ]
        };

        let vertex_shader_src = r#"
            #version 330
            in vec3 position;
            in vec3 normal;

            uniform mat4 matrix;

            void main() {
                gl_Position = matrix * vec4(position, 1.0);
            }
        "#;

        let fragment_shader_src = r#"
            #version 330
            out vec4 color;

            void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
            }
        "#;
        
        let program = glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).expect("Error: \"glium::Program::from_source\" Fail");
        let mut frame = display.draw();
        frame.clear_color(1.0, 1.0, 1.0, 1.0);
        frame.draw((&self.positions, &self.normals), &self.indices, &program, &uniforms, &Default::default()).unwrap();
        frame.finish().unwrap();
    }
}