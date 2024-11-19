use super::parser::ObjParams;
use super::utils::has_duplicate;


// pub struct ObjParams {
//     pub s: String,
//     pub name: Option<String>,
//     pub mtlpath: Option<String>,
//     pub vertexs: Vec<Vertex>,
//     pub indices: Vec<u32>,
//     pub faces_normals: Vec<Normal>,
//     pub vn: Vec<Normal>,
//     pub vt: Vec<(f32, f32)>,
//     pub vertex_normals: Vec<Normal>,
//     pub vertex_textures: Vec<(f32, f32)>,
//     pub centroid: [f32; 3],
//     pub faces: Vec<Face>
// }

// pub fn checker

pub fn check_coherence(parsed_obj: &ObjParams) -> Result<(), String> {
    let len: u32 = parsed_obj.vertexs.len() as u32;
    if len <= 0 || len >= 100000{
        return Err(format!("Error: No vertexs."));
    } else {
        // check 
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