use super::{AppEvent, Event, Key, KeyboardEvent, Modifier, MouseButton, MouseEvent, WindowEvent};

pub trait EventDispatcher {
    fn dispatch(&mut self, event: Event);
    fn consume(&mut self, listener: &mut dyn GenericEventListener);
}
pub struct EventDispatcherImpl {
    queue: EventQueue,
}

impl EventDispatcherImpl {
    pub fn new() -> EventDispatcherImpl {
        EventDispatcherImpl {
            queue: EventQueue::new(),
        }
    }
}

impl EventDispatcher for EventDispatcherImpl {
    fn dispatch(&mut self, event: Event) {
        self.queue.push(event);
    }

    fn consume(&mut self, listener: &mut dyn GenericEventListener) {
        for event in self.queue.pop() {
            listener.on_event(event);
        }
    }
}

pub struct EventQueue {
    events: Vec<Event>,
}

impl EventQueue {
    pub fn new() -> EventQueue {
        EventQueue { events: Vec::new() }
    }
    pub fn push(&mut self, event: Event) {
        self.events.push(event);
    }
    pub fn pop(&mut self) -> Option<Event> {
        self.events.pop()
    }
    pub fn iter(&self) -> std::slice::Iter<Event> {
        self.events.iter()
    }
}

pub trait GenericEventListener {
    fn on_event(&mut self, _event: Event) -> bool {
        false
    }
}

pub trait EventListener: AsEventListener {
    // app
    fn on_app_tick(&mut self) -> bool {
        false
    }
    fn on_app_update(&mut self) -> bool {
        false
    }
    fn on_app_render(&mut self) -> bool {
        false
    }

    // window
    fn on_window_close(&mut self) -> bool {
        false
    }
    fn on_window_resize(&mut self, _width: u32, _height: u32) -> bool {
        false
    }

    // keyboard
    fn on_key_press(&mut self, _key: Key, _mods: Modifier) -> bool {
        false
    }
    fn on_key_release(&mut self, _key: Key, _mods: Modifier) -> bool {
        false
    }

    // mouse
    fn on_mouse_press(&mut self, _button: MouseButton, _mods: Modifier) -> bool {
        false
    }
    fn on_mouse_release(&mut self, _button: MouseButton, _mods: Modifier) -> bool {
        false
    }
    fn on_mouse_move(&mut self, _x: f64, _y: f64) -> bool {
        false
    }
    fn on_mouse_scroll(&mut self, _xoffset: f64, _yoffset: f64) -> bool {
        false
    }
}

pub fn propagate_event(event: Event, listener: &mut dyn EventListener) -> bool {
    match event {
        Event::App(AppEvent::Tick) => (*listener).on_app_tick(),
        Event::Window(WindowEvent::Close) => (*listener).on_window_close(),
        Event::Window(WindowEvent::Resize(w, h)) => (*listener).on_window_resize(w, h),
        Event::Keyboard(event) => match event {
            KeyboardEvent::Press(key, mods) => (*listener).on_key_press(key, mods),
            KeyboardEvent::Release(key, mods) => (*listener).on_key_release(key, mods),
        },
        Event::Mouse(event) => match event {
            MouseEvent::Press(button, mods) => (*listener).on_mouse_press(button, mods),
            MouseEvent::Release(button, mods) => (*listener).on_mouse_release(button, mods),
            MouseEvent::Move(x, y) => (*listener).on_mouse_move(x, y),
            MouseEvent::Scroll(xoffset, yoffset) => (*listener).on_mouse_scroll(xoffset, yoffset),
        },
        _ => false,
    }
}

pub trait AsEventListener {
    fn as_event_listener(&mut self) -> &mut dyn EventListener;
}

impl<T: EventListener> AsEventListener for T {
    fn as_event_listener(&mut self) -> &mut dyn EventListener {
        self
    }
}
