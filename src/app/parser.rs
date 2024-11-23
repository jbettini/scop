use super::vec::{Normal, Normalize};

use std::fs::read_to_string;

#[derive(Clone, Debug)]
pub struct Face {
    pub v: [u32; 3],
    pub vn: [u32; 3],
    pub vt: [u32; 3],
    pub mtl: String
}

impl Face {
    pub fn new(
        v: [u32; 3],
        vn: [u32; 3],
        vt: [u32; 3],
        mtl: String) -> Self 
    {
        Self { v, vt, vn, mtl }
    }
    pub fn from_vvnvt(vvnvt: Vec<[u32; 3]>, mtl: String) -> Self {
        Self { 
            v: [vvnvt[0][0], vvnvt[1][0], vvnvt[2][0]],
            vn: [vvnvt[0][1], vvnvt[1][1], vvnvt[2][1]],
            vt: [vvnvt[0][2], vvnvt[1][2], vvnvt[2][2]],
            mtl
        }
    }
}

#[derive(Clone, Debug)]
pub struct Obj {
    pub s: String,
    pub name: Option<String>,
    pub mtlpath: Option<String>,
    
    pub vertexs: Vec<[f32; 3]>,
    pub vn: Vec<[f32; 3]>,
    pub vt: Vec<[f32; 2]>,
    pub faces: Vec<Face>,

    pub centroid: [f32; 3],
}

impl Obj {
    pub fn new() -> Self {
        Self {
            s: "off".to_string(),
            name: None,
            mtlpath: None,

            vertexs: vec!([0.0, 0.0, 0.0]),
            vn: vec!([0.0, 0.0, 0.0]),
            vt: vec!([0.0, 0.0]),
            faces: Vec::new(),

            centroid: [0.0, 0.0, 0.0],
        }
    }

    fn get_faces_normals(& mut self) -> Vec<[f32; 3]> {
        let mut faces_normals = vec![[0.0, 0.0, 0.0]];
        for face in &self.faces {
            let (a, b, c) = (
                self.vertexs[face.v[0] as usize],
                self.vertexs[face.v[1] as usize],
                self.vertexs[face.v[2] as usize]
            );
            faces_normals.push(a.calc_face_normal(b, c));
        }
        return faces_normals;
    }

    pub fn calculate_vertex_normals(&mut self) -> Vec<[f32; 3]>{
        let mut ret: Vec<[f32; 3]> = vec![[0.0, 0.0, 0.0]; self.vertexs.len()];
        let faces_normals = self.get_faces_normals();
        
        for (face_index, face) in self.faces.iter().enumerate() {
            let normal = &faces_normals[face_index + 1];
            for vertex_index in &face.v {
                let vertex_normal = &mut ret[*vertex_index as usize];
                vertex_normal[0] += normal[0];
                vertex_normal[1] += normal[1];
                vertex_normal[2] += normal[2];
            }
        }
        ret.normalize();
        return ret;
    }

    fn init_centroid(& mut self) {
        let len: f32 = self.vertexs.len() as f32;
        let mut x: f32 = 0.0;
        let mut y: f32 = 0.0;
        let mut z: f32 = 0.0;
        for vtx in &self.vertexs {
            x += vtx[0];
            y += vtx[1];
            z += vtx[2];
        }
        self.centroid = [x / len, y / len, z / len];
    }

}

impl Default for Obj {
    fn default() -> Self {
        Self::new()
    }
}

// pub fn check_coherence(parsed_obj: &Obj) -> Result<(), String> {
//     let vlen: u32 = parsed_obj.vertexs.len() as u32;
//     let vnlen: u32 = parsed_obj.vn.len() as u32;
//     if vlen <= 0 || vlen >= 100000{
//         return Err(format!("Error: vertexs must be between 1 and 1e6."));
//     } else {
//         for face in &parsed_obj.faces {
//             for v in &face.f {
//                 if *v <= 0 || *v > vlen {
//                     return Err(format!("Error: A face is out of the vertex range."));
//                 }
//             }
//             if vnlen != 0 {
//                 for vn in &face.vn {
//                     if *vn <= 0 || *vn > vnlen {
//                         return Err(format!("Error: A vn is out of the vertex normals range."));
//                     }
//                 }
//             }
//             // TODO add check for vt
//             if has_duplicate(&face.f) {
//                 return Err(format!("Error: A face contain duplicate vertex."));
//             }
//         }
//         Ok(())
//     }
// }

fn get_file_lines(filepath: &str) -> Result<Vec<String>, String> {
    let content = read_to_string(filepath).map_err(|e| format!("Error: Cannot open {}. {}", filepath, e))?;
    let mut lines: Vec<String> = content
        .lines()
        .map(String::from)
        .collect();
    
    lines.retain(|s| !s.starts_with('#'));
    Ok(lines)
}

fn get_v(v: &[&str; 3]) -> Result<[f32; 3], String> {
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
    Ok([ret[0], ret[1], ret[2]])
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

pub fn obj_parser(filepath: &str) -> Result<Obj, String> {
    let mut current_material = "off".to_string();
    let lines = get_file_lines(filepath)?;
    let mut obj: Obj = Obj::new();
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
                    obj.vertexs.push(v);
                },
                "f" => {
                    if splited.len() < 3 || splited.len() > 4 {
                        return Err(format!("Error: Face can contain only triangles or quadrilaterals : {} {:?}.",key, splited));
                    }
                    let face_args = triangulize(splited);
                    for vec in face_args {
                        let mut vvnvt: Vec<[u32; 3]> = Vec::new();
                        for args in vec {
                            let parts: Vec<&str> = args.split('/').collect();
                            match parts.len() {
                                3 => {
                                    if let (Ok(v), Ok(vt), Ok(vn)) = (parts[0].parse::<u32>(), parts[1].parse::<u32>(), parts[2].parse::<u32>()) {
                                        vvnvt.push([v, vn, vt]);
                                    } else {
                                        return Err(format!("Error: Invalid face indices in '{}'", args));
                                    }
                                },
                                1 => {
                                    if let Ok(v) = parts[0].parse::<u32>() {
                                        vvnvt.push([v, 0, 0]);
                                    } else {
                                        return Err(format!("Error: Invalid vertex index in '{}'", args));
                                    }
                                },
                                _ => return Err(format!("Error: Invalid face format in '{}'", args))
                            }
                        }
                        obj.faces.push(Face::from_vvnvt(vvnvt, current_material.clone()));
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
                    obj.vn.push(v);
                },
                "vt" => {
                    if splited.len() != 2 {
                        return Err(format!("Error: Invalid format : {} {:?}.", key, splited));
                    }
                    match (splited[0].parse::<f32>(), splited[1].parse::<f32>()) {
                        (Ok(u), Ok(v)) => obj.vt.push([u, v]),
                        _ => Err(format!("Error: Invalid texture coordinates must be f32."))?
                    }
                }
                _ => return Err(format!("Error: Invalid Token {}.", key)),
            }
        }
    }
    // if let Err(error) = check_coherence(&obj) {
    //     return Err(format!("{}", error));
    // }
    obj.init_centroid();
    Ok(obj)
}

// TODO add this for drag and drop
// fn parsing_handler(filepath: &str) -> Result<Obj, String> {
//     return obj_parser(filepath);
    // if filepath.to_lowercase().ends_with(".obj") {
//     } else if filepath.to_lowercase().ends_with(".tga") {
//         tga_parser(filepath);
//     } else {
//         println!("Error: Unsupported file extension.")
//     }
// }
