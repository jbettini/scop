use super::{
    object::Vertex,
    object::Normal,
    object::Textures,
    utils::has_duplicate
};
use std::fs::read_to_string;

#[derive(Clone, Debug)]
pub struct Face {
    pub f: Vec<u32>,
    pub vn: Vec<u32>,
    pub vt: Vec<u32>,
    pub mtl: String
}

impl Face {
    pub fn new(f: Vec<u32>, 
            t: Vec<u32>, 
            n: Vec<u32>, 
            mtl: String) -> Self {
        Self { 
            f, 
            vt: t,
            vn: n,
            mtl 
        }
    }
}

#[derive(Clone, Debug)]
pub struct Indices {
    pub vertices_indices: Vec<u32>,
    pub textures_indices: Vec<u32>,
}

impl Indices {
    pub fn new() -> Self {
        Self {
            vertices_indices: Vec::new(),
            textures_indices: Vec::new()
        }
    }
}


#[derive(Clone, Debug)]
pub struct ObjParams {
    pub s: String,
    pub name: Option<String>,
    pub mtlpath: Option<String>,
    pub vertexs: Vec<Vertex>,
    pub indices: Indices,
    pub faces_normals: Vec<Normal>,
    pub vn: Vec<Normal>,
    pub vt: Vec<Textures>,
    pub vertex_normals: Vec<Normal>,
    pub centroid: [f32; 3],
    pub faces: Vec<Face>,
}

impl ObjParams {
    pub fn new() -> Self {
        Self {
            name: None,
            mtlpath: None,
            vertexs: {
                let mut v = Vec::new();
                v.push(Vertex::new(0.0, 0.0, 0.0));
                v
            },
            s: "off".to_string(),
            indices: Indices::new(),
            faces: Vec::new(),
            faces_normals: Vec::new(),
            vn: Vec::new(),
            vt: Vec::new(),
            vertex_normals: Vec::new(),
            centroid: [0.0, 0.0, 0.0],
        }
    }
    
    fn init_indices(& mut self) {
        let mut vertices: Vec<u32> = Vec::new();
        let mut textures: Vec<u32> = Vec::new();

        for face in &self.faces {
            for x in &face.f {
                vertices.push(*x);
            }
            for x in &face.vt {
                textures.push(*x);
            }
        }
        self.indices.textures_indices = textures;
        self.indices.vertices_indices = vertices;
    }

    fn cross_product(&self, u: (f32, f32, f32), v: (f32, f32, f32)) -> Normal {
        Normal::new(
            u.1 * v.2 - u.2 * v.1,
            u.2 * v.0 - u.0 * v.2,
            u.0 * v.1 - u.1 * v.0
        )
    }

    fn calc_face_normal(&self, a: Vertex, b: Vertex, c: Vertex) -> Normal {
        let pos_a = a.position;
        let pos_b = b.position;
        let pos_c = c.position;
        let u = (
                pos_b.0 - pos_a.0,
                pos_b.1 - pos_a.1, 
                pos_b.2 - pos_a.2
            );
        let v = (
                pos_c.0 - pos_a.0, 
                pos_c.1 - pos_a.1, 
                pos_c.2 - pos_a.2
            );
        self.cross_product(u, v)
    }
    // fn init_textures_vertex(& mut self) {
    //     let mut texture_vtx: Vec<Normal> = vec![Normal::new(0.0, 0.0, 0.0)];
    //     for face in &self.faces {
    //         let (u, v) = (
    //             self.vt[face.vt[0] as usize],
    //             self.vt[face.vt[1] as usize],
    //         );
    //         texture_vtx.push(self.calc_face_normal(a, b, c));
    //     }
    //     self.vertex_textures = texture_vtx;
    // }
    fn init_faces_normals(& mut self) {
        let mut faces_normals: Vec<Normal> = vec![Normal::new(0.0, 0.0, 0.0)];
        for face in &self.faces {
            let (a, b, c) = (
                self.vertexs[face.f[0] as usize],
                self.vertexs[face.f[1] as usize],
                self.vertexs[face.f[2] as usize]
            );
            faces_normals.push(self.calc_face_normal(a, b, c));
        }
        self.faces_normals = faces_normals;
    }
    // TODO Careful with divide by zero
    pub fn normalize(vn: &mut Vec<Normal>) {
        for normal in vn {
            let length = (normal.normal.0 * normal.normal.0 + 
                          normal.normal.1 * normal.normal.1 + 
                          normal.normal.2 * normal.normal.2)
                          .sqrt();
            normal.normal.0 /= length;
            normal.normal.1 /= length;
            normal.normal.2 /= length;
        }
    } 

// TODO improve this             
    fn calculate_vertex_normals(&mut self, use_vn: bool) {
        let mut ret = vec![Normal { normal: (0.0, 0.0, 0.0) }; self.vertexs.len()];
        for (face_index, face) in self.faces.iter().enumerate() {
            let normal = if use_vn {
                &self.vertex_normals[face.vn[0] as usize]
            } else {
                &self.faces_normals[face_index + 1]
            };
            for vertex_index in &face.f {
                let vertex_normal = &mut ret[*vertex_index as usize];
                vertex_normal.normal.0 += normal.normal.0;
                vertex_normal.normal.1 += normal.normal.1;
                vertex_normal.normal.2 += normal.normal.2;
            }
        }
        ObjParams::normalize(&mut ret);
        self.vertex_normals = ret;
    }

