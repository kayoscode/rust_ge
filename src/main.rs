use ogl33::*;
use window::window::*;
use timer::{Stopwatch, Timer};

fn main() {
    let mut stopwatch = Stopwatch::new();
    stopwatch.start();
    let mut window = GraphicsWindow::new(1920, 1080, "Game Window");

    stopwatch.stop();
    println!("duration: {}", stopwatch.elapsed_millis());

    while !window.update_window() {
        if window.is_key_clicked(window::Key::Escape) {
            window.close_window();
        }

        unsafe {
            glClearColor(0.0, 0.0, 0.0, 0.0);
            glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
        }
    }
}