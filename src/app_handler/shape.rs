use glium;
use glium::{
    glutin::surface::WindowSurface,
    Surface,
    Display,
    uniform
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
                Vertex { position: [ 0.5, -0.25] }
            ]
        }
    }
    pub fn draw_triangle(&mut self, display: &Display<WindowSurface>) {
        let vertex_buffer = glium::VertexBuffer::new(display, &self.vertices).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        self.t += 0.01;
        // let x_off = self.t.sin() * 0.5;
        let uniforms = uniform! {
            matrix: [
                [-self.t.cos(), self.t.sin(), 0.0, 0.0],
                [ self.t.sin(), -self.t.cos(), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32],
            ]
        };
        // let uniforms = uniform! {
        //     matrix: [
        //         [1.0, 0.0, 0.0, 0.0],   // Aucune rotation (cos(0) = 1)
        //         [0.0, 1.0, 0.0, 0.0],   // Aucune rotation (sin(0) = 0)
        //         [0.0, 0.0, 1.0, 0.0],   // Pas de modification de l'axe z (pour 2D)
        //         [0.0 , 0.0, 0.0, 1.0f32], // Translation selon l'axe x
        //     ]
        // };

        // --------------------------
        // Shaders in GLSL

        /*
            Move right to left
        */
        // let vertex_shader_src = r#"
        //     #version 140

        //     in vec2 position;

        //     uniform float x;
                
        //     void main() {
        //         vec2 pos = position;
        //         pos.x += x;
        //         gl_Position = vec4(pos, 0.0, 1.0);
        //     }
        // "#;

        /*
            Rotation
        */
        let vertex_shader_src = r#"
            #version 330
            in vec2 position;
            uniform mat4 matrix;

            void main() {
                gl_Position = matrix * vec4(position, 0.0, 1.0);
            }
        "#;

        let fragment_shader_src = r#"
            #version 330

            out vec4 color;

            void main() {
                color = vec4(0.98, 0.33, 0.20, 1.0);
            }
        "#;
        // --------------------------
        
        let program = glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).expect("Error: \"glium::Program::from_source\" Fail");
        let mut frame = display.draw();
        frame.clear_color(0.45, 0.45, 1.0, 1.0);
        // // To move 
        // frame.draw(&vertex_buffer, &indices, &program, &glium::uniform! { x: x_off },&Default::default()).expect("Error: frame draw Fail");
        frame.draw(&vertex_buffer, &indices, &program, &uniforms,&Default::default()).unwrap();
        frame.finish().unwrap();
    }
}

impl Default for Triangle {
    fn default() -> Self {
        Self::new()
    }
}