// // object.rs

// fn create_normal_buffers(&self, display: &Display<WindowSurface>, ctx: &Ctx) -> (glium::VertexBuffer<Vertex>, glium::IndexBuffer<u16>) {
//     let (normal_vertices, normal_indices) = ctx.mesh.generate_normal_lines(1.0);
//     let normal_vbo = glium::VertexBuffer::new(display, &normal_vertices).unwrap();
//     let normal_ibo = glium::IndexBuffer::new(display, glium::index::PrimitiveType::LinesList, &normal_indices).unwrap();
//     (normal_vbo, normal_ibo)
// }

// // Display normals segment In DrawObj fn
    // if ctx.show_normals {
    //     let (normal_vbo, normal_ibo) = self.create_normal_buffers(display, ctx);
    //     let normal_program = glium::Program::from_source(display, 
    //         self.shaders.normal_vertex_shader, 
    //         self.shaders.normal_fragment_shader, 
    //         None).unwrap();

    //     let normal_uniforms = uniform! {
    //         rotation_matrix: rotation_matrix,
    //         perspective_matrix: perspective_matrix,
    //         object_center: ctx.mesh.centroid,
    //         normal_length: 0.2f32,
    //     };

    //     let normal_params = glium::DrawParameters {
    //         depth: glium::Depth {
    //             test: glium::draw_parameters::DepthTest::IfLess,
    //             write: true,
    //             .. Default::default()
    //         },
    //         .. Default::default()
    //     };

    //     frame.draw(&normal_vbo, &normal_ibo, &normal_program, &normal_uniforms, &normal_params).unwrap();
    // }

// // Object::new 

    // let (normal_lines_vertices, normal_lines_indices) = ctx.mesh.generate_normal_lines(10.0);

    // normal_lines_position: glium::VertexBuffer::new(display, &normal_lines_vertices)
    //     .expect("Failed to create normal lines position buffer"),
    // normal_lines_indice: glium::IndexBuffer::new(
    //     display,
    //     glium::index::PrimitiveType::LinesList,
    //     &normal_lines_indices,
    // ).expect("Failed to create normal lines index buffer"),

// Object Struct
    // normal_lines_position: glium::VertexBuffer<Vertex>,
    // normal_lines_indice: glium::IndexBuffer<u16>,
// ---------------------------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------------------------


// // parser.rs

// pub fn generate_normal_lines(&self, length: f32) -> (Vec<Vertex>, Vec<u16>) {
//     let mut line_vertices = Vec::new();
//     let mut line_indices = Vec::new();
//     let mut index = 0;

//     for (vertex, normal) in self.vertexs.iter().zip(self.vertex_normals.iter()) {
//         line_vertices.push(Vertex { position: vertex.position });
        
//         let end = (
//             vertex.position.0 + normal.normal.0 * length,
//             vertex.position.1 + normal.normal.1 * length,
//             vertex.position.2 + normal.normal.2 * length,
//         );
//         line_vertices.push(Vertex { position: end });

//         line_indices.push(index as u16);
//         line_indices.push((index + 1) as u16);
//         index += 2;
//     }

//     (line_vertices, line_indices)
// }


// ---------------------------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------------------------


// // Ctx.rs

// Ctx struct
    // pub show_normals: bool
// Ctx::new
    // show_normals: false

// ---------------------------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------------------------

// // app.rs Main loop

    // KeyCode::KeyN => {
    //     self.ctx.show_normals = !self.ctx.show_normals;
    // },


// // shaders.rs

// pub normal_vertex_shader: &'static str,
// pub normal_fragment_shader: &'static str,


// normal_vertex_shader: r#"
// #version 330
// in vec3 position;
// in vec3 normal;

// uniform mat4 rotation_matrix;
// uniform mat4 perspective_matrix;
// uniform vec3 object_center;
// uniform float normal_length;

// void main() {
//     vec3 centered_position = position - object_center;
//     vec4 rotated_position = rotation_matrix * vec4(centered_position, 1.0);
//     vec3 final_position = vec3(rotated_position) + object_center;
//     gl_Position = perspective_matrix * vec4(final_position, 1.0);
// }
// "#,

// normal_fragment_shader: r#"
// #version 330
// out vec4 color;

// void main() {
//     color = vec4(1.0, 0.0, 0.0, 1.0);
// }
// "#,
// ---------------------------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------------------------
// ---------------------------------------------------------------------------------------------------------------------------------------



// // First way to calc normals

// pub fn calc_vertex_normals(&mut self) {
//     let mut ret = vec![Normal { normal: (0.0, 0.0, 0.0) }; self.vertexs.len()];
//     for (face_index, face) in self.faces.iter().enumerate() {
//         let normal = &self.faces_normals[face_index + 1];
//         for vertex_index in &face.f {
//             let vertex_normal = &mut ret[*vertex_index as usize];
//             vertex_normal.normal.0 += normal.normal.0;
//             vertex_normal.normal.1 += normal.normal.1;
//             vertex_normal.normal.2 += normal.normal.2;
//         }
//     }
//     ObjParams::normalize(&mut ret);
//     self.vertex_normals = ret;
// }
                
// fn init_vertex_normals_from_vn(&mut self) {
//     let mut ret = vec![Normal { normal: (0.0, 0.0, 0.0) }; self.vertexs.len()];
//     for face in &self.faces {
//         for (&vertex_index, &normal_index) in face.f.iter().zip(face.vn.iter()) {
//             let normal = self.vertex_normals[normal_index as usize];
//             let vertex_normal = &mut ret[vertex_index as usize];
//             vertex_normal.normal.0 += normal.normal.0;
//             vertex_normal.normal.1 += normal.normal.1;
//             vertex_normal.normal.2 += normal.normal.2;
//         }
//     }
//     ObjParams::normalize(&mut ret);
//     self.vertex_normals = ret;
// }