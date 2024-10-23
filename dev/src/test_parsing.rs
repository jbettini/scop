use std::collections::HashSet;

pub fn has_duplicate(f: &Vec<u64>) -> bool {
    let mut tmp:  HashSet<u64> = HashSet::new();
    for x in f {
        if tmp.contains(&x) {
            return true;
        }
        tmp.insert(x.clone());
    }
    return false;
}

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: (f32, f32, f32),
    // pub tex_coords: [f32; 2]
}

impl Vertex {
    pub fn from_vec(vec: &[f64]) -> Result<Self, &'static str> {
        if vec.len() != 3 {
            return Err("Error: Invalid argument: Vertex::from_vec requires exactly 3 elements.");
        }
        Ok(Self {
            position: (vec[0] as f32, vec[1] as f32, vec[2] as f32),
        })
    }
}

//----------------------------->
//----------------------------->
//----------------------------->
//----------------------------->
//----------------------------->
//----------------------------->
use std::fs::read_to_string;

#[derive(Debug)]
struct Faces {
    f: Vec<u64>,
    mtl: String
}

impl Faces {
    pub fn new(f: Vec<u64>, mtl: String) -> Self {
        Self {
            f,
            mtl
        }
    }
}

#[derive(Debug)]
struct ObjParams {
    name: Option<String>,
    mtlpath: Option<String>,
    v: Vec<Vertex>,
    f: Vec<Faces>,
    s: String
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
            s: "off".to_string()
        }
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
    let mut lines: Vec<String>  = read_to_string(filepath).expect("Error: Cannot Open {filepath}.").lines().map(String::from).collect();
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

fn get_face(face: &[&str; 3]) -> Result<Vec<u64>, String> {
    let mut ret: Vec<u64> = Vec::new();
    for s in face {
        let test = s.parse::<u64>();
        match test {
            Ok(ok) => {
                ret.push(ok);
                continue;
            },
            Err(_) => {
                return Err(format!("Error: Invalid face {}, faces must be i64.", s));
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
    let len: u64 = parsed_obj.v.len() as u64;
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

                }
                _ => return Err(format!("Error: Invalid Token {}.", key)),
            }
        } else {
            continue;
        }
    }
    match check_coherence(&obj) {
        Ok(_) => Ok(obj),
        Err(e) => Err(e),
    }
}

fn parsing_handler(filepath: &str) -> Result<ObjParams, String> {
    match obj_parser(filepath) {
        Ok(obj) => {
            println!("Parsing successful!");
            println!("Object name: {:?}", obj.name);
            println!("Number of vertices: {}", obj.v.len());
            println!("Number of faces: {}", obj.f.len());
            
            // Afficher quelques vertices (par exemple, les 5 premiers)
            println!("First 5 vertices:");
            for vertex in obj.v.iter().take(5) {
                println!("{:?}", vertex);
            }
            
            // Afficher quelques faces (par exemple, les 5 premières)
            println!("First 5 faces:");
            for face in obj.f.iter().take(5) {
                println!("{:?}", face);
            }
            
            // Afficher le chemin du fichier MTL si présent
            if let Some(mtl_path) = obj.mtlpath {
                println!("MTL file: {}", mtl_path);
            } else {
                println!("No MTL file specified");
            }
        },
        Err(e) => {
            eprintln!("Error parsing file: {}", e);
        }
    }
    Err(format!("temp"))
}
    // if filepath.to_lowercase().ends_with(".obj") {
    // } else if filepath.to_lowercase().ends_with(".tga") {
    //     tga_parser(filepath);
    // } else  if filepath.to_lowercase().ends_with(".mtl") {
    //     mtl_parser(filepath);
    // } else {
    //     println!("Error: Unsupported file extension.")
    // }

fn main() {
    parsing_handler("../obj/test.obj");
}

