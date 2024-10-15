use glium::{
    self,
    Texture2d,
    glutin::surface::WindowSurface,
    Surface,
    Display,
    uniform
};

use image;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
    tex_coords: [f32; 2]
    // pub color: [u32; 3]
}
glium::implement_vertex!(Vertex, position, tex_coords); 

pub struct Triangle {
    pub vertices: [Vertex; 3],
    pub t: f32,
    pub texture: Option<Texture2d>
}

impl Triangle {
    pub fn new() -> Self {
        Self {
            t: 0.0,
            // vertices: [
            //     Vertex { position: [ -0.40019527, -0.5], color: [1, 0, 0] },     // Coin inférieur gauche
            //     Vertex { position: [ 0.0,  0.1931586], color: [0, 1, 0] },              // Coin superieur
            //     Vertex { position: [ 0.40019527, -0.5], color: [0, 0, 1] }       // Coin inférieur droit
            // ]
            vertices: [
                Vertex { position: [ -0.40019527, -0.5], tex_coords: [0.0, 0.0] },
                Vertex { position: [ 0.0,  0.1931586], tex_coords: [1.0, 0.0] },
                Vertex { position: [ 0.40019527, -0.5], tex_coords: [1.0, 1.0] },
            ],
            texture: None
        }
    }
    pub fn load_textures(&mut self, display: &Display<WindowSurface>) {
        let image = image::load(std::io::Cursor::new(&include_bytes!("../../obj/Texture/TCom_DustOverlay_overlay_M.tif")),
            image::ImageFormat::Tiff).unwrap().to_rgba8();
        let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
        self.texture = Some(glium::texture::Texture2d::new(display, image).unwrap());
    }
    pub fn draw_triangle(&mut self, display: &Display<WindowSurface>) {

        // -----------
        let vertex_buffer = glium::VertexBuffer::new(display, &self.vertices).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

        // ------------
        self.t += 0.01;

        let uniforms = uniform! {
            matrix: [
                [ self.t.cos(), -self.t.sin(), 0.0, 0.0],
                [self.t.sin(), self.t.cos(), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0f32], 
            ],
            tex: self.texture.as_ref().map(|t| t.sampled()).unwrap_or_else(|| {
                panic!("Texture non disponible")
            })
        };

        let vertex_shader_src = r#"
            #version 330
            in vec2 position;
            in vec2 tex_coords;
            out vec2 v_tex_coords;
            
            uniform mat4 matrix;
            
            void main() {
                v_tex_coords = tex_coords;
                gl_Position = matrix * vec4(position, 0.0, 1.0);
            }
        "#;

        let fragment_shader_src = r#"
            #version 330
            in vec2 v_tex_coords;
            out vec4 color;
            
            uniform sampler2D tex;
            
            void main() {
                color = texture(tex, v_tex_coords);
            }
        "#;
        // --------------------------
        
        let program = glium::Program::from_source(display, vertex_shader_src, fragment_shader_src, None).expect("Error: \"glium::Program::from_source\" Fail");
        let mut frame = display.draw();
        frame.clear_color(1.0, 1.0, 1.0, 1.0);
        frame.draw(&vertex_buffer, &indices, &program, &uniforms,&Default::default()).unwrap();
        frame.finish().unwrap();
    }
}

impl Default for Triangle {
    fn default() -> Self {
        Self::new()
    }
}

pub trait Utils {
    fn get_vpos(&self, vtx: usize) -> (f32, f32);
    fn get_dist(&self, vtx1: usize, vtx2: usize) -> f32;
}

impl Utils for Triangle {
    fn get_vpos(&self, vtx: usize) -> (f32, f32) {
        if vtx > 3 || vtx < 1 {
            panic!("Error: Invalid vtx in get_vpos");
        } 
        let v: Vertex = self.vertices[vtx - 1];
        let x: f32 = (v.position[0] + 1.0) / 2.0 * 1920.0;
        let h: f32 = (v.position[1] + 1.0) / 2.0 * 1920.0;
        return (x, h);
    }
    fn get_dist(&self, vtx1: usize, vtx2: usize) -> f32 {
        if vtx1 > 3 || vtx1 < 1 || vtx2 > 3 || vtx2 < 1 && vtx1 == vtx2{
            panic!("Error: Invalid vtx in get_dist");
        } 
        let v1 = self.get_vpos(vtx1);
        let v2 = self.get_vpos(vtx2);

        let dx = v2.0 - v1.0;
        let dy = v2.1 - v1.1;
        
        return (dx * dx + dy * dy).sqrt();
    }
}


// [m11, m12, m13, m14],       // X
// [m21, m22, m23, m24],   // Y
// [m31, m32, m33, m34],     // Z
// [m41, m42, m43, m44],    // W