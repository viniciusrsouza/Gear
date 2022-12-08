use super::event::{propagate_event, Event, EventListener, GenericEventListener};

pub trait Layer: EventListener {
    fn on_attach(&mut self) {}
    fn on_detach(&mut self) {}
    fn on_update(&mut self) {}
    fn get_dbg_name(&self) -> &'static str;
}

pub trait LayerStack {
    fn push_layer(&mut self, layer: Box<dyn Layer>);
    fn push_overlay(&mut self, overlay: Box<dyn Layer>);
    fn pop_layer(&mut self) -> Option<Box<dyn Layer>>;
    fn pop_overlay(&mut self) -> Option<Box<dyn Layer>>;
}

pub struct LayerStackImpl {
    layers: Vec<Box<dyn Layer>>,
    overlay_offset: usize,
}

impl LayerStackImpl {
    pub fn new() -> LayerStackImpl {
        LayerStackImpl {
            layers: Vec::new(),
            overlay_offset: 0,
        }
    }

    pub fn iter(&self) -> std::slice::Iter<Box<dyn Layer>> {
        self.layers.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<Box<dyn Layer>> {
        self.layers.iter_mut()
    }
}

impl LayerStack for LayerStackImpl {
    fn push_layer(&mut self, layer: Box<dyn Layer>) {
        self.layers.insert(self.overlay_offset, layer);
        self.overlay_offset += 1;
    }

    fn push_overlay(&mut self, overlay: Box<dyn Layer>) {
        self.layers.push(overlay);
    }

    fn pop_layer(&mut self) -> Option<Box<dyn Layer>> {
        if self.overlay_offset > 0 {
            self.overlay_offset -= 1;
            Some(self.layers.remove(self.overlay_offset))
        } else {
            None
        }
    }

    fn pop_overlay(&mut self) -> Option<Box<dyn Layer>> {
        self.layers.pop()
    }
}

impl GenericEventListener for LayerStackImpl {
    fn on_event(&mut self, event: Event) -> bool {
        for layer in self.layers.iter_mut().rev() {
            if propagate_event(event, layer.as_event_listener()) {
                return true;
            }
        }
        false
    }
}
