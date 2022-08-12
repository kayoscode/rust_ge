extern crate glfw;

use glfw::{Context, ffi::{glfwGetProcAddress, glfwSetKeyCallback, glfwSetMouseButtonCallback}, Window};
use ogl33::*;

use crate::keyboard_input;
use crate::mouse_input;
const NUM_KEYS_INPUT: usize = 500;
const NUM_MOUSE_BUTTONS: usize = 8;

struct ButtonInputState {
    down: Vec<bool>,
    clicked: Vec<bool>
}

impl ButtonInputState {
    fn update(&mut self, pressed_this_frame: &mut Vec<i32>, released_this_frame: &mut Vec<i32>) {
        // Used memset for highly optimized resetting of the keys pressed.
        unsafe {
            let p_keys_pressed = self.clicked.as_mut_ptr();
            p_keys_pressed.write_bytes(false as u8, self.clicked.len());
        }

        while let Some(button) = pressed_this_frame.pop() {
            self.down[button as usize] = true;
        }

        while let Some(button) = released_this_frame.pop() {
            self.down[button as usize] = false;
            self.clicked[button as usize] = true;
        }
    }
}

pub trait WindowControl {
    /// Updates the state of the input, updates the frame,
    /// and returns true if the window should close.
    fn update_window(&mut self) -> bool;
    fn close_window(&mut self);

    /// Returns true if the key state if the key is 'down'
    fn is_key_down(&mut self, key: keyboard_input::Key) -> bool;
    /// Returns true if the key state is 'pressed'
    fn is_key_clicked(&mut self, key: keyboard_input::Key) -> bool;
    /// Returns true if the key state is 'up'
    fn is_key_up(&mut self, key: keyboard_input::Key) -> bool {
        !self.is_key_down(key)
    }

    // Returns true if the mouse button state is 'down'
    fn is_mouse_down(&mut self, button: mouse_input::MouseButton) -> bool;
    // Returns true if the mouse button state is 'pressed'
    fn is_mouse_clicked(&mut self, button: mouse_input::MouseButton) -> bool;
    // Returns true if the mouse button state is 'released'
    fn is_mouse_up(&mut self, button: mouse_input::MouseButton) -> bool {
        !self.is_mouse_down(button)
    }
}

pub struct GraphicsWindow {
    window: Window,
    keyboard_input: ButtonInputState,
    mouse_button_input: ButtonInputState
}

impl WindowControl for GraphicsWindow {
    fn close_window(&mut self) {
        self.window.set_should_close(true);
    }

    fn update_window(&mut self) -> bool {
        // Update input state.
        self.window.glfw.poll_events();

        unsafe {
            self.keyboard_input.update(&mut keyboard_input::KEY_INPUTS.keys_pressed_frame, 
                &mut keyboard_input::KEY_INPUTS.keys_released_frame);

            self.mouse_button_input.update(&mut mouse_input::MOUSE_BUTTON_INPUTS.buttons_pressed_frame, 
                &mut mouse_input::MOUSE_BUTTON_INPUTS.buttons_released_frame);
        }

        self.window.swap_buffers();

        return self.window.should_close();
    }

    fn is_key_down(&mut self, key: keyboard_input::Key) -> bool {
        self.keyboard_input.down[key as usize]
    }

    fn is_key_clicked(&mut self, key: keyboard_input::Key) -> bool {
        self.keyboard_input.clicked[key as usize]
    }

    fn is_mouse_down(&mut self, button: mouse_input::MouseButton) -> bool {
        self.mouse_button_input.down[button as usize]
    }

    fn is_mouse_clicked(&mut self, button: mouse_input::MouseButton) -> bool {
        self.mouse_button_input.clicked[button as usize]
    }
}

fn load_gl_functions() {
    unsafe {
        load_gl_with(|f_name| 
            glfwGetProcAddress(f_name));
    }
}

impl GraphicsWindow {
    pub fn new(width: u32, height: u32, title: &str) -> GraphicsWindow {
        let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        let (mut window, _events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        window.set_key_polling(true);
        window.make_current();

        // Add gl context to window.
        load_gl_functions();

        // Bind input callbacks.
        unsafe {
            let key_cb: Option<glfw::ffi::GLFWkeyfun> = Some(keyboard_input::key_callback);
            glfwSetKeyCallback(window.window_ptr(), key_cb);

            let mouse_cb: Option<glfw::ffi::GLFWmousebuttonfun> = Some(mouse_input::mouse_callback);
            glfwSetMouseButtonCallback(window.window_ptr(), mouse_cb);
        }

        GraphicsWindow {
            window,
            keyboard_input: ButtonInputState {
                down: vec![false; NUM_KEYS_INPUT],
                clicked: vec![false; NUM_KEYS_INPUT]
            },
            mouse_button_input: ButtonInputState { 
                down: vec![false; NUM_MOUSE_BUTTONS],
                clicked: vec![false; NUM_MOUSE_BUTTONS]
            }
        }
    }
}