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

use std::time::Instant;

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
    pub t: Instant,
    // pub texture: Option<Texture2d>,
    pub indice: IndexBuffer<u16>,
    pub normal: VertexBuffer<Normal>,
    pub position: VertexBuffer<Vertex>

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
            position: glium::VertexBuffer::new(display, vertices).unwrap(),
            normal: glium::VertexBuffer::new(display, normals).unwrap(),
            indice: glium::IndexBuffer::new(
                display,
                glium::index::PrimitiveType::TrianglesList,
                indices,
            ).unwrap(),
        }
    }

    pub fn draw_obj(&mut self, display: &Display<WindowSurface>) {
        let elapsed = self.t.elapsed().as_secs_f32() * 0.8;
        let uniforms = uniform! {
            matrix: [
                [(elapsed.cos() / 150.0), 0.0, (-elapsed.sin() / 150.0), 0.0],
                [0.0, 0.0066, 0.0, 0.0],
                [elapsed.sin() / 150.0, 0.0,  elapsed.cos() / 150.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32],
                // [0.01, 0.0, 0.0, 0.0],
                // [0.0, 0.01, 0.0, 0.0],
                // [0.0, 0.0, 0.01, 0.0],
                // [0.0, 0.0, 0.0, 1.0f32],
            ],
            light: [-1.0, 0.4, 0.9f32]
        };

        let vertex_shader_src = r#"
            #version 330
            in vec3 position;
            in vec3 normal;

            out vec3 v_normal;
            uniform mat4 matrix;

            void main() {
                v_normal = transpose(inverse(mat3(matrix))) * normal; 
                gl_Position = matrix * vec4(position, 1.0);
            }
        "#;

        let fragment_shader_src = r#"
            #version 330
            in vec3 v_normal;
            out vec4 color;
            uniform vec3 light;

            void main() {
                float brightness = dot(normalize(v_normal), normalize(light));
                vec3 dark_color = vec3(0.0, 0.4, 0.4);
                vec3 regular_color = vec3(0.0, 1.0, 1.0);
                color = vec4(mix(dark_color, regular_color, brightness), 1.0);
            }
        "#;
        
        let program = glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).expect("Error: \"glium::Program::from_source\" Fail");
        let mut frame = display.draw();

        // ------------->Depth Testing
        frame.clear_color_and_depth((0.5, 0.5, 0.5, 1.0), 1.0);
        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        };

        // ------------->Depth Testing
        frame.draw((&self.position, &self.normal), &self.indice, &program, &uniforms, &params).unwrap();        
        frame.finish().unwrap();
    }
}