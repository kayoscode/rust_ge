use core_engine::{self, engine::GameManager, shader_program::{ShaderProgram, ShaderUniforms}, mesh::{Mesh2D, DrawableMesh}, texture::Texture, MouseKeyboardInputControl};
use glmath::glmath::Vec2f;
use timer::Stopwatch;
use core_engine::render_pipeline::*;

struct SnakeRenderPipeline {
    background_mesh: Mesh2D,
    gui_shader: ShaderProgram,
    bg_texture: Texture,
    pos: Vec2f,
    movement_direction: Vec2f,
    location_pos: i32
}

impl SnakeRenderPipeline {
    pub fn new(game_manager: &GameManager) -> SnakeRenderPipeline {
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

        let gui_shader = game_manager.resources.shader_resouces.get_registry("shader_game").unwrap().clone();
        let bg_texture = game_manager.resources.texture_resources.get_registry("tex_background").unwrap().clone();

        SnakeRenderPipeline { 
            background_mesh: mesh,
            gui_shader,
            bg_texture,
            pos: Vec2f::new(0.0, 0.0),
            movement_direction: Vec2f::new(0.0, 1.0),
            location_pos: 0
        }
    }
}

impl RenderPipelineHandler for SnakeRenderPipeline {
    fn init(&mut self) {
        self.gui_shader.bind();

        self.location_pos = self.gui_shader.get_uniform_location("pos");
        let location_scale = self.gui_shader.get_uniform_location("scale");
        let location_gui_texture = self.gui_shader.get_uniform_location("guiTexture");

        self.gui_shader.load_vec2(location_scale, glmath::glmath::Vec2f::new(0.10, 0.10));
        self.gui_shader.load_int(location_gui_texture, self.bg_texture.texture_id() as i32);
    }

    fn prepare(&self) {
        self.gui_shader.load_vec2(self.location_pos, self.pos);
        self.gui_shader.bind();
    }

    fn execute(&self) {
        self.background_mesh.render();
    }

    /// TODO: eventually this has to move to a scene controller type.
    /// TODO: allow setting of update tick speed.
    /// TODO: create delta.
    fn update(&mut self, input: &Box<dyn MouseKeyboardInputControl>, _delta: f32) {
        if input.is_key_down(core_engine::Key::W) {
            self.movement_direction = Vec2f::new(0.0, 1.0);
        }
        else if input.is_key_down(core_engine::Key::S) {
            self.movement_direction = Vec2f::new(0.0, -1.0);
        }
        else if input.is_key_down(core_engine::Key::A) {
            self.movement_direction = Vec2f::new(-1.0, 0.0);
        }
        else if input.is_key_down(core_engine::Key::D) {
            self.movement_direction = Vec2f::new(1.0, 0.0);
        }

        self.pos += self.movement_direction * 0.003;
    }
}

fn main() {
    let game_manager = GameManager::from_conf
        ("./res", "app_config.js");

    match game_manager {
        Some(mut game_manager) => {
            // Create a shader.

            let pipeline = SnakeRenderPipeline::new(&game_manager);
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