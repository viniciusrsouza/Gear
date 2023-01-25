use std::collections::HashMap;

use log::warn;

use super::{
    entity::Mesh,
    renderer::shader::{Shader, ShaderError},
};

pub struct AssetsManager {
    assets_root: &'static str,
    shaders: HashMap<String, Shader>,
    objects: HashMap<String, Mesh>,
}

impl AssetsManager {
    pub fn new(assets_root: &'static str) -> Self {
        Self {
            assets_root,
            shaders: HashMap::new(),
            objects: HashMap::new(),
        }
    }

    pub fn load_shader(
        &mut self,
        name: &str,
        vertex_path: &str,
        fragment_path: &str,
    ) -> Result<bool, ShaderError> {
        let vertex_path = format!("{}/{}", self.assets_root, vertex_path);
        let fragment_path = format!("{}/{}", self.assets_root, fragment_path);

        let shader = Shader::new(vertex_path.as_str(), fragment_path.as_str())?;

        if self.shaders.contains_key(name) {
            warn!(target: "GEAR", "Shader with name '{}' already exists", name);
            return Ok(false);
        }

        self.shaders.insert(name.to_string(), shader);
        Ok(true)
    }

    pub fn put_shader(&mut self, name: &str, shader: Shader) {
        self.shaders.insert(name.to_string(), shader);
    }

    pub fn get_shader(&self, name: &str) -> Option<&Shader> {
        self.shaders.get(name)
    }

    pub fn put_object(&mut self, name: &str, object: Mesh) {
        self.objects.insert(name.to_string(), object);
    }

    pub fn get_object(&self, name: &str) -> Option<&Mesh> {
        self.objects.get(name)
    }

    pub fn get_mut_object(&mut self, name: &str) -> Option<&mut Mesh> {
        self.objects.get_mut(name)
    }
}
