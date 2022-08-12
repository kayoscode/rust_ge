use ogl33::*;
use window::window::*;

fn main() {
    let mut window = GraphicsWindow::new(1920, 1080, "Game Window");

    while !window.update_window() { 
        unsafe {
            if window.is_key_clicked(window::Key::Escape) {
                window.close_window();
            }

            glClearColor(0.0, 0.0, 0.0, 0.0);
            glClear(GL_COLOR_BUFFER_BIT);
        }
    }
}