use glium;
use glium::{
    glutin::surface::WindowSurface,
    Surface,
    Display
};

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
}
glium::implement_vertex!(Vertex, position);

pub struct Triangle {
    pub vertices: [Vertex; 3],
    pub t: f32
}

impl Triangle {
    pub fn new() -> Self {
        Self {
            t: 0.0,
            vertices: [
                Vertex { position: [-0.5, -0.5] },
                Vertex { position: [ 0.0,  0.5] },
                Vertex { position: [ 0.5, -0.3] }
            ]
        }
    }
    pub fn draw_triangle(&mut self, display: &Display<WindowSurface>) {
        let vertex_buffer = glium::VertexBuffer::new(display, &self.vertices).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        self.t += 0.02;
        let x_off = self.t.sin() * 0.5;

        // --------------------------
        // Shaders in GLSL
        let vertex_shader_src = r#"
            #version 140

            in vec2 position;

            uniform float x;
                
            void main() {
                vec2 pos = position;
                pos.x += x;
                gl_Position = vec4(pos, 0.0, 1.0);
            }
        "#;
        let fragment_shader_src = r#"
            #version 140

            out vec4 color;

            void main() {
                color = vec4(0.98, 0.33, 0.20, 1.0);
            }
        "#;
        // --------------------------
        
        let program = glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).expect("Error: \"glium::Program::from_source\" Fail");
        let mut frame = display.draw();
        frame.clear_color(0.45, 0.45, 1.0, 1.0);
        // frame.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms,&Default::default()).expect("Error: frame draw Fail");
        // frame.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms,&Default::default()).expect("Error: frame draw Fail");
           frame.draw(&vertex_buffer, &indices, &program, &glium::uniform! { x: x_off },&Default::default()).expect("Error: frame draw Fail");
        println!("t: {}", self.t);
        frame.finish().unwrap();
    }
}

impl Default for Triangle {
    fn default() -> Self {
        Self::new()
    }
}