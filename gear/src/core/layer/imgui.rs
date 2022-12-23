use log::info;

use crate::core::event::EventListener;

use super::Layer;

pub struct ImGuiLayer {}

impl ImGuiLayer {
    pub fn new() -> ImGuiLayer {
        ImGuiLayer {}
    }
}

impl Layer for ImGuiLayer {
    fn get_dbg_name(&self) -> &'static str {
        "ImGuiLayer"
    }

    fn on_attach(&mut self) {
        info!(target: "GEAR", "ImGuiLayer attached.");
    }

    fn on_detach(&mut self) {
        info!(target: "GEAR", "ImGuiLayer detached.");
    }

    fn on_update(&mut self) {}
}

impl EventListener for ImGuiLayer {}
