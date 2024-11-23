use glium::{
    self,
    Texture2d,
    glutin::surface::WindowSurface,
    Surface,
    Display,
    VertexBuffer,
    uniform
};

use super::{
    ctx::Ctx, matrix::Matrix, parser::Obj, shaders::Shader, mesh::Mesh
};


pub struct Images {
    pub dimension: (u32, u32),
    pub diffuse_texture: Texture2d
}

impl Images {
    pub fn new(display: &Display<WindowSurface>) -> Self {
        let img = image::load(std::io::Cursor::new(&include_bytes!("/Users/xtem/Desktop/scop/obj/Texture/test.png")),
        image::ImageFormat::Png).unwrap().to_rgba8();
        let dim = img.dimensions();
        let img = glium::texture::RawImage2d::from_raw_rgba_reversed(&img.into_raw(), dim);
        let tex = glium::texture::Texture2d::new(display, img).unwrap();
        Self {
            dimension: dim,
            diffuse_texture: tex
        }
    }
}

pub struct Renderer {
    mesh: Vec<Mesh>,
    shaders: Shader,
    img: Images,
}

impl Renderer {
    pub fn new(display: &Display<WindowSurface>, ctx: &Ctx) -> Self {
        let mut mesh:  Vec<Mesh> = Vec::new();
        let obj = &ctx.obj;
        for face in &obj.faces {
            for i in 0..3 {
                mesh.push(Mesh::new(
                    obj.vertexs[face.v[i] as usize],
                    obj.vn[face.vn[i] as usize],
                    obj.vt[face.vt[i] as usize]
                ))
            }
        }
        Self {
            mesh,
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
        let vertex_buffer = VertexBuffer::<Mesh>::new(display, &self.mesh).unwrap();

    
        let uniforms = uniform! {
            rotation_matrix: rotation_matrix,
            perspective_matrix: perspective_matrix,
            object_center: ctx.obj.centroid,
            tex: &self.img.diffuse_texture,
            mix_factor: ctx.mix_factor,
            light: ctx.light
        };
    
        let program = glium::Program::from_source(display, self.shaders.vertex_shader, self.shaders.fragment_shader, None)
            .expect("Error: \"glium::Program::from_source\" Fail");
    
        let mut frame = display.draw();
        frame.clear_color_and_depth(Renderer::get_color(0x05, 0x05, 0x05), 1.0);
    
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
            } else if ctx.polmode == 1 {
                glium::draw_parameters::PolygonMode::Line
            } else {
                glium::draw_parameters::PolygonMode::Point
            },
            .. Default::default()
        };

        frame.draw(
			&vertex_buffer,
            &glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
			&program,
			&uniforms,
			&params
		).unwrap();
        frame.finish().unwrap();
    }
}