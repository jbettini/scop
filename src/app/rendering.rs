use glium::{
    self,
    glutin::surface::WindowSurface,
    Surface,
    Display,
    VertexBuffer,
    uniform
};

use super::{
    ctx::Ctx, 
    matrix::Matrix,
    shaders::Shader, 
    mesh::Mesh,
    parser::Images
};


pub struct Renderer {
    pub mesh: Vec<Mesh>,
    pub img: Images,
    shaders: Shader,
}

impl Renderer {
    pub fn new(display: &Display<WindowSurface>, ctx: & mut Ctx) -> Self {
        Self {
            mesh: Mesh::get_mesh_vector(ctx),
            shaders: Shader::default(),
            img: {
                let img = Images::new(display, "/Users/xtem/Desktop/scop/obj/Texture/metal.ppm");
                match img {
                    Ok(img) => img,
                    Err(err) => {
                        println!("{:?}", err);
                        std::process::exit(1);
                    }
                }
            }
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
        if ctx.texture && ctx.mix_factor < 1.0 {
            ctx.mix_factor += 0.05;
        } else if !ctx.texture && ctx.mix_factor > 0.0 {
            ctx.mix_factor -= 0.05;
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
        frame.clear_color_and_depth(Renderer::get_color(0x00, 0x05, 0x10), 1.0);
    
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