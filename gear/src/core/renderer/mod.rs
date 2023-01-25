pub mod shader;

use crate::core::window::Window;

use super::{assets::AssetsManager, entity::EntityBuffer};

pub mod gl {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub struct Renderer {}

impl Renderer {
    pub fn new() -> Self {
        Renderer {}
    }

    pub fn init(
        &mut self,
        window: &mut Window,
        assets: &mut AssetsManager,
        entity_buffer: &mut EntityBuffer,
    ) {
        gl::load_with(|name| window.get_proc_address(name));

        for entity in entity_buffer.entities.iter_mut() {
            entity.init(assets);
        }
    }

    pub fn render(&mut self, assets: &mut AssetsManager, entity_buffer: &mut EntityBuffer) {
        unsafe {
            gl::ClearColor(0.3, 0.3, 0.5, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            for entity in entity_buffer.entities.iter_mut() {
                let shader = entity.shader.as_str();
                let shader = assets.get_shader(shader).unwrap();
                gl::UseProgram(shader.get_id());

                shader.with_transform(&entity.transform);
                shader.with_material(&entity.renderable.material);

                gl::BindVertexArray(entity.get_mesh_id());
                gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
            }
        }
    }
}
