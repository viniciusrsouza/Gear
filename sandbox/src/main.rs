use gear::core::{
    application::{Application, Gear},
    event::EventListener,
};

struct Sandbox {}

fn main() {
    let mut gear = Gear::new();
    gear.run::<Sandbox>();
}

impl Application for Sandbox {
    fn init() -> Self {
        Sandbox {}
    }
}

impl EventListener for Sandbox {
    fn on_app_tick(&mut self) -> bool {
        true
    }

    fn on_app_update(&mut self) -> bool {
        true
    }

    fn on_app_render(&mut self) -> bool {
        true
    }

    fn on_window_close(&mut self) -> bool {
        true
    }

    fn on_window_resize(&mut self, _width: u32, _height: u32) -> bool {
        true
    }

    fn on_key_press(
        &mut self,
        _key: gear::core::event::Key,
        _mods: gear::core::event::Modifier,
    ) -> bool {
        true
    }

    fn on_key_release(
        &mut self,
        _key: gear::core::event::Key,
        _mods: gear::core::event::Modifier,
    ) -> bool {
        true
    }

    fn on_mouse_press(
        &mut self,
        _button: gear::core::event::MouseButton,
        _mods: gear::core::event::Modifier,
    ) -> bool {
        true
    }

    fn on_mouse_release(
        &mut self,
        _button: gear::core::event::MouseButton,
        _mods: gear::core::event::Modifier,
    ) -> bool {
        true
    }

    fn on_mouse_move(&mut self, _x: f64, _y: f64) -> bool {
        false
    }

    fn on_mouse_scroll(&mut self, _xoffset: f64, _yoffset: f64) -> bool {
        false
    }
}
