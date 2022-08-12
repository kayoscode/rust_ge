use std::vec;

use glfw::ffi::GLFWwindow;

/// Mouse buttons. The `MouseButtonLeft`, `MouseButtonRight`, and
/// `MouseButtonMiddle` aliases are supplied for convenience.
#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum MouseButton {
    /// The left mouse button. A `MouseButtonLeft` alias is provided to improve clarity.
    Button1 = glfw::ffi::MOUSE_BUTTON_1,
    /// The right mouse button. A `MouseButtonRight` alias is provided to improve clarity.
    Button2 = glfw::ffi::MOUSE_BUTTON_2,
    /// The middle mouse button. A `MouseButtonMiddle` alias is provided to improve clarity.
    Button3 = glfw::ffi::MOUSE_BUTTON_3,
    Button4 = glfw::ffi::MOUSE_BUTTON_4,
    Button5 = glfw::ffi::MOUSE_BUTTON_5,
    Button6 = glfw::ffi::MOUSE_BUTTON_6,
    Button7 = glfw::ffi::MOUSE_BUTTON_7,
    Button8 = glfw::ffi::MOUSE_BUTTON_8,
}

/// Stores information about pressed keys
/// Nothing is threaded, so this is safe to do.
pub struct MouseButtonInputs {
    pub buttons_pressed_frame: Vec<i32>,
    pub buttons_released_frame: Vec<i32>
}

pub static mut MOUSE_BUTTON_INPUTS: MouseButtonInputs = MouseButtonInputs { buttons_pressed_frame: vec![], buttons_released_frame: vec![] };

pub extern "C" fn mouse_callback(_window: *mut GLFWwindow, button: i32, action: i32, _mods: i32) {
    unsafe {
        match action {
            _ if action == glfw::Action::Press as i32 => {
                MOUSE_BUTTON_INPUTS.buttons_pressed_frame.push(button);
            },
            _ if action == glfw::Action::Release as i32 => {
                MOUSE_BUTTON_INPUTS.buttons_released_frame.push(button);
            },
            // The only other option is repeat which we don't have to do
            // anything for.
            _ => ()
        }
    }
}