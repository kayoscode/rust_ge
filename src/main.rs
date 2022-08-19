use core_engine::{self, engine::GameManager, shader_program::{ShaderProgram, ShaderUniforms}, mesh::{Mesh2D, DrawableMesh}};
use glmath::glmath::Vec2f;
use timer::Stopwatch;

fn main() {
    let game_manager = GameManager::from_conf
        ("./res", "app_config.js");

    match game_manager {
        Some(mut game_manager) => {
            // Create a shader.
            let gui_shader = ShaderProgram::open_shaders("./res/shaders/GUIShader.vert", 
                "./res/shaders/GUIShader.frag").unwrap();

            // Create a mesh.
            let vertices = vec![
                -1.0, -1.0,
                0.0, 1.0,
                1.0, -1.0
            ];

            let mut mesh: Mesh2D = Mesh2D::new();
            mesh.add_float_buffer(vertices, 2);

            let game_window = game_manager.get_window_mut();

            gui_shader.bind();

            let location_pos = gui_shader.get_uniform_location("pos");
            let location_scale = gui_shader.get_uniform_location("scale");

            let scale = Vec2f::new(0.5, 0.5);

            gui_shader.load_vec2(location_pos, glmath::glmath::Vec2f::new(0.0, 0.0));
            gui_shader.load_vec2(location_scale, scale);

            let mut frame_timer: Stopwatch = Stopwatch::new();

            let mut frame_count = 0;
            while !game_window.update_window() {
                mesh.render();
                frame_count += 1;

                if frame_timer.elapsed_seconds() > 1.0 {
                    frame_timer.start();
                    println!("{}", frame_count);
                    frame_count = 0;
                }
            }
        },
        None => {
            println!("Failed to load app config.");
        }
    }
}