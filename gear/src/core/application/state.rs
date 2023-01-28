pub struct AppState {
    pub window: WindowState,
}

pub struct WindowState {
    should_close: bool,
    focus: bool,
}

impl WindowState {
    pub fn new() -> Self {
        Self {
            should_close: false,
            focus: false,
        }
    }

    pub fn should_close(&self) -> bool {
        self.should_close
    }

    pub fn close(&mut self) {
        self.should_close = true;
    }

    pub fn is_focused(&mut self) -> bool {
        self.focus
    }

    pub fn set_focus(&mut self, locked: bool) {
        self.focus = locked;
    }
}

impl AppState {
    pub fn new() -> Self {
        Self {
            window: WindowState::new(),
        }
    }
}
