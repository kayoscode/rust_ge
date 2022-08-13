use std::vec;

use glfw::ffi::GLFWwindow;

#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
/// Enumerates every key supported by the window.
pub enum Key {
    Space = glfw::ffi::KEY_SPACE,
    Apostrophe = glfw::ffi::KEY_APOSTROPHE,
    Comma = glfw::ffi::KEY_COMMA,
    Minus = glfw::ffi::KEY_MINUS,
    Period = glfw::ffi::KEY_PERIOD,
    Slash = glfw::ffi::KEY_SLASH,
    Num0 = glfw::ffi::KEY_0,
    Num1 = glfw::ffi::KEY_1,
    Num2 = glfw::ffi::KEY_2,
    Num3 = glfw::ffi::KEY_3,
    Num4 = glfw::ffi::KEY_4,
    Num5 = glfw::ffi::KEY_5,
    Num6 = glfw::ffi::KEY_6,
    Num7 = glfw::ffi::KEY_7,
    Num8 = glfw::ffi::KEY_8,
    Num9 = glfw::ffi::KEY_9,
    Semicolon = glfw::ffi::KEY_SEMICOLON,
    Equal = glfw::ffi::KEY_EQUAL,
    A = glfw::ffi::KEY_A,
    B = glfw::ffi::KEY_B,
    C = glfw::ffi::KEY_C,
    D = glfw::ffi::KEY_D,
    E = glfw::ffi::KEY_E,
    F = glfw::ffi::KEY_F,
    G = glfw::ffi::KEY_G,
    H = glfw::ffi::KEY_H,
    I = glfw::ffi::KEY_I,
    J = glfw::ffi::KEY_J,
    K = glfw::ffi::KEY_K,
    L = glfw::ffi::KEY_L,
    M = glfw::ffi::KEY_M,
    N = glfw::ffi::KEY_N,
    O = glfw::ffi::KEY_O,
    P = glfw::ffi::KEY_P,
    Q = glfw::ffi::KEY_Q,
    R = glfw::ffi::KEY_R,
    S = glfw::ffi::KEY_S,
    T = glfw::ffi::KEY_T,
    U = glfw::ffi::KEY_U,
    V = glfw::ffi::KEY_V,
    W = glfw::ffi::KEY_W,
    X = glfw::ffi::KEY_X,
    Y = glfw::ffi::KEY_Y,
    Z = glfw::ffi::KEY_Z,
    LeftBracket = glfw::ffi::KEY_LEFT_BRACKET,
    Backslash = glfw::ffi::KEY_BACKSLASH,
    RightBracket = glfw::ffi::KEY_RIGHT_BRACKET,
    GraveAccent = glfw::ffi::KEY_GRAVE_ACCENT,
    World1 = glfw::ffi::KEY_WORLD_1,
    World2 = glfw::ffi::KEY_WORLD_2,

