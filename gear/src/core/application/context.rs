use crate::core::{
    assets::AssetsManager, entity::EntityBuffer, renderer::camera::Camera, window::Window,
};

pub struct Context {
    pub assets: AssetsManager,
    pub entity_buffer: EntityBuffer,
    pub camera: Camera,
    pub window: Window,
}

impl Context {
    pub fn new(assets_root: &'static str, window: Window) -> Self {
        Context {
            assets: AssetsManager::new(assets_root),
            entity_buffer: EntityBuffer::new(),
            camera: Camera::new(),
            window,
        }
    }
}
