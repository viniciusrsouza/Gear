mod state;

use log::{debug, info};

use crate::core::{
    entity::EntityBuffer,
    event::{EventDispatcher, EventDispatcherImpl},
    window::Window,
};

use self::state::AppState;

use super::{
    assets::AssetsManager,
    event::{propagate_event, Event, EventListener, GenericEventListener, Key},
    layer::{imgui::ImGuiLayer, LayerStack, LayerStackImpl},
    logger,
    renderer::Renderer,
};

pub trait Application: EventListener {
    fn init() -> Self;
    fn post_init(&mut self, assets: &mut AssetsManager, entities: &mut EntityBuffer);
    fn get_assets_path() -> &'static str;
}

pub struct Gear<T: Application> {
    layers: LayerStackImpl,
    app: T,
    state: AppState,
    assets: AssetsManager,
}

impl<T: Application> Gear<T> {
    pub fn new() -> Self {
        logger::init();

        Gear {
            layers: LayerStackImpl::new(),
            app: T::init(),
            state: AppState::new(),
            assets: AssetsManager::new(T::get_assets_path()),
        }
    }

    fn load_default_layers(&mut self) {
        let imgui_layer = ImGuiLayer::new();
        self.push_layer(Box::new(imgui_layer));
    }

    pub fn run(&mut self) {
        debug!(target: "GEAR", "Application started.");

        self.load_default_layers();

        let mut window = Window::new("Gear", 800, 600);
        let mut dispatcher = EventDispatcherImpl::new();
        window.open();

        let mut entity_buffer = EntityBuffer::new();

        let mut renderer = Renderer::new();
        renderer.init(&mut window, &mut self.assets, &mut entity_buffer);

        self.app.post_init(&mut self.assets, &mut entity_buffer);

        while !self.state.window.should_close() {
            renderer.render(&mut self.assets, &mut entity_buffer);

            window.update();
            window.dispatch_events(&mut dispatcher);
            dispatcher.consume(self);
        }

        window.close();
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
            _ => false,
        }
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
