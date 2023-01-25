use glfw::Context;
use log::{debug, info};

extern crate glfw;

use super::event::{
    modifiers as app_modifiers, AppEvent, Event, EventDispatcher, Key, KeyboardEvent, MouseButton,
    MouseEvent, WindowEvent,
};

pub struct Window {
    api: glfw::Window,
    glfw: glfw::Glfw,
    events: std::sync::mpsc::Receiver<(f64, glfw::WindowEvent)>,
    vsync: bool,
    resizable: bool,
    pub should_close: bool,
}

impl Window {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));

        #[cfg(target_os = "macos")]
        {
            glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
            debug!(target: "GEAR", "MacOS detected. Setting forward compat.")
        }

        let (window, events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");
        Self {
            api: window,
            glfw: glfw,
            events: events,
            vsync: false,
            resizable: true,
            should_close: false,
        }
    }

    fn create_window(&mut self) {
        info!(target: "GEAR", "creating window");

        self.api.set_all_polling(true);
        self.api.make_current();
    }

    pub fn update(&mut self) {
        self.api.swap_buffers();
        self.glfw.poll_events();
    }

    fn dispatch(&mut self, dispatcher: &mut dyn EventDispatcher) {
        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                // window
                glfw::WindowEvent::Close => dispatcher.dispatch(Event::Window(WindowEvent::Close)),
                glfw::WindowEvent::Size(width, height) => dispatcher.dispatch(Event::Window(
                    WindowEvent::Resize(width as u32, height as u32),
                )),

                // keyboard
                glfw::WindowEvent::Key(key, _, action, modifiers) => match action {
                    glfw::Action::Press => dispatcher.dispatch(Event::Keyboard(
                        KeyboardEvent::Press(glfw_to_key(key), glfw_to_modifier(modifiers)),
                    )),

                    glfw::Action::Release => dispatcher.dispatch(Event::Keyboard(
                        KeyboardEvent::Release(glfw_to_key(key), glfw_to_modifier(modifiers)),
                    )),
                    _ => {}
                },

                // mouse
                glfw::WindowEvent::MouseButton(button, action, modifiers) => match action {
                    glfw::Action::Press => dispatcher.dispatch(Event::Mouse(MouseEvent::Press(
                        glfw_to_mouse_button(button),
                        glfw_to_modifier(modifiers),
                    ))),
                    glfw::Action::Release => {
                        dispatcher.dispatch(Event::Mouse(MouseEvent::Release(
                            glfw_to_mouse_button(button),
                            glfw_to_modifier(modifiers),
                        )))
                    }
                    _ => {}
                },
                glfw::WindowEvent::CursorPos(x, y) => {
                    dispatcher.dispatch(Event::Mouse(MouseEvent::Move(x, y)))
                }
                glfw::WindowEvent::Scroll(x, y) => {
                    dispatcher.dispatch(Event::Mouse(MouseEvent::Scroll(x, y)))
                }
                _ => {}
            };
        }
    }

    pub fn close(&mut self) {
        info!(target: "GEAR", "closing window");

        self.api.set_should_close(true);
    }

    pub fn get_proc_address(&mut self, name: &str) -> *const std::ffi::c_void {
        self.api.get_proc_address(name)
    }

    pub fn set_vsync(&mut self, vsync: bool) {
        self.vsync = vsync;
    }

    pub fn set_resizable(&mut self, resizable: bool) {
        self.resizable = resizable;
    }

    pub fn open(&mut self) {
        self.create_window();
    }

    pub fn dispatch_events(&mut self, dispatcher: &mut impl EventDispatcher) {
        dispatcher.dispatch(Event::App(AppEvent::Tick));
        self.dispatch(dispatcher);
    }

    pub fn should_close(&self) -> bool {
        self.api.should_close()
    }
}

fn glfw_to_modifier(modifier: glfw::Modifiers) -> u8 {
    let mut result = 0;
    if modifier.contains(glfw::Modifiers::Shift) {
        result |= app_modifiers::SHIFT;
    }
    if modifier.contains(glfw::Modifiers::Control) {
        result |= app_modifiers::CTRL;
    }
    if modifier.contains(glfw::Modifiers::Alt) {
        result |= app_modifiers::ALT;
    }
    if modifier.contains(glfw::Modifiers::Super) {
        result |= app_modifiers::SUPER;
    }
    result
}

