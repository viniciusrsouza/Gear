pub mod context;
mod state;

use context::Context;
use log::{debug, info, trace};

use crate::core::{
    event::{EventDispatcher, EventDispatcherImpl},
    window::Window,
};

use self::state::AppState;

use super::{
    event::{propagate_event, Event, EventListener, GenericEventListener, Key},
    layer::{imgui::ImGuiLayer, LayerStack, LayerStackImpl},
    logger,
    renderer::Renderer,
};

pub trait Application: EventListener {
    fn init() -> Self;
    fn post_init(&mut self, context: &mut Context);
    fn get_assets_path() -> &'static str;
}

pub struct Gear<T: Application> {
    layers: LayerStackImpl,
    app: T,
    state: AppState,
    context: context::Context,
}

impl<T: Application> Gear<T> {
    pub fn new() -> Self {
        logger::init();

        let mut window = Window::new("Gear", 800, 600);
        window.open();
        window.set_mouse_lock(true);

        Gear {
            layers: LayerStackImpl::new(),
            app: T::init(),
            state: AppState::new(),
            context: Context::new(T::get_assets_path(), window),
        }
    }

    fn load_default_layers(&mut self) {
        let imgui_layer = ImGuiLayer::new();
        self.push_layer(Box::new(imgui_layer));
    }

    pub fn run(&mut self) {
        debug!(target: "GEAR", "Application started.");

        self.load_default_layers();

        let mut dispatcher = EventDispatcherImpl::new();

        let mut renderer = Renderer::new();
        renderer.init(&mut self.context);

        self.app.post_init(&mut self.context);

        while !self.state.window.should_close() {
            self.context.camera.update();
            renderer.render(&mut self.context);

            self.context.window.update();
            self.context.window.dispatch_events(&mut dispatcher);
            dispatcher.consume(self);
        }

        self.context.window.close();
        self.on_close();
    }

    fn on_close(&mut self) {
        debug!(target: "GEAR", "Application closed.");

        // detaching layers
        while let Some(layer) = self.pop_layer() {
            drop(layer);
        }
        while let Some(overlay) = self.pop_overlay() {
            drop(overlay);
        }
    }
}

impl<T: Application> GenericEventListener for Gear<T> {
    fn on_event(&mut self, event: Event) -> bool {
        if propagate_event(event, self) {
            return true;
        }

        if propagate_event(event, &mut self.app) {
            return true;
        }

        self.layers.on_event(event)
    }
}

impl<T: Application> EventListener for Gear<T> {
    fn on_window_close(&mut self) -> bool {
        info!(target: "GEAR", "Window closed.");
        self.state.window.close();
        true
    }

    fn on_key_press(&mut self, key: super::event::Key, _mods: super::event::Modifier) -> bool {
        match key {
            Key::Escape => {
                info!(target: "GEAR", "Escape pressed.");
                self.state.window.close();
                true
            }
            Key::W => {
                self.context.camera.move_forward(0.1);
                false
            }
            Key::S => {
                self.context.camera.move_backward(0.1);
                false
            }
            Key::A => {
                self.context.camera.move_left(0.1);
                false
            }
            Key::D => {
                self.context.camera.move_right(0.1);
                false
            }
            _ => false,
        }
    }

    fn on_key_repeat(&mut self, key: Key, _mods: super::event::Modifier) -> bool {
        match key {
            Key::W => {
                self.context.camera.move_forward(0.1);
                false
            }
            Key::S => {
                self.context.camera.move_backward(0.1);
                false
            }
            Key::A => {
                self.context.camera.move_left(0.1);
                false
            }
            Key::D => {
                self.context.camera.move_right(0.1);
                false
            }
            _ => false,
        }
    }

    fn on_mouse_move(&mut self, x: f64, y: f64) -> bool {
        if self.state.window.is_focused() {
            self.context.camera.on_mouse_move(x as f32, y as f32);
        }
        false
    }

    fn on_window_focus(&mut self, focused: bool) -> bool {
        trace!(target: "GEAR", "focus: {}", focused);
        self.state.window.set_focus(focused);
        false
    }
}

impl<T: Application> LayerStack for Gear<T> {
    fn push_layer(&mut self, mut layer: Box<dyn super::layer::Layer>) {
        layer.on_attach();
        self.layers.push_layer(layer)
    }

    fn push_overlay(&mut self, mut overlay: Box<dyn super::layer::Layer>) {
        overlay.on_attach();
        self.layers.push_overlay(overlay)
    }

    fn pop_layer(&mut self) -> Option<Box<dyn super::layer::Layer>> {
        let mut layer = self.layers.pop_layer();
        if let Some(layer) = layer.as_mut() {
            layer.on_detach();
        }
        layer
    }

    fn pop_overlay(&mut self) -> Option<Box<dyn super::layer::Layer>> {
        let mut overlay = self.layers.pop_overlay();
        if let Some(overlay) = overlay.as_mut() {
            overlay.on_detach();
        }
        overlay
    }
}
