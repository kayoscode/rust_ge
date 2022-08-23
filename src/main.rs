use core_engine::{self, engine::GameManager, shader_program::{ShaderProgram, ShaderUniforms}, mesh::{Mesh2D, DrawableMesh}, texture::Texture};
use glmath::glmath::Vec2f;
use timer::Stopwatch;
use core_engine::render_pipeline::*;

struct SnakeRenderPipeline {
    background_mesh: Mesh2D,
    gui_shader: ShaderProgram,
    bg_texture: Texture
}

impl SnakeRenderPipeline {
    pub fn new() -> SnakeRenderPipeline {
        // Create a mesh.
        let vertices = vec![
            -1.0, -1.0,
            -1.0, 1.0,
            1.0, 1.0,
            1.0, 1.0,
            1.0, -1.0,
            -1.0, -1.0
        ];

        let mut mesh: Mesh2D = Mesh2D::new();
        mesh.add_float_buffer(vertices, 2);

        SnakeRenderPipeline { 
            background_mesh: mesh,
            gui_shader: ShaderProgram::open_shaders(
                "./res/shaders/GUIShader.vert", 
                "./res/shaders/GUIShader.frag").unwrap(), 
            bg_texture: Texture::open("./res/textures/snake_bg.png").unwrap()
        }
    }
}

impl RenderPipelineHandler for SnakeRenderPipeline {
    fn init(&self) {
        self.gui_shader.bind();

        let location_pos = self.gui_shader.get_uniform_location("pos");
        let location_scale = self.gui_shader.get_uniform_location("scale");
        let location_gui_texture = self.gui_shader.get_uniform_location("guiTexture");

        let scale = Vec2f::new(0.98, 0.98);

        self.gui_shader.load_vec2(location_pos, glmath::glmath::Vec2f::new(0.0, 0.0));
        self.gui_shader.load_vec2(location_scale, scale);
        self.gui_shader.load_int(location_gui_texture, self.bg_texture.texture_id() as i32);
    }

    fn prepare(&self) {
        self.gui_shader.bind();
    }

    fn execute(&self) {
        self.background_mesh.render();
    }
}

fn main() {
    let game_manager = GameManager::from_conf
        ("./res", "app_config.js");

    match game_manager {
        Some(mut game_manager) => {
            // Create a shader.
            let pipeline = SnakeRenderPipeline::new();
            game_manager.add_render_pipeline(Box::new(pipeline));
            game_manager.init();

            let mut frame_timer: Stopwatch = Stopwatch::new();
            let mut frame_count = 0;

            while !game_manager.update() {
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