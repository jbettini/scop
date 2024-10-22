use std::fs::read_to_string;

fn get_file_lines(filepath: &str) -> Vec<String> {
    let mut lines: Vec<String>  = read_to_string(filepath) 
                                    .expect("Error: Cannot Open {filepath}.")
                                    .lines()
                                    .map(String::from)
                                    .collect();
    lines.retain(|s| !s.starts_with('#'));
    return lines;
}

fn mtl_parser(_filepath: &str) {
    return 
}

fn tga_parser(_filepath: &str) {
    return 
}

fn obj_parser(filepath: &str) {
    println!("{:?}", get_file_lines(filepath));
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

fn main() {
    parsing_handler("../obj/42.obj");
}
