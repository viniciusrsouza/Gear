use crate::core::renderer::gl;
extern crate nalgebra as na;

use super::Shader;

impl Shader {
    pub fn set_bool(&self, name: &str, value: bool) {
        let name = std::ffi::CString::new(name).unwrap();
        unsafe {
            gl::Uniform1i(
                gl::GetUniformLocation(self.id, name.as_ptr() as *const i8),
                value as i32,
            );
        }
    }

    pub fn set_int(&self, name: &str, value: i32) {
        let name = std::ffi::CString::new(name).unwrap();
        unsafe {
            gl::Uniform1i(
                gl::GetUniformLocation(self.id, name.as_ptr() as *const i8),
                value,
            );
        }
    }

    pub fn set_float(&self, name: &str, value: f32) {
        let name = std::ffi::CString::new(name).unwrap();
        unsafe {
            gl::Uniform1f(
                gl::GetUniformLocation(self.id, name.as_ptr() as *const i8),
                value,
            );
        }
    }

    pub fn set_vec2(&self, name: &str, value: &na::Vector2<f32>) {
        let name = std::ffi::CString::new(name).unwrap();
        unsafe {
            gl::Uniform2fv(
                gl::GetUniformLocation(self.id, name.as_ptr() as *const i8),
                1,
                value.as_ptr(),
            );
        }
    }

    pub fn set_vec3(&self, name: &str, value: &na::Vector3<f32>) {
        let name = std::ffi::CString::new(name).unwrap();
        unsafe {
            gl::Uniform3fv(
                gl::GetUniformLocation(self.id, name.as_ptr() as *const i8),
                1,
                value.as_ptr(),
            );
        }
    }

    pub fn set_vec4(&self, name: &str, value: &na::Vector4<f32>) {
        let name = std::ffi::CString::new(name).unwrap();
        unsafe {
            gl::Uniform4fv(
                gl::GetUniformLocation(self.id, name.as_ptr() as *const i8),
                1,
                value.as_ptr(),
            );
        }
    }

    pub fn set_mat2(&self, name: &str, value: &na::Matrix2<f32>) {
        let name = std::ffi::CString::new(name).unwrap();
        unsafe {
            gl::UniformMatrix2fv(
                gl::GetUniformLocation(self.id, name.as_ptr() as *const i8),
                1,
                gl::FALSE,
                value.as_ptr(),
            );
        }
    }

    pub fn set_mat3(&self, name: &str, value: &na::Matrix3<f32>) {
        let name = std::ffi::CString::new(name).unwrap();
        unsafe {
            gl::UniformMatrix3fv(
                gl::GetUniformLocation(self.id, name.as_ptr() as *const i8),
                1,
                gl::FALSE,
                value.as_ptr(),
            );
        }
    }

    pub fn set_mat4(&self, name: &str, value: &na::Matrix4<f32>) {
        let name = std::ffi::CString::new(name).unwrap();
        unsafe {
            gl::UniformMatrix4fv(
                gl::GetUniformLocation(self.id, name.as_ptr() as *const i8),
                1,
                gl::FALSE,
                value.as_ptr(),
            );
        }
    }
}
