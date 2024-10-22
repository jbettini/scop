// use std::fs::read_to_string;

// fn get_file_lines(filepath: &str) -> Vec<String> {
//     let mut lines: Vec<String>  = read_to_string(filepath).expect("Error: Cannot Open {filepath}.").lines().map(String::from).collect();
//     lines.retain(|s| !s.starts_with('#'));
//     return lines;
// }


// fn tga_parser(_filepath: &str) {
//     return 
// }

// struct ObjParams {
//     name: Option<String>,
//     mtlpath: Option<String>,
//     v: Option<Vec<[String; 2]>>,
//     f: Option<Vec<[String; 2]>>,
// }

// impl ObjParams {
//     pub fn new() -> Self {
//         Self {
//             name: None,
//             mtlpath: None,
//             v: None,
//             f: None
//         }
//     }
// }

// impl Default for ObjParams {
//     fn default() -> Self {
//         Self::new()
//     }
// }

// fn mtl_parser(_filepath: &str) {
//     return 
// }

// fn check_line(splited: Vec<String>, name: &str) -> Result<(), &str> {
//     let joined = splited.join(" ");
//     match name {
//         "usemtl" if splited.len() != 2 => Err("Error: Invalid usemtl."),
//         "v" if splited.len() != 4 => Err("Error: Invalid vertex."),
//         "f" if splited.len() < 4 || splited.len() > 5 => Err("Error: Face can contain only triangles or quadrilaterals."),
//         _ => Ok(()),
//     }
// }

// fn obj_parser(filepath: &str) -> Result<ObjParams, &str> {
//     let mut current_material = "off";
//     let lines = get_file_lines(filepath);
//     let mut obj: ObjParams = ObjParams::new(lines);
//     for line in lines {
//         let splited: Vec<String> = line.split_whitespace().map(String::from).collect();
//         if splited.is_empty() {
//             continue;
//         }
//         match splited[0].as_str() {
//             "usemtl" => {
//                 check_line(splited.clone(), "usemtl")?;
//                 current_material = splited[1].clone().as_str();
//             },
//             "v" => {
//                 check_line(splited.clone(), "v")?;

//             },
//             "f" => {
//                 check_line(splited.clone(), "f")?;
//             }
//             _ => return Err("Error: Invalid Token {_}."),
//         }
//     }
//     Ok(obj)
// }

// fn parsing_handler(filepath: &str) {

//     if filepath.to_lowercase().ends_with(".obj") {
//         obj_parser(filepath);
//     } else if filepath.to_lowercase().ends_with(".tga") {
//         tga_parser(filepath);
//     } else  if filepath.to_lowercase().ends_with(".mtl") {
//         mtl_parser(filepath);
//     } else {
//         println!("Error: Unsupported file extension.")
//     }
// }

fn main() {
    println!("{}", get_parent_path("../lol/lol"));
    // parsing_handler("../obj/42.obj");
}
