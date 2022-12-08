use std::ops::DerefMut;

use super::{AppEvent, Event, Key, KeyboardEvent, Modifier, MouseButton, MouseEvent, WindowEvent};

pub trait EventDispatcher {
    fn dispatch(&mut self, event: Event);
    fn collect(&mut self, listeners: Vec<Box<&mut dyn EventListener>>) -> bool;
}

pub trait EventListener {
    // app
    fn on_app_tick(&mut self) -> bool;
    fn on_app_update(&mut self) -> bool;
    fn on_app_render(&mut self) -> bool;

    // window
    fn on_window_close(&mut self) -> bool;
    fn on_window_resize(&mut self, width: u32, height: u32) -> bool;

    // keyboard
    fn on_key_press(&mut self, key: Key, mods: Modifier) -> bool;
    fn on_key_release(&mut self, key: Key, mods: Modifier) -> bool;

    // mouse
    fn on_mouse_press(&mut self, button: MouseButton, mods: Modifier) -> bool;
    fn on_mouse_release(&mut self, button: MouseButton, mods: Modifier) -> bool;
    fn on_mouse_move(&mut self, x: f64, y: f64) -> bool;
    fn on_mouse_scroll(&mut self, xoffset: f64, yoffset: f64) -> bool;
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

    fn collect_event(&mut self, event: Event, listener: &mut dyn EventListener) -> bool {
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
                MouseEvent::Scroll(xoffset, yoffset) => {
                    (*listener).on_mouse_scroll(xoffset, yoffset)
                }
            },
            _ => false,
        }
    }
}

impl EventDispatcher for EventDispatcherImpl {
    fn dispatch(&mut self, event: Event) {
        self.queue.push(event);
    }

    fn collect(&mut self, mut listeners: Vec<Box<&mut dyn EventListener>>) -> bool {
        let mut result = false;
        while let Some(event) = self.queue.pop() {
            'collectors: for listener in listeners.iter_mut() {
                if self.collect_event(event, listener.deref_mut().deref_mut()) {
                    result = true;
                    break 'collectors;
                }
            }
        }
        result
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
}
