use std::{collections::HashMap};

use ::glmath::glmath::Vec2f;
use glmath::glmath::{Vec3f, Vec4f, Mat22f, Mat33f, Mat44f};
use ogl33::*;

#[repr(u32)]
#[derive(Default, PartialEq, PartialOrd)]
pub enum ShaderType {
    #[default]
    Vertex = GL_VERTEX_SHADER,
    Fragment = GL_FRAGMENT_SHADER
}

#[derive(Default)]
struct Shader {
    shader_id: GLuint
}

// Do some shady stuff to compile the shader, it's safe, I promise.
// This kind of thing isn't too common in open gl functions im sure.
fn compile_shader(shader_data: &str, shader_type: ShaderType) -> u32 {
    unsafe {
        let shader_id = glCreateShader(shader_type as u32);

        // Create a double pointer to the shader text data.
        let shader_text: [*const i8; 1] = [
            shader_data.as_ptr() as *const i8
        ];

        let shader_length: [i32; 1] = [
            shader_data.len() as i32
        ];

        glShaderSource(shader_id, 1, 
            shader_text.as_ptr(), shader_length.as_ptr());
        glCompileShader(shader_id);

        shader_id
    }
}

impl Shader {
    fn open(file_name: &str, shader_type: ShaderType) -> Result<Shader, Box<dyn std::error::Error>> {
        let shader_data = std::fs::read_to_string(file_name)?;
        let shader_id = compile_shader(shader_data.as_str(), shader_type);

        Ok(Shader {
            shader_id
        })
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            glDeleteShader(self.shader_id);
        }
    }
}

#[derive(Default)]
/// Holds the entire shader program. Stores the sub-shaders as dependents 
/// so they aren't dropped too early.
pub struct ShaderProgram {
    program_id: GLuint,
    attribute_locations: HashMap<String, i32>,
    current_attribute_location: i32
}

impl ShaderProgram {
    pub fn open_shaders(vertex_shader_path: &str, fragment_shader_path: &str) -> Result<ShaderProgram, Box<dyn std::error::Error>> {
        unsafe {
            let vert_shader = Shader::open(vertex_shader_path, ShaderType::Vertex)?;
            let frag_shader = Shader::open(fragment_shader_path, ShaderType::Fragment)?;

            let program_id = glCreateProgram();

            // If we successfully loaded both shaders, attach them.
            glAttachShader(program_id, vert_shader.shader_id);
            glAttachShader(program_id, frag_shader.shader_id);
            glLinkProgram(program_id);
            
            Ok(ShaderProgram { 
                program_id,
                attribute_locations: HashMap::<String, i32>::default(),
                current_attribute_location: 0
            })

            // Here, the two shaders should be dropped and freed.
        }
    }

    pub fn bind(&self) {
        unsafe {
            glUseProgram(self.program_id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            glUseProgram(0);
        }
    }

    /// Creates a new attribute location and returns it for later use.
    pub fn add_attribute(&mut self, attribute_name: &str) -> i32 {
        #[cfg(debug_assertions)]
        if self.attribute_locations.contains_key(attribute_name) {
            dbg!("Attribute already exists in shader {}", attribute_name);
            return -1;
        }

        self.attribute_locations.insert(attribute_name.to_string(), self.current_attribute_location);

        // Again with this nonsense of converting a u8 ptr to an i8 ptr.
        unsafe {
            glBindAttribLocation(self.program_id, 
                self.current_attribute_location as u32, 
                attribute_name.as_ptr() as *const i8);
        }

        self.current_attribute_location += 1;

        // Give the new locationn back to the caller.
        self.current_attribute_location - 1
    }

    pub fn get_attribute_location(&self, name: &str) -> Option<&i32> {
        self.attribute_locations.get(name)
    }
}

/// Make sure to free the shader from graphics memory before destroying the object.
impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe {
            glDeleteProgram(self.program_id);
        }
    }
}

/// Uniform controllers for clients to use from the shader. This encapsulates most
/// of the shader we don't want implementers to touch.
pub trait ShaderUniforms {
    /// Returns the location of the uniform by name.
    fn get_uniform_location(&self, uniform_name: &str) -> i32;

    // Floats
    fn load_float(&self, location: i32, value: f32);
    fn load_vec2(&self, location: i32, value: Vec2f);
    fn load_vec3(&self, location: i32, value: Vec3f);
    fn load_vec4(&self, location: i32, value: Vec4f);

    // Integers
    fn load_int(&self, location: i32, value: i32);

    // Arrays
    fn load_float_array(&self, location: i32, value: Vec<f32>);
    fn load_integer_array(&self, location: i32, value: Vec<i32>);

    // Matrix
    fn load_matrix22(&self, location: i32, value: Mat22f);
    fn load_matrix33(&self, location: i32, value: Mat33f);
    fn load_matrix44(&self, location: i32, value: Mat44f);
}

impl ShaderUniforms for ShaderProgram {
    fn get_uniform_location(&self, uniform_name: &str) -> i32 {
        unsafe {
            let name_as_c_str = std::ffi::CString::new(uniform_name);

            return match name_as_c_str {
                Ok(uniform_name) => {
                    let loc = glGetUniformLocation(self.program_id as u32, 
                        uniform_name.as_ptr() as *const i8);
                    loc 
                },
                Err(_) => -1
            }
        }
    }

    fn load_float(&self, location: i32, value: f32) {
        unsafe {
            glUniform1f(location, value);
        }
    }

    fn load_vec2(&self, location: i32, value: Vec2f) {
        unsafe {
            glUniform2f(location, value.x, value.y);
        }
    }

    fn load_vec3(&self, location: i32, value: Vec3f) {
        unsafe {
            glUniform3f(location, value.x, value.y, value.z);
        }
    }

    fn load_vec4(&self, location: i32, value: Vec4f) {
        unsafe {
            glUniform4f(location, value.x, value.y, value.z, value.w);
        }
    }

    fn load_int(&self, location: i32, value: i32) {
        unsafe {
            glUniform1i(location, value);
        }
    }

    fn load_float_array(&self, location: i32, value: Vec<f32>) {
        unsafe {
            glUniform1fv(location, value.len() as i32, value.as_ptr());
        }
    }

    fn load_integer_array(&self, location: i32, value: Vec<i32>) {
        unsafe {
            glUniform1iv(location, value.len() as i32, value.as_ptr());
        }
    }

    fn load_matrix22(&self, location: i32, value: Mat22f) {
        unsafe {
            glUniformMatrix2fv(location, 1, GL_FALSE, 
                value.data.as_ptr() as *const f32);
        }
    }

    fn load_matrix33(&self, location: i32, value: Mat33f) {
        unsafe {
            glUniformMatrix3fv(location, 1, GL_FALSE, 
                value.data.as_ptr() as *const f32);
        }
    }

    fn load_matrix44(&self, location: i32, value: Mat44f) {
        unsafe {
            glUniformMatrix4fv(location, 1, GL_FALSE, 
                value.data.as_ptr() as *const f32);
        }
    }
}