    // TODO add vt
    fn init_others_fields(& mut self) {
        let mut use_vn = true;
        self.init_centroid();
        self.init_indices();
        if self.vertex_normals.len() == 0 {
            use_vn = false;
            self.init_faces_normals();
        }
        self.calculate_vertex_normals(use_vn);
        // self.init_textures_indices();
    }

    fn init_centroid(& mut self) {
        let len: f32 = self.vertexs.len() as f32;
        let mut x: f32 = 0.0;
        let mut y: f32 = 0.0;
        let mut z: f32 = 0.0;
        for vtx in self.vertexs.clone() {
            x += vtx.position.0;
            y += vtx.position.1;
            z += vtx.position.2;
        }
        self.centroid = [x / len, y / len, z / len];
    }
}

impl Default for ObjParams {
    fn default() -> Self {
        Self::new()
    }
}


pub fn check_coherence(parsed_obj: &ObjParams) -> Result<(), String> {
    let vlen: u32 = parsed_obj.vertexs.len() as u32;
    let vnlen: u32 = parsed_obj.vn.len() as u32;
    if vlen <= 0 || vlen >= 100000{
        return Err(format!("Error: vertexs must be between 1 and 1e6."));
    } else {
        for face in &parsed_obj.faces {
            for v in &face.f {
                if *v <= 0 || *v > vlen {
                    return Err(format!("Error: A face is out of the vertex range."));
                }
            }
            if vnlen != 0 {
                for vn in &face.vn {
                    if *vn <= 0 || *vn > vnlen {
                        return Err(format!("Error: A vn is out of the vertex normals range."));
                    }
                }
            }
            // TODO add check for vt
            if has_duplicate(&face.f) {
                return Err(format!("Error: A face contain duplicate vertex."));
            }
        }
        Ok(())
    }
}

fn get_file_lines(filepath: &str) -> Result<Vec<String>, String> {
    let content = read_to_string(filepath).map_err(|e| format!("Error: Cannot open {}. {}", filepath, e))?;
    let mut lines: Vec<String> = content
        .lines()
        .map(String::from)
        .collect();
    
    lines.retain(|s| !s.starts_with('#'));
    Ok(lines)
}

fn get_v(v: &[&str; 3]) -> Result<(f32, f32, f32), String> {
    let mut ret: Vec<f32> = Vec::new();
    for s in v {
        let test = s.parse::<f32>();
        match test {
            Ok(ok) => {
                ret.push(ok);
                continue;
            },
            Err(_) => {
                return Err(format!("Error: Invalid vertex {}, vertex must be f32.", s));
            }
        }
    }
    Ok((ret[0], ret[1], ret[2]))
}

fn get_parent_path(path: &str) -> Result<&str, &str> {
    if let Some((parent, _)) = path.rsplit_once('/') {
        if parent.is_empty() {
            Ok("/")
        } else {
            Ok(parent)
        }
    } else {
        Err("Error: mtllib option cannot be empty.")
    }
}

fn triangulize(splited: Vec<&str>) -> Vec<Vec<&str>> {
    let mut ret:  Vec<Vec<&str>> = Vec::new();
    ret.push(vec![splited[0], splited[1], splited[2]]);
    if splited.len() == 4 {
        ret.push(vec![splited[0], splited[2], splited[3]]);
    }
    return ret;
}

