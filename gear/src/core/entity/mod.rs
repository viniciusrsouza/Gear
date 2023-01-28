mod buffer;
mod light;
mod mesh;
mod player;
mod renderable;
mod transform;

use nalgebra::{Vector3, Vector4};

pub use buffer::EntityBuffer;
pub use light::{Light, LightBuilder};
pub use mesh::{cube, square, Mesh};
pub use renderable::Renderable;
pub use transform::Transform;

use super::assets::AssetsManager;

#[derive(Debug)]
pub struct Material {
    pub color: Vector4<f32>,
}

impl Material {
    pub fn new(color: [f32; 4]) -> Self {
        Self {
            color: Vector4::from(color),
        }
    }
}

#[derive(Debug)]
pub struct Entity {
    pub id: u32,
    pub renderable: Renderable,
}

impl Entity {
    pub fn init(&mut self, assets: &mut AssetsManager) {
        self.renderable.init(assets);
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_mesh_id(&self) -> u32 {
        self.renderable.mesh_id
    }

    pub fn get_mesh_indices(&self) -> u32 {
        self.renderable.mesh_indices
    }
}

pub struct EntityBuilder {
    // pub renderable: Option<Renderable>,
    pub mesh: Option<String>,
    pub shader: Option<String>,
    pub material: Material,
    pub position: Vector3<f32>,
    pub rotation: Vector3<f32>,
    pub scale: Vector3<f32>,
}

impl EntityBuilder {
    pub fn new() -> Self {
        Self {
            // renderable: None,
            mesh: None,
            shader: None,
            material: Material {
                color: Vector4::from([1.0, 1.0, 1.0, 1.0]),
            },
            position: Vector3::from([0.0, 0.0, 0.0]),
            rotation: Vector3::from([0.0, 0.0, 0.0]),
            scale: Vector3::from([1.0, 1.0, 1.0]),
        }
    }

    pub fn with_mesh(mut self, mesh: &str) -> Self {
        self.mesh = Some(mesh.to_owned());
        self
    }

    pub fn with_material(mut self, material: Material) -> Self {
        self.material = material;
        self
    }

    pub fn with_shader(mut self, shader: &str) -> Self {
        self.shader = Some(shader.to_owned());
        self
    }

    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.position = transform.position;
        self.rotation = transform.rotation;
        self.scale = transform.scale;
        self
    }

    pub fn with_position(mut self, position: Vector3<f32>) -> Self {
        self.position = position;
        self
    }

    pub fn with_rotation(mut self, rotation: Vector3<f32>) -> Self {
        self.rotation = rotation;
        self
    }

    pub fn with_scale(mut self, scale: Vector3<f32>) -> Self {
        self.scale = scale;
        self
    }

    pub fn build(
        self,
        assets: &AssetsManager,
        buffer: &mut EntityBuffer,
    ) -> Result<Entity, EntityError> {
        let mesh_id;
        let mesh_indices;
        let mesh = self.mesh.expect("Mesh not found");
        if let Some(mesh_obj) = assets.get_object(mesh.as_str()) {
            mesh_id = mesh_obj.get_id();
            mesh_indices = mesh_obj.indices.len() as u32;
        } else {
            return Err(EntityError::MeshNotFound);
        }

        let transform = Transform {
            position: self.position,
            rotation: self.rotation,
            scale: self.scale,
        };

        let renderable = Renderable {
            mesh: mesh,
            material: self.material,
            transform,
            shader: self.shader.expect("A shader is required"),
            mesh_id,
            mesh_indices,
        };

        let entity = Entity {
            id: buffer.get_id(),
            renderable,
        };

        Ok(entity)
    }
}

#[derive(Debug)]
pub enum EntityError {
    MeshNotFound,
    ShaderNotFound,
    EntityNotFound,
}
