use crate::core::assets::AssetsManager;

use super::{Material, Transform};

#[derive(Debug)]
pub struct Renderable {
    pub mesh: String,
    pub material: Material,
    pub transform: Transform,
    pub shader: String,
    pub mesh_id: u32,
    pub mesh_indices: u32,
}

impl Renderable {
    pub fn init(&mut self, assets: &mut AssetsManager) {
        let mesh = assets.get_mut_object(self.mesh.as_str()).unwrap();
        mesh.init();
    }
}
