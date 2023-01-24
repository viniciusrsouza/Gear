use gear::core::{
    event::{self, EventListener},
    layer::Layer,
};
use log::info;

pub struct RootLayer {}

impl RootLayer {
    pub fn new() -> RootLayer {
        RootLayer {}
    }
}

impl Layer for RootLayer {
    fn get_dbg_name(&self) -> &'static str {
        "RootLayer"
    }

    fn on_attach(&mut self) {
        info!(target: self.get_dbg_name(), "RootLayer attached.");
    }

    fn on_detach(&mut self) {
        info!(target: self.get_dbg_name(), "RootLayer detached.");
    }
}

impl EventListener for RootLayer {
    fn on_key_press(&mut self, _key: event::Key, _mods: event::Modifier) -> bool {
        info!(target: self.get_dbg_name(), "Key pressed: {:?}", _key);
        false
    }
}
