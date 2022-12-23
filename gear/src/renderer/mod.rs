use crate::platform::WinType;

mod opengl;

pub trait Renderer {
    fn init(&mut self, window: &mut WinType);
    fn render(&mut self);
}

enum RendererType {
    OpenGL,
    None,
}

pub struct RendererImpl {
    gl: Option<opengl::OpenGlRenderer>,
    current: RendererType,
}

impl RendererImpl {
    pub fn api(&mut self) -> &mut dyn Renderer {
        match self.current {
            RendererType::OpenGL => self.gl.as_mut().unwrap(),
            RendererType::None => panic!("No renderer selected!"),
        }
    }

    pub fn new() -> Self {
        RendererImpl {
            gl: None,
            current: RendererType::None,
        }
    }

    pub fn use_opengl(&mut self, window: &mut WinType) {
        self.current = RendererType::OpenGL;
        let mut gl = opengl::OpenGlRenderer::new();
        gl.init(window);
        self.gl = Some(gl);
    }
}
