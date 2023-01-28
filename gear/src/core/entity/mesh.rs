use crate::core::renderer::gl;

#[derive(Debug)]
pub struct Mesh {
    pub vertices: Vec<f32>,
    pub indices: Vec<u32>,

    vao: u32,
    vbo: u32,
    ebo: u32,
}

impl Mesh {
    pub fn new(vertices: Vec<f32>, indices: Vec<u32>) -> Self {
        Self {
            vertices,
            indices,
            vao: 0,
            vbo: 0,
            ebo: 0,
        }
    }

    pub fn get_id(&self) -> u32 {
        self.vao
    }

    pub fn init(&mut self) {
        let mut vao = 0;
        let mut vbo = 0;
        let mut ebo = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);

            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (self.vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                &self.vertices[0] as *const f32 as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (self.indices.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr,
                &self.indices[0] as *const u32 as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );

            // Position attribute
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                6 * std::mem::size_of::<f32>() as i32,
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(0);

            // Normal attribute
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                6 * std::mem::size_of::<f32>() as i32,
                (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid,
            );
            gl::EnableVertexAttribArray(1);

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        self.vao = vao;
        self.vbo = vbo;
        self.ebo = ebo;
    }
}

impl Drop for Mesh {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteBuffers(1, &self.ebo);
        }
    }
}

// temporary shapes

#[rustfmt::skip]
pub fn square() -> Mesh {
    let vertices = vec![
        -0.5,  0.5, 0.0, 0.0, 0.0, 1.0, // bottom left
        -0.5, -0.5, 0.0, 0.0, 0.0, 1.0, // bottom right
         0.5,  0.5, 0.0, 0.0, 0.0, 1.0, // top right
         0.5, -0.5, 0.0, 0.0, 0.0, 1.0, // top left
    ];

    let indices = vec![0, 1, 2, 2, 3, 1];

    Mesh::new(vertices, indices)
}

#[rustfmt::skip]
pub fn cube() -> Mesh {
    let vertices = vec![
         0.5,  0.5,  0.5, 1.0, 0.0, 0.0,
         0.5,  0.5, -0.5, 1.0, 0.0, 0.0,
         0.5, -0.5,  0.5, 1.0, 0.0, 0.0,
         0.5, -0.5, -0.5, 1.0, 0.0, 0.0,
        -0.5,  0.5,  0.5, 1.0, 0.0, 0.0,
        -0.5,  0.5, -0.5, 1.0, 0.0, 0.0,
        -0.5, -0.5,  0.5, 1.0, 0.0, 0.0,
        -0.5, -0.5, -0.5, 1.0, 0.0, 0.0
    ];

    let indices = vec![
        0, 1, 2, 2, 1, 3,
        4, 5, 6, 6, 5, 7,
        0, 2, 4, 4, 2, 6,
        1, 5, 3, 3, 5, 7,
        2, 3, 6, 6, 3, 7,
        0, 4, 1, 1, 4, 5
    ];

    Mesh::new(vertices, indices)
}
