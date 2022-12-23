use log::{debug, info};

use crate::{
    core::{
        event::{EventDispatcher, EventDispatcherImpl},
        logger::init,
        window::Window,
    },
    platform::WinType,
};

use super::{
    event::{propagate_event, Event, EventListener, GenericEventListener},
    layer::{imgui::ImGuiLayer, LayerStack, LayerStackImpl},
};

pub trait Application: EventListener {
    fn init() -> Self;
}

pub struct Gear<T: Application> {
    window: Option<Window<WinType<'static>>>,
    layers: LayerStackImpl,
    app: T,
}

impl<T: Application> Gear<T> {
    pub fn new() -> Self {
        init();

        Gear {
            window: None,
            layers: LayerStackImpl::new(),
            app: T::init(),
        }
    }

    fn load_default_layers(&mut self) {
        let imgui_layer = ImGuiLayer::new();
        self.push_layer(Box::new(imgui_layer));
    }

    pub fn run(&mut self) {
        debug!(target: "GEAR", "Application started.");

        self.load_default_layers();

        let mut window = Window::<WinType>::new("Gear", 800, 600);
        let mut dispatcher = EventDispatcherImpl::new();
        window.open();

        self.window = Some(window);

        while !self.window().should_close() {
            let window = self.window();
            window.update();
            window.dispatch_events(&mut dispatcher);

            dispatcher.consume(self);
        }

        self.on_close();
    }

    pub fn window(&mut self) -> &mut Window<WinType<'static>> {
        self.window.as_mut().unwrap()
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
        self.window().close();
        true
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
