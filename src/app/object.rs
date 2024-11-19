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

use super::{
    ctx::Ctx,
    matrix::Matrix,
    shaders::Shader,
    // parser::Indices,
};


pub struct Images {
    pub dimension: (u32, u32),
    pub diffuse_texture: Texture2d
}

impl Images {
    pub fn new(display: &Display<WindowSurface>) -> Self {
        let img = image::load(std::io::Cursor::new(&include_bytes!("/Users/xtem/Desktop/scop/obj/Texture/test.tif")),
        image::ImageFormat::Tiff).unwrap().to_rgba8();
        let dim = img.dimensions();
        let img = glium::texture::RawImage2d::from_raw_rgba_reversed(&img.into_raw(), dim);
        let tex = glium::texture::Texture2d::new(display, img).unwrap();
        Self {
            dimension: dim,
            diffuse_texture: tex
        }
    }
}
#[derive(Copy, Clone, Debug)]
pub struct Textures {
    pub tex_coords: (f32, f32),
}
impl Textures {
    pub fn new(u: f32, v: f32) -> Self {
        Self {
            tex_coords: (u, v)
        }
    }
}
glium::implement_vertex!(Textures, tex_coords);


#[derive(Copy, Clone, Debug)]
pub struct Normal {
    pub normal: (f32, f32, f32),
}
impl Normal {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            normal: (x, y, z)
        }
    }
}
glium::implement_vertex!(Normal, normal);

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: (f32, f32, f32),
}
impl Vertex {
    pub fn new(i: f32, j: f32, k: f32) -> Self {
        Self {
            position: (i, j, k)
        }
    }
}
glium::implement_vertex!(Vertex, position);

pub struct Object {
    indice: IndexBuffer<u32>,
    normal: VertexBuffer<Normal>,
    position: VertexBuffer<Vertex>,
    tex_coords: VertexBuffer<Textures>,
    shaders: Shader,
    img: Images,
}

impl Object {
    pub fn new(display: &Display<WindowSurface>, ctx: &Ctx) -> Self {
        
        Self {
            position: glium::VertexBuffer::new(display, &ctx.mesh.vertexs)
                .expect("Failed to create position buffer"),
            normal: glium::VertexBuffer::new(display, &ctx.mesh.vertex_normals)
                .expect("Failed to create normal buffer"),
            tex_coords: glium::VertexBuffer::new(display, &ctx.mesh.vt)
                .expect("Failed to create normal buffer"),
            indice: glium::IndexBuffer::new(
                display,
                glium::index::PrimitiveType::TrianglesList,
                &ctx.mesh.indices.vertices_indices,
            ).expect("Failed to create index buffer"),
            shaders: Shader::default(),
            img: Images::new(&display)
        }
    }

    pub fn get_color(r: u8, g: u8, b: u8) -> (f32, f32, f32, f32) {
        return ((r as f32 / 255.0),  (g as f32 / 255.0), (b as f32 / 255.0) , 1.0);
    }
    pub fn shaders_switch(&mut self, ctx: &mut Ctx) {
        self.shaders.switch_shading(ctx);
    }
    pub fn draw_obj(&mut self, display: &Display<WindowSurface>, ctx: &mut Ctx) {
        if ctx.rotation {
            ctx.rot_speed += ctx.speed_factor;
        }
    
        let rotation_matrix = Matrix::new_rotation(ctx).get_4x4_matrix();
        let perspective_matrix = Matrix::new_perspective(ctx).get_4x4_matrix();
    
        let uniforms = uniform! {
            rotation_matrix: rotation_matrix,
            perspective_matrix: perspective_matrix,
            object_center: ctx.mesh.centroid,
            tex: &self.img.diffuse_texture,
            mix_factor: ctx.mix_factor,
            light: ctx.light
        };
    
        let program = glium::Program::from_source(display, self.shaders.vertex_shader, self.shaders.fragment_shader, None)
            .expect("Error: \"glium::Program::from_source\" Fail");
    
        let mut frame = display.draw();
        frame.clear_color_and_depth(Object::get_color(0x05, 0x05, 0x05), 1.0);
    
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
            polygon_mode: if ctx.polmode == 0 {
                glium::draw_parameters::PolygonMode::Fill
            } else if  ctx.polmode == 1 {
                glium::draw_parameters::PolygonMode::Line
            } else {
                glium::draw_parameters::PolygonMode::Point
            },
            .. Default::default()
        };
        frame.draw((&self.position, &self.normal, &self.tex_coords), &self.indice, &program, &uniforms, &params).unwrap();
        frame.finish().unwrap();
    }
}