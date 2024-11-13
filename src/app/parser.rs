use std::fs::read_to_string;

use super::{
    object::Vertex,
    utils::has_duplicate,
    object::Normal,
};

#[derive(Clone, Debug)]
struct Faces {
    f: Vec<u16>,
    mtl: String
}

impl Faces {
    pub fn new(f: Vec<u16>, mtl: String) -> Self {
        Self {
            f,
            mtl
        }
    }
}
#[derive(Clone, Debug)]
pub struct ObjParams {
    pub name: Option<String>,
    pub mtlpath: Option<String>,
    pub vn: Option<Vec<Normal>>,
    pub vt: Option<Vec<Vertex>>,
    pub v: Vec<Vertex>,
    pub f: Vec<Faces>,
    pub s: String,
    pub centroid: [f32; 3]
}

impl ObjParams {
    pub fn new() -> Self {
        Self {
            name: None,
            mtlpath: None,
            v: {
                let mut v = Vec::new();
                v.push(Vertex::from_vec(&[0.0, 0.0, 0.0]).unwrap());
                v
            },
            f: Vec::new(),
            s: "off".to_string(),
            centroid: [0.0, 0.0, 0.0],
            vt: None,
            vn: None
        }
    }
    
    pub fn get_indices(self) -> Vec<u16> {
        let mut ret: Vec<u16> = Vec::new();
        for face in self.f {
            for x in face.f {
                ret.push(x);
            }
        }
        return ret;
    }

    pub fn get_normals(&self) -> Vec<Normal> {
        let mut vertex_normals: Vec<Normal> = vec![Normal::new(0.0, 0.0, 0.0); self.v.len()];
        for face in &self.f {
            let (a, b, c) = (
                self.v[face.f[0] as usize].position,
                self.v[face.f[1] as usize].position,
                self.v[face.f[2] as usize].position
            );
            let u = (b.0 - a.0, b.1 - a.1, b.2 - a.2);
            let v = (c.0 - a.0, c.1 - a.1, c.2 - a.2);
            let normal = Normal::new(
                (u.1 * v.2) - (u.2 * v.1),
                (u.2 * v.0) - (u.0 * v.2),
                (u.0 * v.1) - (u.1 * v.0)
            );
            for &index in &face.f {
                let vertex_normal = &mut vertex_normals[index as usize];
                vertex_normal.normal.0 += normal.normal.0;
                vertex_normal.normal.1 += normal.normal.1;
                vertex_normal.normal.2 += normal.normal.2;
            }
        }
        for normal in &mut vertex_normals {
            let length = (normal.normal.0.powi(2) + normal.normal.1.powi(2) + normal.normal.2.powi(2)).sqrt();
            if length > 0.0 {
                normal.normal.0 /= length;
                normal.normal.1 /= length;
                normal.normal.2 /= length;
            }
        }
        vertex_normals
    }

    fn init_centroid(& mut self) {
        let len: f32 = self.v.len() as f32;
        let mut x: f32 = 0.0;
        let mut y: f32 = 0.0;
        let mut z: f32 = 0.0;
        for vtx in self.v.clone() {
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

// fn mtl_parser(_filepath: &str) {
//     return 
// }

// fn tga_parser(_filepath: &str) {
//     return 
// }


fn get_file_lines(filepath: &str) -> Vec<String> {
    let mut lines: Vec<String>  = read_to_string(filepath).expect(format!("Error: Cannot Open {}\n.", filepath)
                                                            .as_str())
                                                            .lines()
                                                            .map(String::from)
                                                            .collect();
    lines.retain(|s| !s.starts_with('#'));
    return lines;
}

fn check_line(splited: &Vec<&str>, name: &str) -> Result<(), String> {
    let joined = splited.join(" ");
    match name {
        "usemtl" | "mtllib" | "o" if splited.len() != 1 => Err(format!("Error: Invalid format {}.", joined)),
        "v" if splited.len() != 3 => Err(format!("Error: Invalid format {}.", joined)),
        "f" if splited.len() < 3 || splited.len() > 4 => Err(format!("Error: Face can contain only triangles or quadrilaterals : {}.", joined)),
        _ => Ok(()),
    }
}

fn get_vertex(face: &[&str; 3]) -> Result<Vertex, String> {
    let mut ret: Vec<f64> = Vec::new();
    for s in face {
        let test = s.parse::<f64>();
        match test {
            Ok(ok) => {
                ret.push(ok);
                continue;
            },
            Err(_) => {
                return Err(format!("Error: Invalid vertex {}, vertex must be f64.", s));
            }
        }
    }
    Vertex::from_vec(&ret).map_err(|e| e.to_string())
}

fn get_face(face: &[&str; 3]) -> Result<Vec<u16>, String> {
    let mut ret: Vec<u16> = Vec::new();
    for s in face {
        let test = s.parse::<u16>();
        match test {
            Ok(ok) => {
                ret.push(ok);
                continue;
            },
            Err(_) => {
                return Err(format!("Error: Invalid face {}, faces must be i16.", s));
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
    let len: u16 = parsed_obj.v.len() as u16;
    for face in &parsed_obj.f {
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

pub fn obj_parser(filepath: &str) -> Result<ObjParams, String> {
    let mut current_material = "off".to_string();
    let lines = get_file_lines(filepath);
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
                    obj.v.push(v1);
                },
                "f" => {
                    check_line(&splited, "f")?;
                    let t1 = get_face(&[splited[0], splited[1], splited[2]])?;
                    obj.f.push(Faces::new(t1, current_material.clone()));
                    if splited.len() == 4 {
                        let t2 = get_face(&[splited[0], splited[2], splited[3]])?;
                        obj.f.push(Faces::new(t2, current_material.clone()));
                    }
                },
                "s" => {
                    //TODO: Implementation
                },
                "vn" => {

                },
                "vt" => {

                }
                _ => return Err(format!("Error: Invalid Token {}.", key)),
            }
        } else {
            continue;
        }
    }
    match check_coherence(&obj) {
        Ok(_) => {
            obj.init_centroid();
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

// fn main() {
//     parsing_handler("../obj/42.obj");
// }
