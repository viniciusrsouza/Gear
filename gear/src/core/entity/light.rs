use nalgebra::Vector3;

use crate::core::assets::AssetsManager;

use super::{Entity, EntityBuffer, EntityBuilder, EntityError, Material, Transform};

pub struct Light {
    pub entity: Entity,
    pub ambient: f32,
    pub diffuse: f32,
    pub specular: f32,
    pub color: Vector3<f32>,
}

pub struct LightBuilder {
    entity: EntityBuilder,
    ambient: f32,
    diffuse: f32,
    specular: f32,
    color: Vector3<f32>,
}

impl LightBuilder {
    pub fn new() -> Self {
        Self {
            entity: EntityBuilder::new(),
            ambient: 0.1,
            diffuse: 0.8,
            specular: 1.0,
            color: Vector3::from([1.0, 1.0, 1.0]),
        }
    }

    pub fn with_ambient(mut self, ambient: f32) -> Self {
        self.ambient = ambient;
        self
    }

    pub fn with_diffuse(mut self, diffuse: f32) -> Self {
        self.diffuse = diffuse;
        self
    }

    pub fn with_specular(mut self, specular: f32) -> Self {
        self.specular = specular;
        self
    }

    pub fn with_color(mut self, color: Vector3<f32>) -> Self {
        self.color = color;
        self
    }

    pub fn with_mesh(mut self, mesh: &str) -> Self {
        self.entity = self.entity.with_mesh(mesh);
        self
    }

    pub fn with_material(mut self, material: Material) -> Self {
        self.entity = self.entity.with_material(material);
        self
    }

    pub fn with_shader(mut self, shader: &str) -> Self {
        self.entity = self.entity.with_shader(shader);
        self
    }

    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.entity = self.entity.with_transform(transform);
        self
    }

    pub fn with_position(mut self, position: Vector3<f32>) -> Self {
        self.entity = self.entity.with_position(position);
        self
    }

    pub fn with_rotation(mut self, rotation: Vector3<f32>) -> Self {
        self.entity = self.entity.with_rotation(rotation);
        self
    }

    pub fn with_scale(mut self, scale: Vector3<f32>) -> Self {
        self.entity = self.entity.with_scale(scale);
        self
    }

    pub fn build(
        self,
        assets: &AssetsManager,
        entities: &mut EntityBuffer,
    ) -> Result<Light, EntityError> {
        let light = Light {
            entity: self.entity.build(assets, entities)?,
            ambient: self.ambient,
            diffuse: self.diffuse,
            specular: self.specular,
            color: self.color,
        };
        Ok(light)
    }
}
