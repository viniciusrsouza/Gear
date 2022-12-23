use crate::core::event::EventDispatcher;

pub(crate) mod mac;

pub trait WindowApi {
    fn new(title: &str, width: u32, height: u32) -> Self;
    fn create_window(&mut self);
    fn update(&mut self);
    fn close(&mut self);
    fn dispatch(&mut self, dispatcher: &mut dyn EventDispatcher);
    fn get_proc_address(&mut self, name: &str) -> *const std::ffi::c_void;
}

macro_rules! WIN_TYPE {
    () => {
        #[cfg(target_os = "macos")]
        pub type WinType<'a> = crate::platform::mac::MacWindow;

        #[cfg(target_os = "windows")]
        pub type WinType = WinWindow;
    };
}

WIN_TYPE!();