pub fn obj_parser(filepath: &str) -> Result<ObjParams, String> {
    let mut current_material = "off".to_string();
    let lines = get_file_lines(filepath)?;
    let mut obj: ObjParams = ObjParams::new();
    for line in lines {
        if let Some((key, rest)) = line.split_once(' ') {
            let splited: Vec<&str> = rest.split_whitespace().collect();
            match key {
                "mtllib" => {
                    if obj.mtlpath == None {
                        if splited.len() != 1 {
                            return Err(format!("Error: Invalid format : {} {:?}.", key, splited));
                        }
                        let parent_path = get_parent_path(filepath)?;
                        obj.mtlpath = Some(format!("{}/{}", parent_path, rest));
                    } else {
                        return Err(format!("Error: mtllib cannot be set twice {}.", line));
                    }
                },
                "o" => {
                    if obj.name == None {
                        if splited.len() != 1 {
                            return Err(format!("Error: Invalid format : {} {:?}.", key, splited));
                        }
                        obj.name = Some(rest.to_string());
                    } else {
                        return Err(format!("Error: object name cannot be set twice {}.", line));
                    }
                },
                "usemtl" => {
                    if splited.len() != 1 {
                        return Err(format!("Error: Invalid format : {} {:?}.", key, splited));
                    }
                    current_material = rest.to_string();
                },
                "v" => {
                    if splited.len() != 3 {
                        return Err(format!("Error: Invalid format : {} {:?}.", key, splited));
                    }
                    let v = get_v(&[splited[0], splited[1], splited[2]])?;
                    obj.vertexs.push(Vertex::new(v.0, v.1, v.2));
                },
                "f" => {
                    if splited.len() < 3 || splited.len() > 4 {
                        return Err(format!("Error: Face can contain only triangles or quadrilaterals : {} {:?}.",key, splited));
                    }
                    let face_args = triangulize(splited);
                    for vec in face_args {
                        let mut vertices = Vec::new();
                        let mut normals = Vec::new();
                        let mut textures = Vec::new();
                        for args in vec {
                            let parts: Vec<&str> = args.split('/').collect();
                            match parts.len() {
                                3 => {
                                    if let (Ok(v), Ok(vt), Ok(vn)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>(), parts[2].parse::<u32>()) {
                                        vertices.push(v);
                                        textures.push(vt);
                                        normals.push(vn);
                                    } else {
                                        return Err(format!("Error: Invalid face indices in '{}'", args));
                                    }
                                },
                                1 => {
                                    if let Ok(v) = parts[0].parse::<u32>() {
                                        vertices.push(v);
                                    } else {
                                        return Err(format!("Error: Invalid vertex index in '{}'", args));
                                    }
                                },
                                _ => return Err(format!("Error: Invalid face format in '{}'", args))
                            }
                        }
                        obj.faces.push(Face::new(vertices, textures, normals, current_material.clone()));
                    }
                },
                "s" => {
                    //LATER: Implementation
                },
                "vn" => {
                    if splited.len() != 3 {
                        return Err(format!("Error: Invalid format : {} {:?}.", key, splited));
                    }
                    let v = get_v(&[splited[0], splited[1], splited[2]])?;
                    obj.vn.push(Normal::new(v.0, v.1, v.2));
                },
                "vt" => {
                    if splited.len() != 2 {
                        return Err(format!("Error: Invalid format : {} {:?}.", key, splited));
                    }
                    match (splited[0].parse::<f32>(), splited[1].parse::<f32>()) {
                        (Ok(u), Ok(v)) => obj.vt.push(Textures::new(u, v)),
                        _ => Err(format!("Error: Invalid texture coordinates must be f32."))?
                    }
                }
                _ => return Err(format!("Error: Invalid Token {}.", key)),
            }
        }
    }
    if let Err(error) = check_coherence(&obj) {
        return Err(format!("{}", error));
    }
    obj.init_others_fields();
    Ok(obj)
}

// TODO add this for drag and drop
// fn parsing_handler(filepath: &str) -> Result<ObjParams, String> {
//     return obj_parser(filepath);
    // if filepath.to_lowercase().ends_with(".obj") {
//     } else if filepath.to_lowercase().ends_with(".tga") {
//         tga_parser(filepath);
//     } else {
//         println!("Error: Unsupported file extension.")
//     }
// }
