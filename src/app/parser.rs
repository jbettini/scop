use std::fs::read_to_string;

fn get_file_lines(filepath: &str) -> Vec<String> {
    let mut lines: Vec<String>  = read_to_string(filepath).expect("Error: Cannot Open {filepath}.").lines().map(String::from).collect();
    lines.retain(|s| !s.starts_with('#'));
    return lines;
}


fn tga_parser(_filepath: &str) {
    return 
}

struct ObjParams {
    name: Option<String>,
    mtlpath: Option<String>,
    v: Option<Vec<String>>,
    f: Option<Vec<[String; 2]>>,
    s: String
}

impl ObjParams {
    pub fn new() -> Self {
        Self {
            name: None,
            mtlpath: None,
            v: None,
            f: None,
            s: "off".to_string()
        }
    }
}

impl Default for ObjParams {
    fn default() -> Self {
        Self::new()
    }
}

fn mtl_parser(_filepath: &str) {
    return 
}

fn check_line(splited: Vec<String>, name: &str) -> Result<(), String> {
    let joined = splited.join(" ");
    match name {
        "usemtl" | "mtllib" | "o" if splited.len() != 2 => Err(format!("Error: Invalid format {}.", joined)),
        "v" if splited.len() != 4 => Err(format!("Error: Invalid format {}.", joined)),
        "f" if splited.len() < 4 || splited.len() > 5 => Err(format!("Error: Face can contain only triangles or quadrilaterals : {}.", joined)),
        _ => Ok(()),
    }
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

fn obj_parser(filepath: &str) -> Result<ObjParams, String> {
    let mut current_material = "off";
    let lines = get_file_lines(filepath);
    let mut obj: ObjParams = ObjParams::new();
    for line in lines {
        let splited: Vec<String> = line.split_whitespace().map(String::from).collect();
        if splited.is_empty() {
            continue;
        }
        match splited[0].as_str() {
            "mtllib" => {
                if obj.mtlpath == None {
                    check_line(splited.clone(), "mtllib")?;
                    let parent = get_parent_path(filepath)?;
                    obj.mtlpath = Some(format!("{}/{}", parent, splited[1]));
                } else {
                    return Err(format!("Error: mtllib cannot be set twice {}.", line));
                }
            },
            "o" => {
                if obj.name == None {
                    check_line(splited.clone(), "o")?;
                    obj.name = Some(splited[1].clone());
                } else {
                    return Err(format!("Error: object name cannot be set twice {}.", line));
                }

            },
            "usemtl" => {
                check_line(splited.clone(), "usemtl")?;
                current_material = splited[1].clone().as_str();
            },
            "v" => {
                check_line(splited.clone(), "v")?;
                
            },
            "f" => {
                check_line(splited.clone(), "f")?;
            }
            _ => return Err(format!("Error: Invalid Token {}.", splited[0])),
        }
    }
    Ok(obj)
}

fn parsing_handler(filepath: &str) {
    if filepath.to_lowercase().ends_with(".obj") {
        obj_parser(filepath);
    } else if filepath.to_lowercase().ends_with(".tga") {
        tga_parser(filepath);
    } else  if filepath.to_lowercase().ends_with(".mtl") {
        mtl_parser(filepath);
    } else {
        println!("Error: Unsupported file extension.")
    }
}

// fn main() {
//     parsing_handler("../obj/42.obj");
// }
