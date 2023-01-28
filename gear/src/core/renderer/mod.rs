pub mod camera;
pub mod shader;

use super::{
    application::context::Context,
    entity::{EntityBuffer, Renderable},
};

pub mod gl {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub struct Renderer {}

impl Renderer {
    pub fn new() -> Self {
        Renderer {}
    }

    pub fn init(&mut self, ctx: &mut Context) {
        let Context {
            assets,
            window,
            entity_buffer,
            ..
        } = ctx;
        gl::load_with(|name| window.get_proc_address(name));
        unsafe { gl::Enable(gl::DEPTH_TEST) }

        for entity in entity_buffer.entities.iter_mut() {
            entity.init(assets);
        }
    }

    pub fn render(&mut self, ctx: &mut Context) {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            for entity in ctx.entity_buffer.entities.iter() {
                self.render_renderable(ctx, &entity.renderable);
            }
            self.render_light(ctx);
        }
    }

    fn render_light(&self, ctx: &Context) {
        let Context {
            assets,
            entity_buffer,
            camera,
            window,
            ..
        } = ctx;
        let EntityBuffer { light, .. } = entity_buffer;
        let light = light.as_ref().unwrap();

        let shader = assets.get_shader("light").unwrap();
        unsafe { gl::UseProgram(shader.get_id()) };

        shader.with_transform(&light.entity.renderable.transform);
        shader.with_camera(&camera, window.get_aspect_ratio());
        shader.set_vec3("lightColor", &light.color);

        unsafe {
            gl::BindVertexArray(light.entity.renderable.mesh_id);
            gl::DrawElements(
                gl::TRIANGLES,
                light.entity.renderable.mesh_indices as i32,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
        }
    }

    fn render_renderable(&self, ctx: &Context, renderable: &Renderable) {
        let Context {
            assets,
            camera,
            window,
            entity_buffer,
            ..
        } = ctx;
        let EntityBuffer { light, .. } = entity_buffer;

        let shader = renderable.shader.as_str();
        let shader = assets.get_shader(shader).unwrap();
        unsafe { gl::UseProgram(shader.get_id()) };

        shader.with_transform(&renderable.transform);
        shader.with_material(&renderable.material);
        shader.with_camera(&camera, window.get_aspect_ratio());
        shader.with_light(&light.as_ref().unwrap());

        unsafe {
            gl::BindVertexArray(renderable.mesh_id);
            gl::DrawElements(
                gl::TRIANGLES,
                renderable.mesh_indices as i32,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
        }
    }
}
