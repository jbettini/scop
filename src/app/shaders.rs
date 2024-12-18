use super::ctx::Ctx;

pub struct Shader {
    pub vertex_shader: &'static str,
    pub fragment_shader: &'static str,
}

impl Shader {
    pub fn new() -> Self {
        Self {
            vertex_shader: r#"
            #version 330
                in vec3 position;
                in vec3 normal;
                in vec2 tex_coords;
                in int id;
                
                out vec2 v_tex_coords;
                out vec3 v_normal;
                out vec3 v_position;
                flat out int v_id; 

                uniform mat4 rotation_matrix;
                uniform mat4 perspective_matrix;
                uniform vec3 object_center;

                void main() {
                    v_id = id;
                    v_tex_coords = tex_coords;
                    v_normal = normalize(transpose(inverse(mat3(rotation_matrix))) * normal);
                    v_position = vec3(rotation_matrix * vec4(position, 1.0));
                    vec3 centered_position = position - object_center;
                    vec4 rotated_position = rotation_matrix * vec4(centered_position, 1.0);
                    vec3 final_position = vec3(rotated_position) + object_center;
                    gl_Position = perspective_matrix * vec4(final_position, 1.0);
                }
            "#,
            fragment_shader: r#"
            #version 330
                flat in int v_id;
                in vec2 v_tex_coords;


                out vec4 color;

                uniform sampler2D diffuse_texture;
                uniform float mix_factor;

                void main() {
                    vec3 texture_color = texture(diffuse_texture, v_tex_coords).rgb;

                    float gray_levels[4] = float[4](0.2, 0.4, 0.6, 0.8);

                    int index = v_id % 4;
                    float gray = gray_levels[index];
                    vec3 gray_vec = vec3(gray, gray, gray);
                    vec3 regular_color = mix(gray_vec, texture_color, mix_factor);

                    color = vec4(regular_color, 1.0);
                }
            "#,
        }
    }

    pub fn switch_shading(&mut self, ctx: &mut Ctx) {
        // println!("{:?}", ctx.shading);
        if ctx.shading == 0 {
            self.fragment_shader = r#"
            #version 330
                in vec3 v_normal;
                in vec2 v_tex_coords;

                out vec4 color;

                uniform vec3 light;
                uniform sampler2D diffuse_texture;
                uniform float mix_factor;

                void main() {
                    float brightness = dot(normalize(v_normal), normalize(light));
                    vec3 dark_color = vec3(0.0, 0.05, 0.05);

                    vec3 texture_color = texture(diffuse_texture, v_tex_coords).rgb;
                    vec3 default_color = vec3(0.0, 1.0, 1.0);
                    vec3 regular_color = mix(default_color, texture_color, mix_factor);

                    color = vec4(mix(dark_color, regular_color, brightness), 1.0);
                }
            "#;
            ctx.shading = 1;
        } else if ctx.shading == 1 {
            self.fragment_shader = r#"
                #version 330
                in vec3 v_normal;
                in vec3 v_position;
                in vec2 v_tex_coords;

                out vec4 color;

                uniform vec3 light;
                uniform sampler2D diffuse_texture;
                uniform float mix_factor;
                
                void main() {

                    vec3 diffuse_color = vec3(0.0, 0.6, 0.6);
                    vec3 specular_color = vec3(1.0, 1.0, 1.0);
                    vec3 texture_color = texture(diffuse_texture, v_tex_coords).rgb;
                    
                    vec3 regular_color = mix(diffuse_color, texture_color, mix_factor);
                    vec3 ambient_color = regular_color * 0.3;
                    
                    float diffuse = max(dot(v_normal, light), 0.0);

                    vec3 camera_dir = -v_position;
                    vec3 half_direction = normalize(light + camera_dir);
                    float specular = pow(max(dot(v_normal, half_direction), 0.0), 16.0);
                    color = vec4(ambient_color + diffuse * regular_color + specular * specular_color, 1.0);
                }
            "#;
            ctx.shading = 2;
        } else if ctx.shading == 2 {
            *self = Shader::new();
            ctx.shading = 0;
        }
    }
}

impl Default for Shader {
    fn default() -> Self {
        Self::new()
    }
}
