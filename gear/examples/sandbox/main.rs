pub mod layers;

use gear::prelude::*;
use log::error;
use nalgebra::Vector3;

extern crate nalgebra as na;

struct Sandbox {}

fn main() {
    let mut gear = Gear::<Sandbox>::new();

    let root_layer = layers::RootLayer::new();
    gear.push_layer(Box::new(root_layer));

    gear.run();
}

impl Application for Sandbox {
    fn init() -> Self {
        Sandbox {}
    }

    fn post_init(&mut self, ctx: &mut Context) {
        let Context {
            ref mut assets,
            ref mut entity_buffer,
            ..
        } = ctx;
        self.load_objects(assets);

        if let Err(err) = self.load_shaders(assets) {
            error!("Error: {:?}", err);
        }

        if let Err(err) = self.load_entities(assets, entity_buffer) {
            error!("Error: {:?}", err);
        }
    }

    fn get_assets_path() -> &'static str {
        "gear/examples/sandbox/assets/"
    }
}

impl Sandbox {
    fn load_shaders(&mut self, assets: &mut AssetsManager) -> Result<(), ShaderError> {
        assets.load_shader("default", "vert.glsl", "frag.glsl")?;
        assets.load_shader("light", "vert.glsl", "light.glsl")?;
        Ok(())
    }

    fn load_objects(&mut self, assets: &mut AssetsManager) {
        let mut mesh = cube();
        mesh.init();
        assets.put_object("cube", mesh);
    }

    fn load_entities(
        &mut self,
        assets: &mut AssetsManager,
        entities: &mut EntityBuffer,
    ) -> Result<(), EntityError> {
        let entity = EntityBuilder::new()
            .with_shader("default")
            .with_mesh("cube")
            .with_material(Material::new([1.0, 0.0, 0.0, 1.0]))
            .with_position(Vector3::from([0.5, -0.5, 0.0]))
            .with_scale(Vector3::from([0.5, 0.5, 0.5]))
            .with_rotation(Vector3::from([0.0, 0.0, 1.0]) * 15.0f32.to_radians())
            .build(assets, entities)?;

        entities.add_entity(entity);

        let entity = EntityBuilder::new()
            .with_shader("default")
            .with_mesh("cube")
            .with_material(Material::new([0.0, 0.0, 1.0, 1.0]))
            .with_position(Vector3::from([-0.2, 0.2, 0.0]))
            .with_scale(Vector3::from([0.2, 0.2, 0.2]))
            .with_rotation(Vector3::from([0.0, 0.0, 1.0]) * 60.0f32.to_radians())
            .build(assets, entities)?;

        entities.add_entity(entity);

        let light = LightBuilder::new()
            .with_shader("light")
            .with_mesh("cube")
            .with_material(Material::new([1.0, 1.0, 1.0, 1.0]))
            .with_position(Vector3::from([1.2, 1.0, 2.0]))
            .with_scale(Vector3::from([0.1, 0.1, 0.1]))
            .with_rotation(Vector3::from([0.0, 0.0, 1.0]) * 60.0f32.to_radians())
            .build(assets, entities)?;

        entities.add_light(light);
        Ok(())
    }
}

impl EventListener for Sandbox {}
