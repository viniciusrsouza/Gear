mod uniform;

use std::ffi::CString;

use crate::core::entity;

use self::gl::types::{GLchar, GLint};
use super::gl;

pub struct Shader {
    id: u32,
}

impl Shader {
    pub fn new(vertex_path: &str, fragment_path: &str) -> Result<Self, ShaderError> {
        let mut shader = Self { id: 0 };
        shader.compile(vertex_path, fragment_path)?;

        Ok(shader)
    }

    fn compile(&mut self, vertex_path: &str, fragment_path: &str) -> Result<(), ShaderError> {
        let vertex_shader = unsafe { compile_shader(vertex_path, gl::VERTEX_SHADER) }?;
        let fragment_shader = unsafe { compile_shader(fragment_path, gl::FRAGMENT_SHADER) }?;

        self.id = unsafe { compile_program(vertex_shader, fragment_shader)? };

        Ok(())
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn with_transform(&self, transformation: &entity::Transform) {
        unsafe {
            gl::UseProgram(self.id);

            let model = transformation.get_model_matrix();
            self.set_mat4("model", &model);
        }
    }

    pub fn with_material(&self, material: &entity::Material) {
        unsafe {
            gl::UseProgram(self.id);
            self.set_vec4("material", &material.color);
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}

#[derive(Debug)]
pub enum ShaderError {
    ShaderCompilationFailed(String),
    ShaderProgramLinkFailed(String),
    ShaderFileNotFound(String),
}

unsafe fn compile_shader(path: &str, shader_type: gl::types::GLenum) -> Result<u32, ShaderError> {
    let source = std::fs::read_to_string(&path)
        .map_err(|_| ShaderError::ShaderFileNotFound(path.to_string()))?;
    let shader = gl::CreateShader(shader_type);

    let source = CString::new(source.as_bytes()).unwrap();

    gl::ShaderSource(shader, 1, &source.as_ptr(), std::ptr::null());
    gl::CompileShader(shader);

    let mut success = gl::FALSE as GLint;
    let mut info_log = Vec::with_capacity(512);

    gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);

    if success != (gl::TRUE as GLint) {
        gl::GetShaderInfoLog(
            shader,
            512,
            std::ptr::null_mut(),
            info_log.as_mut_ptr() as *mut GLchar,
        );
        return Err(ShaderError::ShaderCompilationFailed(
            std::str::from_utf8(&info_log).unwrap().to_string(),
        ));
    }

    Ok(shader)
}

unsafe fn compile_program(vertex_shader: u32, fragment_shader: u32) -> Result<u32, ShaderError> {
    let program = gl::CreateProgram();

    gl::AttachShader(program, vertex_shader);
    gl::AttachShader(program, fragment_shader);
    gl::LinkProgram(program);

    let mut success = gl::FALSE as GLint;
    let mut info_log = Vec::with_capacity(512);

    gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);

    if success != (gl::TRUE as GLint) {
        gl::GetProgramInfoLog(
            program,
            512,
            std::ptr::null_mut(),
            info_log.as_mut_ptr() as *mut GLchar,
        );
        return Err(ShaderError::ShaderProgramLinkFailed(
            std::str::from_utf8(&info_log).unwrap().to_string(),
        ));
    }

    gl::DeleteShader(vertex_shader);
    gl::DeleteShader(fragment_shader);

    Ok(program)
}
