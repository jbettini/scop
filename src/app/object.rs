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
    shaders::Shader,
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
    pub fn new(i: f32, j: f32, k: f32) -> Self {
        Self {
            position: (i, j, k)
        }
    }
}


pub struct Object {
    indice: glium::IndexBuffer<u16>,
    normal: glium::VertexBuffer<Normal>,
    position: glium::VertexBuffer<Vertex>,
    shaders: Shader,
    normal_lines_position: glium::VertexBuffer<Vertex>,
    normal_lines_indice: glium::IndexBuffer<u16>,

}

impl Object {
    // pub fn new(
    //     display: &Display<WindowSurface>,
    //     ctx: &Ctx
    // ) -> Self {
    //     Self {
    //         // // texture: None,
    //         position: glium::VertexBuffer::new(display, &ctx.mesh.vertexs)
    //             .expect("Failed to create position buffer"),
    //         normal: glium::VertexBuffer::new(display, &ctx.mesh.vertex_normals)
    //             .expect("Failed to create normal buffer"),
    //         indice: glium::IndexBuffer::new(
    //             display,
    //             glium::index::PrimitiveType::TrianglesList,
    //             &ctx.mesh.indices,
    //         ).expect("Failed to create index buffer"),
    //         shaders: Shader::default()
    //     }
    // }
    //// TODO delete this
    pub fn new(display: &Display<WindowSurface>, ctx: &Ctx) -> Self {
        let (normal_lines_vertices, normal_lines_indices) = ctx.mesh.generate_normal_lines(0.1);
        
        Self {
            position: glium::VertexBuffer::new(display, &ctx.mesh.vertexs)
                .expect("Failed to create position buffer"),
            normal: glium::VertexBuffer::new(display, &ctx.mesh.vertex_normals)
                .expect("Failed to create normal buffer"),
            indice: glium::IndexBuffer::new(
                display,
                glium::index::PrimitiveType::TrianglesList,
                &ctx.mesh.indices,
            ).expect("Failed to create index buffer"),
            normal_lines_position: glium::VertexBuffer::new(display, &normal_lines_vertices)
                .expect("Failed to create normal lines position buffer"),
            normal_lines_indice: glium::IndexBuffer::new(
                display,
                glium::index::PrimitiveType::LinesList,
                &normal_lines_indices,
            ).expect("Failed to create normal lines index buffer"),
            shaders: Shader::default(),
        }
    }
    fn create_normal_buffers(&self, display: &Display<WindowSurface>, ctx: &Ctx) -> (glium::VertexBuffer<Vertex>, glium::IndexBuffer<u16>) {
        let (normal_vertices, normal_indices) = ctx.mesh.generate_normal_lines(0.1); // 0.1 est la longueur des normales
        let normal_vbo = glium::VertexBuffer::new(display, &normal_vertices).unwrap();
        let normal_ibo = glium::IndexBuffer::new(display, glium::index::PrimitiveType::LinesList, &normal_indices).unwrap();
        (normal_vbo, normal_ibo)
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
            light: ctx.light
        };
    
        let program = glium::Program::from_source(display, self.shaders.vertex_shader, self.shaders.fragment_shader, None)
            .expect("Error: \"glium::Program::from_source\" Fail");
    
        let mut frame = display.draw();
        frame.clear_color_and_depth(Object::get_color(0x02, 0x02, 0x02), 1.0);
    
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
    
        // Dessin de l'objet principal
        frame.draw((&self.position, &self.normal), &self.indice, &program, &uniforms, &params).unwrap();
    
        // Dessin des normales si activé
        if ctx.show_normals {
            let (normal_vbo, normal_ibo) = self.create_normal_buffers(display, ctx);
            let normal_program = glium::Program::from_source(display, 
                self.shaders.normal_vertex_shader, 
                self.shaders.normal_fragment_shader, 
                None).unwrap();
    
            let normal_uniforms = uniform! {
                rotation_matrix: rotation_matrix,
                perspective_matrix: perspective_matrix,
                object_center: ctx.mesh.centroid,
                normal_length: 0.1f32,
            };
    
            let normal_params = glium::DrawParameters {
                depth: glium::Depth {
                    test: glium::draw_parameters::DepthTest::IfLess,
                    write: true,
                    .. Default::default()
                },
                .. Default::default()
            };
    
            frame.draw(&normal_vbo, &normal_ibo, &normal_program, &normal_uniforms, &normal_params).unwrap();
        }
    
        frame.finish().unwrap();
    }
    // pub fn draw_obj(&mut self, display: &Display<WindowSurface>, ctx: & mut Ctx) {
    //     if ctx.rotation == true {
    //         ctx.rot_speed += ctx.speed_factor;
    //     }
    //     let uniforms = uniform! {
    //         rotation_matrix: Matrix::new_rotation(ctx).get_4x4_matrix(),
    //         perspective_matrix: Matrix::new_perspective(ctx).get_4x4_matrix(),
    //         object_center: ctx.mesh.centroid,
    //         light: ctx.light
    //     };
    //     let program = glium::Program::from_source(display, self.shaders.vertex_shader, self.shaders.fragment_shader, None)
    //                                     .expect("Error: \"glium::Program::from_source\" Fail");
    //     let mut frame = display.draw();
    //     frame.clear_color_and_depth(Object::get_color(0x02, 0x02, 0x02), 1.0);
    //     // -------------> Depth Testing + WireFrame + BackFaceCulling

    //     let params = glium::DrawParameters {
    //         depth: glium::Depth {
    //             test: glium::draw_parameters::DepthTest::IfLess,
    //             write: true,
    //             .. Default::default()
    //         },
    //         backface_culling: if ctx.backface {
    //             glium::draw_parameters::BackfaceCullingMode::CullCounterClockwise
    //         } else {
    //             glium::draw_parameters::BackfaceCullingMode::CullingDisabled
    //         },
    //         polygon_mode: if !ctx.polmode {
    //             glium::draw_parameters::PolygonMode::Line
    //         } else {
    //             glium::draw_parameters::PolygonMode::Fill
    //         },
    //         .. Default::default()
    //     };
    //     // -------------> Depth Testing + WireFrame + BackFaceCulling
    //     frame.draw((&self.position, &self.normal), &self.indice, &program, &uniforms, &params).unwrap();        
    //     frame.finish().unwrap();
    // }
}