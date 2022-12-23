use crate::platform::{WinType, WindowApi};

use super::Renderer;

mod gl {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

static VERTEX_SHADER_SRC: &str = "#version 330 core

layout (location = 0) in vec3 aPos;
void main()

{
   gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
}\0
";

static FRAGMENT_SHADER_SRC: &str = "#version 330 core
out vec4 FragColor;

void main()
{
    FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
}\0
";

pub struct OpenGlRenderer {
    vbo: u32,
    vao: u32,
    ebo: u32,
    program: u32,
}

impl OpenGlRenderer {
    pub fn new() -> Self {
        OpenGlRenderer {
            vbo: 0,
            vao: 0,
            ebo: 0,
            program: 0,
        }
    }
}

impl Renderer for OpenGlRenderer {
    fn init(&mut self, window: &mut WinType) {
        gl::load_with(|name| window.get_proc_address(name));
        unsafe {
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(
                vertex_shader,
                1,
                &(VERTEX_SHADER_SRC.as_ptr() as *const i8),
                std::ptr::null(),
            );
            gl::CompileShader(vertex_shader);

            let mut success = 0;
            let mut info_log = [0; 512];
            gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
            if success != 1 {
                gl::GetShaderInfoLog(
                    vertex_shader,
                    512,
                    std::ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut i8,
                );
                panic!(
                    "Vertex shader compilation failed: {}",
                    std::str::from_utf8(&info_log).unwrap()
                );
            }

            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(
                fragment_shader,
                1,
                &(FRAGMENT_SHADER_SRC.as_ptr() as *const i8),
                std::ptr::null(),
            );
            gl::CompileShader(fragment_shader);

            self.program = gl::CreateProgram();
            gl::AttachShader(self.program, vertex_shader);
            gl::AttachShader(self.program, fragment_shader);
            gl::LinkProgram(self.program);

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            gl::ClearColor(0.3, 0.3, 0.5, 1.0);
            gl::GenVertexArrays(1, &mut self.vao);
            gl::GenBuffers(1, &mut self.vbo);
            gl::GenBuffers(1, &mut self.ebo);

            let vertices: [f32; 12] = [
                0.5, 0.5, 0.0, // top right
                0.5, -0.5, 0.0, // bottom right
                -0.5, -0.5, 0.0, // bottom left
                -0.5, 0.5, 0.0, // top left
            ];

            gl::BindVertexArray(self.vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                &vertices[0] as *const f32 as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );

            let indices: [u32; 6] = [0, 1, 3, 1, 2, 3];

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, self.ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr,
                &indices[0] as *const u32 as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                3 * std::mem::size_of::<f32>() as gl::types::GLint,
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(0);

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
    }

    fn render(&mut self) {
        unsafe {
            gl::ClearColor(0.3, 0.3, 0.5, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::UseProgram(self.program);
            gl::BindVertexArray(self.vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }
    }
}
