pub mod layers;

use gear::core::{
    application::{Application, Gear},
    event::EventListener,
    layer::LayerStack,
};

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
}

impl EventListener for Sandbox {}
