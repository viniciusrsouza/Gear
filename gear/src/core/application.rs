use log::{debug, info};

use crate::{
    core::{
        event::{EventDispatcher, EventDispatcherImpl},
        logger::init,
        window::Window,
    },
    platform::WinType,
};

use super::event::EventListener;

pub trait Application: EventListener {
    fn init() -> Self;
}

pub struct Gear {
    window: Option<Window<WinType<'static>>>,
}

impl Gear {
    pub fn new() -> Self {
        Gear { window: None }
    }

    pub fn run<T: Application>(&mut self) {
        init();

        let mut window = Window::<WinType>::new("Gear", 800, 600);

        let mut app = T::init();
        let mut dispatcher = EventDispatcherImpl::new();

        debug!(target: "GEAR", "Application started.");

        window.open();

        self.window = Some(window);

        while !self.window().should_close() {
            let window = self.window();
            window.update();
            window.dispatch_events(&mut dispatcher);

            dispatcher.collect(vec![Box::new(self), Box::new(&mut app)]);
        }
    }

    pub fn window(&mut self) -> &mut Window<WinType<'static>> {
        self.window.as_mut().unwrap()
    }
}

impl EventListener for Gear {
    fn on_app_tick(&mut self) -> bool {
        false
    }
    fn on_app_update(&mut self) -> bool {
        false
    }
    fn on_app_render(&mut self) -> bool {
        false
    }
    fn on_window_close(&mut self) -> bool {
        info!(target: "GEAR", "Window closed.");
        self.window().close();
        true
    }
    fn on_window_resize(&mut self, _width: u32, _height: u32) -> bool {
        false
    }
    fn on_key_press(&mut self, _key: super::event::Key, _mods: super::event::Modifier) -> bool {
        false
    }
    fn on_key_release(&mut self, _key: super::event::Key, _mods: super::event::Modifier) -> bool {
        false
    }
    fn on_mouse_press(
        &mut self,
        _button: super::event::MouseButton,
        _mods: super::event::Modifier,
    ) -> bool {
        false
    }
    fn on_mouse_release(
        &mut self,
        _button: super::event::MouseButton,
        _mods: super::event::Modifier,
    ) -> bool {
        false
    }
    fn on_mouse_move(&mut self, _x: f64, _y: f64) -> bool {
        false
    }
    fn on_mouse_scroll(&mut self, _xoffset: f64, _yoffset: f64) -> bool {
        false
    }
}