fn glfw_to_key(key: glfw::Key) -> Key {
    match key {
        glfw::Key::A => Key::A,
        glfw::Key::B => Key::B,
        glfw::Key::C => Key::C,
        glfw::Key::D => Key::D,
        glfw::Key::E => Key::E,
        glfw::Key::F => Key::F,
        glfw::Key::G => Key::G,
        glfw::Key::H => Key::H,
        glfw::Key::I => Key::I,
        glfw::Key::J => Key::J,
        glfw::Key::K => Key::K,
        glfw::Key::L => Key::L,
        glfw::Key::M => Key::M,
        glfw::Key::N => Key::N,
        glfw::Key::O => Key::O,
        glfw::Key::P => Key::P,
        glfw::Key::Q => Key::Q,
        glfw::Key::R => Key::R,
        glfw::Key::S => Key::S,
        glfw::Key::T => Key::T,
        glfw::Key::U => Key::U,
        glfw::Key::V => Key::V,
        glfw::Key::W => Key::W,
        glfw::Key::X => Key::X,
        glfw::Key::Y => Key::Y,
        glfw::Key::Z => Key::Z,
        glfw::Key::Num0 => Key::Num0,
        glfw::Key::Num1 => Key::Num1,
        glfw::Key::Num2 => Key::Num2,
        glfw::Key::Num3 => Key::Num3,
        glfw::Key::Num4 => Key::Num4,
        glfw::Key::Num5 => Key::Num5,
        glfw::Key::Num6 => Key::Num6,
        glfw::Key::Num7 => Key::Num7,
        glfw::Key::Num8 => Key::Num8,
        glfw::Key::Num9 => Key::Num9,
        glfw::Key::Escape => Key::Escape,
        glfw::Key::Enter => Key::Enter,
        glfw::Key::Tab => Key::Tab,
        glfw::Key::Space => Key::Space,
        glfw::Key::Minus => Key::Minus,
        glfw::Key::Equal => Key::Equal,
        glfw::Key::LeftBracket => Key::LBracket,
        glfw::Key::RightBracket => Key::RBracket,
        glfw::Key::Backslash => Key::Backslash,
        glfw::Key::Semicolon => Key::Semicolon,
        glfw::Key::Apostrophe => Key::Apostrophe,
        glfw::Key::GraveAccent => Key::GraveAccent,
        glfw::Key::Comma => Key::Comma,
        glfw::Key::Period => Key::Period,
        glfw::Key::Slash => Key::Slash,
        glfw::Key::CapsLock => Key::CapsLock,
        glfw::Key::F1 => Key::F1,
        glfw::Key::F2 => Key::F2,
        glfw::Key::F3 => Key::F3,
        glfw::Key::F4 => Key::F4,
        glfw::Key::F5 => Key::F5,
        glfw::Key::F6 => Key::F6,
        glfw::Key::F7 => Key::F7,
        glfw::Key::F8 => Key::F8,
        glfw::Key::F9 => Key::F9,
        glfw::Key::F10 => Key::F10,
        glfw::Key::F11 => Key::F11,
        glfw::Key::F12 => Key::F12,
        glfw::Key::PrintScreen => Key::PrintScreen,
        glfw::Key::ScrollLock => Key::ScrollLock,
        glfw::Key::Pause => Key::Pause,
        glfw::Key::Insert => Key::Insert,
        glfw::Key::Home => Key::Home,
        glfw::Key::PageUp => Key::PageUp,
        glfw::Key::Delete => Key::Delete,
        glfw::Key::End => Key::End,
        glfw::Key::PageDown => Key::PageDown,
        glfw::Key::Right => Key::Right,
        glfw::Key::Left => Key::Left,
        glfw::Key::Down => Key::Down,
        glfw::Key::Up => Key::Up,
        glfw::Key::NumLock => Key::NumLock,
        glfw::Key::KpDivide => Key::NumPadDivide,
        glfw::Key::KpMultiply => Key::NumPadMultiply,
        glfw::Key::KpSubtract => Key::NumPadSubtract,
        glfw::Key::KpAdd => Key::NumPadAdd,
        glfw::Key::KpEnter => Key::NumPadEnter,
        glfw::Key::KpDecimal => Key::NumPadDecimal,
        glfw::Key::Kp0 => Key::NumPad0,
        glfw::Key::Kp1 => Key::NumPad1,
        glfw::Key::Kp2 => Key::NumPad2,
        glfw::Key::Kp3 => Key::NumPad3,
        glfw::Key::Kp4 => Key::NumPad4,
        glfw::Key::Kp5 => Key::NumPad5,
        glfw::Key::Kp6 => Key::NumPad6,
        glfw::Key::Kp7 => Key::NumPad7,
        glfw::Key::Kp8 => Key::NumPad8,
        glfw::Key::Kp9 => Key::NumPad9,
        glfw::Key::KpEqual => Key::NumPadEqual,
        glfw::Key::LeftShift => Key::LShift,
        glfw::Key::LeftControl => Key::LControl,
        glfw::Key::LeftAlt => Key::LAlt,
        glfw::Key::LeftSuper => Key::LSuper,
        glfw::Key::RightShift => Key::RShift,
        glfw::Key::RightControl => Key::RControl,
        glfw::Key::RightAlt => Key::RAlt,
        glfw::Key::RightSuper => Key::RSuper,
        glfw::Key::Menu => Key::Menu,
        _ => Key::Unknown,
    }
}

fn glfw_to_mouse_button(button: glfw::MouseButton) -> MouseButton {
    match button {
        glfw::MouseButton::Button1 => MouseButton::Left,
        glfw::MouseButton::Button2 => MouseButton::Right,
        glfw::MouseButton::Button3 => MouseButton::Middle,
        glfw::MouseButton::Button4 => MouseButton::Button4,
        glfw::MouseButton::Button5 => MouseButton::Button5,
        glfw::MouseButton::Button6 => MouseButton::Button6,
        glfw::MouseButton::Button7 => MouseButton::Button7,
        glfw::MouseButton::Button8 => MouseButton::Button8,
    }
}
