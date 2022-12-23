use crate::{
    platform::{WinType, WindowApi},
    renderer::RendererImpl,
};

use super::event::{AppEvent, Event, EventDispatcher};

pub struct Window<T: WindowApi> {
    vsync: bool,
    resizable: bool,
    should_close: bool,
    api: T,
    renderer: RendererImpl,
}

impl<'a> Window<WinType<'a>> {
    pub fn new(title: &str, width: u32, height: u32) -> Window<WinType> {
        Window {
            vsync: true,
            resizable: false,
            should_close: false,
            api: WinType::new(title, width, height),
            renderer: RendererImpl::new(),
        }
    }

    pub fn set_vsync(&mut self, vsync: bool) {
        self.vsync = vsync;
    }

    pub fn set_resizable(&mut self, resizable: bool) {
        self.resizable = resizable;
    }

    pub fn open(&mut self) {
        self.api.create_window();
        self.renderer.use_opengl(&mut self.api);
    }

    pub fn dispatch_events(&mut self, dispatcher: &mut impl EventDispatcher) {
        dispatcher.dispatch(Event::App(AppEvent::Tick));
        self.api.dispatch(dispatcher);
    }

    pub fn update(&mut self) {
        self.renderer.api().render();
        self.api.update();
    }

    pub fn should_close(&self) -> bool {
        self.should_close
    }

    pub fn close(&mut self) {
        self.api.close();
        self.should_close = true;
    }

    pub fn get_proc_address(&mut self, name: &str) -> *const std::ffi::c_void {
        self.api.get_proc_address(name)
    }
}
