use log::debug;

use crate::core::logger::init;

pub struct Application {}

impl Application {
    pub fn new() -> Application {
        init();
        Application {}
    }

    pub fn run(&mut self) {
        debug!(target: "GEAR", "Application started.");
        loop {}
    }
}
