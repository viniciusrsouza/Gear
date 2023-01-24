pub struct AppState {
    pub window: WindowState,
}

pub struct WindowState {
    should_close: bool,
}

impl WindowState {
    pub fn new() -> Self {
        Self {
            should_close: false,
        }
    }

    pub fn should_close(&self) -> bool {
        self.should_close
    }

    pub fn close(&mut self) {
        self.should_close = true;
    }
}

impl AppState {
    pub fn new() -> Self {
        Self {
            window: WindowState::new(),
        }
    }
}
