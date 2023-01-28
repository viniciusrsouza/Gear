macro_rules! BIT {
    ($x:expr) => {
        1 << $x
    };
}

#[derive(Debug, Copy, Clone)]
pub enum Event {
    App(AppEvent),
    Window(WindowEvent),
    Keyboard(KeyboardEvent),
    Mouse(MouseEvent),
    None,
}

#[derive(Debug, Copy, Clone)]
pub enum AppEvent {
    Tick,
}

#[derive(Debug, Copy, Clone)]
pub enum WindowEvent {
    Close,
    Resize(u32, u32),
    Focus(bool),
}

#[derive(Debug, Copy, Clone)]
pub enum KeyboardEvent {
    Press(Key, Modifier),
    Release(Key, Modifier),
    Repeat(Key, Modifier),
}

#[derive(Debug, Copy, Clone)]
pub enum MouseEvent {
    Press(MouseButton, Modifier),
    Release(MouseButton, Modifier),
    Repeat(MouseButton, Modifier),
    Move(f64, f64),
    Scroll(f64, f64),
}

#[derive(Debug, Copy, Clone)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Button4,
    Button5,
    Button6,
    Button7,
    Button8,
    Unknown,
}

pub type Modifier = u8;

// bit modifiers
pub mod modifiers {
    use super::Modifier;

    pub const SHIFT: Modifier = BIT!(0);
    pub const CTRL: Modifier = BIT!(1);
    pub const ALT: Modifier = BIT!(2);
    pub const SUPER: Modifier = BIT!(3);
}

#[rustfmt::skip]
#[derive(Debug, Copy, Clone)]
pub enum Key {
  // letters
  A, B, C, D, E, F, G, H, I, J, K, L, M, 
  N, O, P, Q, R, S, T, U, V, W, X, Y, Z,

  // numbers
  Num0, Num1, Num2, Num3, Num4, Num5, 
  Num6, Num7, Num8, Num9, NumPad0, NumPad1,
  NumPad2, NumPad3, NumPad4, NumPad5, NumPad6,
  NumPad7, NumPad8, NumPad9,

  // function keys
  F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,

  // special keys
  Escape, LControl, LShift, LAlt, LSuper,
  RControl, RShift, RAlt, RSuper, Menu,
  LBracket, RBracket, Semicolon, Comma, Period,
  Quote, Slash, Backslash, Tilde, Equal, Minus,
  Plus, Space, Return, Backspace, Tab, PageUp, 
  PageDown, End, Home, Insert, Delete, Left, 
  Right, Up, Down, NumpadComma, Pause, Enter,
  Apostrophe, CapsLock, PrintScreen, ScrollLock,
  GraveAccent, NumLock, NumPadDivide, NumPadMultiply,
  NumPadSubtract, NumPadAdd, NumPadEnter, NumPadEqual,
  NumPadDecimal,
  
  // unknown
  Unknown,
}
