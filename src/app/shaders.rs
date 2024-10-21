use super::ctx::Ctx;

pub struct Shader {
    pub vertex_shader: &'static str,
    pub fragment_shader: &'static str
}

impl Shader {
    pub fn new() -> Self {
        Self {
            vertex_shader: r#"
                #version 330
                in vec3 position;
                in vec3 normal;

                out vec3 v_normal;
                out vec3 v_position; 
                uniform mat4 rotation_matrix;
                uniform mat4 perspective_matrix;

                void main() {
                    v_normal = transpose(inverse(mat3(rotation_matrix))) * normal; 
                    v_position = vec3(rotation_matrix * vec4(position, 1.0));
                    gl_Position = perspective_matrix * rotation_matrix * vec4(position, 1.0);
                }
            "#,
            fragment_shader: r#"
                #version 330
                in vec3 v_normal;
                in vec3 v_position;

                out vec4 color;

                uniform vec3 light;

                const vec3 ambient_color = vec3(0.0, 0.2, 0.2);
                const vec3 diffuse_color = vec3(0.0, 0.6, 0.6);
                const vec3 specular_color = vec3(1.0, 1.0, 1.0);

                void main() {
                    float diffuse = max(dot(normalize(v_normal), normalize(light)), 0.0);

                    vec3 camera_dir = normalize(-v_position);
                    vec3 half_direction = normalize(normalize(light) + camera_dir);
                    float specular = pow(max(dot(normalize(v_normal), half_direction), 0.0), 16.0);

                    color = vec4(ambient_color + diffuse * diffuse_color + specular * specular_color, 1.0);
                }
            "#
        }
    }
    pub fn switch_shading(&mut self, ctx: &mut Ctx) {
        if ctx.shading {
            self.fragment_shader = r#"
                    #version 330
                    in vec3 v_normal;
                    out vec4 color;
                    uniform vec3 light;
        
                    void main() {
                        float brightness = dot(normalize(v_normal), normalize(light));
                        vec3 dark_color = vec3(0.0, 0.45, 0.45);
                        vec3 regular_color = vec3(0.0, 1.0, 1.0);
                        color = vec4(mix(dark_color, regular_color, brightness), 1.0);
                    }
                "#;
        } else {
            *self = Shader::new();
        }
        ctx.shading = !ctx.shading;
    }
}

impl Default for Shader {
    fn default() -> Self {
        Self::new()
    }
}