    Escape = glfw::ffi::KEY_ESCAPE,
    Enter = glfw::ffi::KEY_ENTER,
    Tab = glfw::ffi::KEY_TAB,
    Backspace = glfw::ffi::KEY_BACKSPACE,
    Insert = glfw::ffi::KEY_INSERT,
    Delete = glfw::ffi::KEY_DELETE,
    Right = glfw::ffi::KEY_RIGHT,
    Left = glfw::ffi::KEY_LEFT,
    Down = glfw::ffi::KEY_DOWN,
    Up = glfw::ffi::KEY_UP,
    PageUp = glfw::ffi::KEY_PAGE_UP,
    PageDown = glfw::ffi::KEY_PAGE_DOWN,
    Home = glfw::ffi::KEY_HOME,
    End = glfw::ffi::KEY_END,
    CapsLock = glfw::ffi::KEY_CAPS_LOCK,
    ScrollLock = glfw::ffi::KEY_SCROLL_LOCK,
    NumLock = glfw::ffi::KEY_NUM_LOCK,
    PrintScreen = glfw::ffi::KEY_PRINT_SCREEN,
    Pause = glfw::ffi::KEY_PAUSE,
    F1 = glfw::ffi::KEY_F1,
    F2 = glfw::ffi::KEY_F2,
    F3 = glfw::ffi::KEY_F3,
    F4 = glfw::ffi::KEY_F4,
    F5 = glfw::ffi::KEY_F5,
    F6 = glfw::ffi::KEY_F6,
    F7 = glfw::ffi::KEY_F7,
    F8 = glfw::ffi::KEY_F8,
    F9 = glfw::ffi::KEY_F9,
    F10 = glfw::ffi::KEY_F10,
    F11 = glfw::ffi::KEY_F11,
    F12 = glfw::ffi::KEY_F12,
    F13 = glfw::ffi::KEY_F13,
    F14 = glfw::ffi::KEY_F14,
    F15 = glfw::ffi::KEY_F15,
    F16 = glfw::ffi::KEY_F16,
    F17 = glfw::ffi::KEY_F17,
    F18 = glfw::ffi::KEY_F18,
    F19 = glfw::ffi::KEY_F19,
    F20 = glfw::ffi::KEY_F20,
    F21 = glfw::ffi::KEY_F21,
    F22 = glfw::ffi::KEY_F22,
    F23 = glfw::ffi::KEY_F23,
    F24 = glfw::ffi::KEY_F24,
    F25 = glfw::ffi::KEY_F25,
    Kp0 = glfw::ffi::KEY_KP_0,
    Kp1 = glfw::ffi::KEY_KP_1,
    Kp2 = glfw::ffi::KEY_KP_2,
    Kp3 = glfw::ffi::KEY_KP_3,
    Kp4 = glfw::ffi::KEY_KP_4,
    Kp5 = glfw::ffi::KEY_KP_5,
    Kp6 = glfw::ffi::KEY_KP_6,
    Kp7 = glfw::ffi::KEY_KP_7,
    Kp8 = glfw::ffi::KEY_KP_8,
    Kp9 = glfw::ffi::KEY_KP_9,
    KpDecimal = glfw::ffi::KEY_KP_DECIMAL,
    KpDivide = glfw::ffi::KEY_KP_DIVIDE,
    KpMultiply = glfw::ffi::KEY_KP_MULTIPLY,
    KpSubtract = glfw::ffi::KEY_KP_SUBTRACT,
    KpAdd = glfw::ffi::KEY_KP_ADD,
    KpEnter = glfw::ffi::KEY_KP_ENTER,
    KpEqual = glfw::ffi::KEY_KP_EQUAL,
    LeftShift = glfw::ffi::KEY_LEFT_SHIFT,
    LeftControl = glfw::ffi::KEY_LEFT_CONTROL,
    LeftAlt = glfw::ffi::KEY_LEFT_ALT,
    LeftSuper = glfw::ffi::KEY_LEFT_SUPER,
    RightShift = glfw::ffi::KEY_RIGHT_SHIFT,
    RightControl = glfw::ffi::KEY_RIGHT_CONTROL,
    RightAlt = glfw::ffi::KEY_RIGHT_ALT,
    RightSuper = glfw::ffi::KEY_RIGHT_SUPER,
    Menu = glfw::ffi::KEY_MENU,
    Unknown = glfw::ffi::KEY_UNKNOWN,
}

/// Stores information about pressed keys
/// Nothing is threaded, so this is safe to do.
pub struct KeyInputs {
    pub keys_pressed_frame: Vec<i32>,
    pub keys_released_frame: Vec<i32>
}

pub static mut KEY_INPUTS: KeyInputs = KeyInputs { 
    keys_pressed_frame: vec![], 
    keys_released_frame: vec![] 
};

pub extern "C" fn key_callback(_window: *mut GLFWwindow, key: i32, _scancode: i32, action: i32, _mods: i32) {
    unsafe {
        match action {
            _ if action == glfw::Action::Press as i32 => {
                KEY_INPUTS.keys_pressed_frame.push(key);
            },
            _ if action == glfw::Action::Release as i32 => {
                KEY_INPUTS.keys_released_frame.push(key);
            },
            // The only other option is repeat which we don't have to do
            // anything for.
            _ => ()
        }
    }
}