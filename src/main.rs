use core_engine::{self, engine::GameManager, shader_program::{ShaderProgram, ShaderUniforms}, mesh::{Mesh2D, DrawableMesh}};

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

            gui_shader.load_vec2(location_pos, glmath::glmath::Vec2f::new(0.0, 0.0));
            gui_shader.load_vec2(location_scale, glmath::glmath::Vec2f::new(0.5, 0.5));

            while !game_window.update_window() {
                if game_window.is_key_clicked(core_engine::Key::Escape) {
                    game_window.close_window();
                }

                mesh.render();
            }
        }
        None => {
            println!("Failed to load app config.");
        }
    }
}