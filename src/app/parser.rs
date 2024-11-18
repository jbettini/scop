

use super::{
    object::Vertex,
    utils::has_duplicate,
    object::Normal,
};
use std::fs::read_to_string;

#[derive(Clone, Debug)]
struct Faces {
    f: Vec<u16>,
    vn: Vec<u16>,
    vt: Vec<u16>,
    mtl: String
}

impl Faces {
    pub fn new(f: Vec<u16>, 
                n: Option<Vec<u16>>, 
                t: Option<Vec<u16>>, 
                mtl: String) -> Self {
        Self { 
            f, 
            vn: n.unwrap_or_else(Vec::new),
            vt: t.unwrap_or_else(Vec::new),
            mtl 
        }
    }
}
#[derive(Clone, Debug)]
pub struct ObjParams {
    pub s: String,
    pub name: Option<String>,
    pub mtlpath: Option<String>,
    pub vertexs: Vec<Vertex>,
    pub indices: Vec<u16>,
    pub faces_normals: Vec<Normal>,
    pub vn: Vec<Normal>,
    pub vt: Vec<(f32, f32)>,
    pub vertex_normals: Vec<Normal>,
    pub vertex_textures: Vec<(f32, f32)>,
    pub centroid: [f32; 3],
    faces: Vec<Faces>
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
            indices: Vec::new(),
            faces: Vec::new(),
            faces_normals: Vec::new(),
            vn: Vec::new(),
            vt: Vec::new(),
            vertex_normals: Vec::new(),
            vertex_textures: Vec::new(),
            centroid: [0.0, 0.0, 0.0],
        }
    }

    pub fn generate_normal_lines(&self, length: f32) -> (Vec<Vertex>, Vec<u16>) {
        let mut line_vertices = Vec::new();
        let mut line_indices = Vec::new();
        let mut index = 0;

        for (vertex, normal) in self.vertexs.iter().zip(self.vertex_normals.iter()) {
            line_vertices.push(Vertex { position: vertex.position });
            
            let end = (
                vertex.position.0 + normal.normal.0 * length,
                vertex.position.1 + normal.normal.1 * length,
                vertex.position.2 + normal.normal.2 * length,
            );
            line_vertices.push(Vertex { position: end });

            line_indices.push(index as u16);
            line_indices.push((index + 1) as u16);
            index += 2;
        }

        (line_vertices, line_indices)
    }

    
    pub fn init_indices(& mut self) {
        let mut ret: Vec<u16> = Vec::new();
        for face in &self.faces {
            for x in &face.f {
                ret.push(*x);
            }
        }
        self.indices = ret;
    }

    pub fn cross_product(&self, u: (f32, f32, f32), v: (f32, f32, f32)) -> Normal {
        Normal::new(
            u.1 * v.2 - u.2 * v.1,
            u.2 * v.0 - u.0 * v.2,
            u.0 * v.1 - u.1 * v.0
        )
    }

    pub fn calc_face_normal(&self, a: Vertex, b: Vertex, c: Vertex) -> Normal {
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

    pub fn init_faces_normals(& mut self) {
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
    // TODO : Refaire les 2 fonctions pour les normal
    pub fn calc_vertex_normals(&mut self) {
        let mut vertex_normals = vec![Normal { normal: (0.0, 0.0, 0.0) }; self.vertexs.len()];
        for (face_index, face) in self.faces.iter().enumerate() {
            let face_normal = &self.faces_normals[face_index + 1];
            for vertex_index in &face.f {
                vertex_normals[*vertex_index as usize].normal.0 += face_normal.normal.0;
                vertex_normals[*vertex_index as usize].normal.1 += face_normal.normal.1;
                vertex_normals[*vertex_index as usize].normal.2 += face_normal.normal.2;
            }
        }
        for normal in &mut vertex_normals {
            let length = (normal.normal.0 * normal.normal.0 + 
                          normal.normal.1 * normal.normal.1 + 
                          normal.normal.2 * normal.normal.2)
                          .sqrt();
            normal.normal.0 /= length;
            normal.normal.1 /= length;
            normal.normal.2 /= length;
        }
        self.vertex_normals = vertex_normals;
    }

    fn init_vertex_normals_from_vn(&mut self) {
        let mut vertex_normals = vec![Normal { normal: (0.0, 0.0, 0.0) }; self.vertexs.len()];
    
        for face in &self.faces {
            for (&vertex_index, &normal_index) in face.f.iter().zip(face.vn.iter()) {
                // TODO = Error handling
                if vertex_index as usize >= self.vertexs.len() || normal_index as usize >= self.vertex_normals.len() {
                    continue;
                }
                let normal = self.vertex_normals[normal_index as usize];

                let vertex_normal = &mut vertex_normals[vertex_index as usize];
                vertex_normal.normal.0 += normal.normal.0;
                vertex_normal.normal.1 += normal.normal.1;
                vertex_normal.normal.2 += normal.normal.2;
            }
        }
    
        // Normaliser toutes les normales de vertex
        for normal in &mut vertex_normals {
            let length = (normal.normal.0.powi(2) + normal.normal.1.powi(2) + normal.normal.2.powi(2)).sqrt();
            if length > 0.0 {
                normal.normal.0 /= length;
                normal.normal.1 /= length;
                normal.normal.2 /= length;
            }
        }
    
        // Remplacer les normales de vertex existantes par les nouvelles
        self.vertex_normals = vertex_normals;
    }
    
    // TODO Ajoutez les vt
    fn init_obj(& mut self) {
        self.init_centroid();
        self.init_indices();
        // Todo init from faces
        if self.vertex_normals.len() == 0 {
            self.init_faces_normals();
            self.calc_vertex_normals();
        } else {
            self.init_vertex_normals_from_vn();
        }
        // println!("vn : {:?}, vx : {:?}", self.vertex_normals.len(), self.vertexs.len() )
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

fn get_file_lines(filepath: &str) -> Result<Vec<String>, String> {
    let content = read_to_string(filepath).map_err(|e| format!("Error: Cannot open {}. {}", filepath, e))?;
    let mut lines: Vec<String> = content
        .lines()
        .map(String::from)
        .collect();
    
    lines.retain(|s| !s.starts_with('#'));
    Ok(lines)
}

fn  check_line(splited: &Vec<&str>, name: &str) -> Result<(), String> {
    let joined = splited.join(" ");
    match name {
        "usemtl" | "mtllib" | "o" if splited.len() != 1 => Err(format!("Error: Invalid format {}{}.", name, joined)),
        "v" | "vn" if splited.len() != 3 => Err(format!("Error: Invalid format {}{}.", name, joined)),
        "vt" if splited.len() != 2 => Err(format!("Error: Invalid format {}{}.", name, joined)),
        "f" if splited.len() < 3 || splited.len() > 4 => Err(format!("Error: Face can contain only triangles or quadrilaterals : {}{}.", name, joined)),
        _ => Ok(()),
    }
}

fn get_vertex(vtx: &[&str; 3]) -> Result<Vertex, String> {
    let mut ret: Vec<f32> = Vec::new();
    for s in vtx {
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
    Ok(Vertex::new(ret[0], ret[1], ret[2]))
}
fn get_vn(vtx: &[&str; 3]) -> Result<Normal, String> {
    let mut ret: Vec<f32> = Vec::new();
    for s in vtx {
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
    Ok(Normal::new(ret[0], ret[1], ret[2]))
}

fn get_simple_face(face: &[&str; 3]) -> Result<Vec<u16>, String> {
    let mut ret: Vec<u16> = Vec::new();
    for s in face {
        let test = s.parse::<u16>();
        match test {
            Ok(ok) => {
                ret.push(ok);
                continue;
            },
            Err(_) => {
                return Err(format!("Error: Invalid face {}, faces must be u16.", s));
            }
        }
    }
    Ok(ret)
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

fn check_coherence(parsed_obj: &ObjParams) -> Result<(), String> {
    let len: u16 = parsed_obj.vertexs.len() as u16;
    if len <= 0 {
        return Err(format!("Error: No vertexs."));
    } else {
        for face in &parsed_obj.faces {
            for point in &face.f {
                if *point <= 0 || *point > len {
                    return Err(format!("Error: A face is out of the vertex range: {}/{}t.", point, len));
                }
            }
            if has_duplicate(&face.f) {
                return Err(format!("Error: A face contain duplicate vertex."));
            }
        }
        Ok(())
    }
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
                        check_line(&splited, "mtllib")?;
                        let parent_path = get_parent_path(filepath)?;
                        obj.mtlpath = Some(format!("{}/{}", parent_path, rest));
                    } else {
                        return Err(format!("Error: mtllib cannot be set twice {}.", line));
                    }
                },
                "o" => {
                    if obj.name == None {
                        check_line(&splited, "o")?;
                        obj.name = Some(rest.to_string());
                    } else {
                        return Err(format!("Error: object name cannot be set twice {}.", line));
                    }
                },
                "usemtl" => {
                    check_line(&splited, "usemtl")?;
                    current_material = rest.to_string();
                },
                "v" => {
                    check_line(&splited, "v")?;
                    let v1 = get_vertex(&[splited[0], splited[1], splited[2]])?;
                    obj.vertexs.push(v1);
                },
                "f" => { // TODO ajouter les quad
                    check_line(&splited, "f")?;
                    if obj.vn.len() > 0 && obj.vt.len() > 0 {
                        let mut vertices = Vec::new();
                        let mut normals = Vec::new();
                        let mut textures = Vec::new();
                        for s in splited {
                            let parts: Vec<&str> = s.split('/').collect();
                            if parts.len() != 3 {
                                Err(format!("Error: Invalid syntax, faces must contain v/vn/vt."))?
                            }
                            match (parts[0].parse::<u16>(), parts[1].parse::<u16>(), parts[2].parse::<u16>()) {
                                (Ok(v), Ok(vn), Ok(vt)) => {
                                    vertices.push(v);
                                    textures.push(vt);
                                    normals.push(vn);
                                },
                                _ => {
                                    Err(format!("Error: Invalid syntax, faces must contain only u32."))?
                                }
                            }
                        }
                        obj.faces.push(Faces::new(
                            vertices,
                            Some(textures),
                            Some(normals),
                            current_material.clone()
                        ));
                    } else {
                        let t1 = get_simple_face(&[splited[0], splited[1], splited[2]])?;
                        obj.faces.push(Faces::new(t1, None, None,  current_material.clone()));
                    }
                    // if splited.len() == 4 {
                    //     let t2 = get_face(&[splited[0], splited[2], splited[3]])?;
                    //     obj.faces.push(Faces::new(t2, current_material.clone()));
                    // }
                },
                "s" => {
                    //TODO: Implementation
                },
                "vn" => {
                    check_line(&splited, "vn")?;
                    let v1 = get_vn(&[splited[0], splited[1], splited[2]])?;
                    obj.vn.push(v1);
                },
                "vt" => {
                    check_line(&splited, "vt")?;
                    match (splited[0].parse::<f32>(), splited[1].parse::<f32>()) {
                        (Ok(u), Ok(v)) => obj.vt.push((u, v)),
                        _ => Err(format!("Error: Invalid texture coordinates must be f32."))?
                    }
                }
                _ => return Err(format!("Error: Invalid Token {}.", key)),
            }
        } else {
            continue;
        }
    }
    match check_coherence(&obj) {
        Ok(_) => {
            obj.init_obj();
            Ok(obj)
        }
        Err(e) => Err(e),
    }
}

// fn parsing_handler(filepath: &str) -> Result<ObjParams, String> {
//     return obj_parser(filepath);
    // if filepath.to_lowercase().ends_with(".obj") {
//     } else if filepath.to_lowercase().ends_with(".tga") {
//         tga_parser(filepath);
//     } else  if filepath.to_lowercase().ends_with(".mtl") {
//         mtl_parser(filepath);
//     } else {
//         println!("Error: Unsupported file extension.")
//     }
// }
