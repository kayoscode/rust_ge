use core_engine::{self, engine::GameManager, shader_program::{ShaderProgram, ShaderUniforms}, mesh::{Mesh2D, DrawableMesh}, texture::Texture, MouseKeyboardInputControl};
use glmath::glmath::Vec2f;
use core_engine::render_pipeline::*;
use rand::Rng;

struct SnakeRenderPipeline {
    background_mesh: Mesh2D,
    gui_shader: ShaderProgram,
    bg_texture: Texture,
    pos: Vec<Vec2f>,
    tile_size: f32,
    movement_direction: Vec2f,
    location_pos: i32,
    speed: i32,
    update_count: i32,
    next_segment_pos: Option<Vec2f>,
    game_over: bool,
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
            tile_size: 0.08,
            pos: vec![Vec2f::new(0.0, 0.0)],
            movement_direction: Vec2f::new(0.0, 1.0),
            location_pos: 0,
            speed: 4,
            update_count: 0,
            next_segment_pos: None,
            game_over: false,
        }
    }

    /// Spawns a new segment somewhere on the map.
    /// Can be anywhere except on one of the snake positions.
    fn spawn_segment(&mut self) {
        let mut thread_rng = rand::thread_rng();
        let lower_range = (-1.0 / self.tile_size) as i32;
        let upper_range = -lower_range; 

        let x = thread_rng.gen_range(lower_range..=upper_range);
        let y = thread_rng.gen_range(lower_range..=upper_range);

        self.next_segment_pos = Some(Vec2f::new(x as f32 * self.tile_size, y as f32 * self.tile_size));
    }

    /// Checks whether the position collides with the square.
    fn check_collision(&mut self, pos: Vec2f, sq: Vec2f) -> bool {
        (pos.x >= sq.x - self.tile_size / 2.0) && (pos.x <= sq.x + self.tile_size / 2.0) && 
            (pos.y >= sq.y - self.tile_size / 2.0) && (pos.y <= sq.y + self.tile_size / 2.0)
    }
}

impl RenderPipelineHandler for SnakeRenderPipeline {
    fn init(&mut self) {
        self.gui_shader.bind();

        self.location_pos = self.gui_shader.get_uniform_location("pos");
        let location_scale = self.gui_shader.get_uniform_location("scale");
        let location_gui_texture = self.gui_shader.get_uniform_location("guiTexture");

        self.gui_shader.load_vec2(location_scale, glmath::glmath::Vec2f::new(self.tile_size / 2.0, self.tile_size / 2.0));
        self.gui_shader.load_int(location_gui_texture, self.bg_texture.texture_id() as i32);
    }

    fn prepare(&self) {
        self.gui_shader.bind();
    }

    fn execute(&self) {
        // Render each snake segment.
        for segment in self.pos.iter() {
            self.gui_shader.load_vec2(self.location_pos, *segment);
            self.background_mesh.render();
        }

        // Render the target segment.
        match self.next_segment_pos {
            Some(segment_pos) => {
                self.gui_shader.load_vec2(self.location_pos, segment_pos);
                self.background_mesh.render();
            }
            _ => {}
        }
    }

    fn update(&mut self, input: &Box<dyn MouseKeyboardInputControl>) {
        self.update_count += 1;
        if self.update_count >= self.speed && !self.game_over {
            let mut previous_head = self.pos[0];

            self.update_count = 0;

            self.pos[0] += self.movement_direction * self.tile_size;

            let half_tile_size = self.tile_size / 2.0;
            if self.pos[0].x > 1.0 - half_tile_size {
                self.pos[0].x = -1.0 + half_tile_size;
            }
            else if self.pos[0].x < -1.0 + half_tile_size {
                self.pos[0].x = 1.0 - half_tile_size;
            }

            if self.pos[0].y > 1.0 - half_tile_size {
                self.pos[0].y = -1.0 + half_tile_size;
            }
            else if self.pos[0].y < -1.0 + half_tile_size {
                self.pos[0].y = 1.0 - half_tile_size;
            }

            // Update other segments, and check for self collisions.
            for i in 1..self.pos.len() {
                let temp = self.pos[i];
                self.pos[i] = previous_head;
                previous_head = temp;

                if self.check_collision(self.pos[0], self.pos[i]) {
                    self.game_over = true;
                }
            }

            // Check for collisions with food.
            match self.next_segment_pos {
                None => self.spawn_segment(),
                Some(segment_pos) => {
                    if self.check_collision(self.pos[0], segment_pos) {
                        self.pos.push(previous_head);
                        self.spawn_segment();
                    }
                }
            }
        }

        // Update new input.
        if (input.is_key_down(core_engine::Key::W) || input.is_key_clicked(core_engine::Key::W)) && (self.pos.len() == 1 || self.movement_direction.y != -1.0 ) {
            self.movement_direction = Vec2f::new(0.0, 1.0);
        }

        if (input.is_key_down(core_engine::Key::S) || input.is_key_clicked(core_engine::Key::S)) && (self.pos.len() == 1 || self.movement_direction.y != 1.0) {
            self.movement_direction = Vec2f::new(0.0, -1.0);
        }

        if (input.is_key_down(core_engine::Key::A) || input.is_key_clicked(core_engine::Key::A)) && (self.pos.len() == 1 || self.movement_direction.x != 1.0) {
            self.movement_direction = Vec2f::new(-1.0, 0.0);
        }

        if (input.is_key_down(core_engine::Key::D) || input.is_key_clicked(core_engine::Key::D)) && (self.pos.len() == 1 || self.movement_direction.x != -1.0) {
            self.movement_direction = Vec2f::new(1.0, 0.0);
        }
    }
}

fn main() {
    let game_manager = GameManager::from_conf
        ("./res", "app_config.json");

    match game_manager {
        Some(mut game_manager) => {
            // Create a shader.

            let pipeline = SnakeRenderPipeline::new(&game_manager);
            game_manager.add_render_pipeline(Box::new(pipeline));
            game_manager.init();

            while !game_manager.update() {
            }
        },
        None => {
            println!("Failed to load app config.");
        }
    }
}