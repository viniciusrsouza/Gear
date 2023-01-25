use crate::core::assets::AssetsManager;

use super::Material;

#[derive(Debug)]
pub struct Renderable {
    pub mesh: String,
    pub material: Material,
    mesh_id: u32,
}

impl Renderable {
    pub fn new(mesh: String, material: Material) -> Self {
        Self {
            mesh,
            material,
            mesh_id: 0,
        }
    }

    pub fn init(&mut self, assets: &mut AssetsManager) {
        let mesh = assets.get_mut_object(self.mesh.as_str()).unwrap();
        mesh.init();
    }

    pub fn get_mesh_id(&self) -> u32 {
        self.mesh_id
    }

    pub fn set_mesh_id(&mut self, id: u32) {
        self.mesh_id = id;
    }
}
