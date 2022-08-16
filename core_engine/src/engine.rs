use std::str::FromStr;

use crate::resource_manager::{ResourceManager};
use crate::texture::{Texture};
use crate::mesh::{Mesh2D};
use crate::shader_program::{ShaderProgram};

use ogl33::glClearColor;
// External dependencies.
use timer::Stopwatch;
use window::window::*;
use serializers::json::lexer::*;
use serializers::json::parser::*;

#[derive(Default)]
pub struct WindowClearColor {
    r: f32,
    g: f32,
    b: f32
}

pub struct GameConfig {
    xres: u32,
    yres: u32,
    clear_color: WindowClearColor,
    title: String
}

impl Default for GameConfig {
    fn default() -> Self {
        GameConfig { 
            xres: 1920, 
            yres: 1080, 
            title: "Game Title".to_string(),
            clear_color: WindowClearColor::default()
        }
    }
}

/// Each implementation of the framework needs to 
/// create a module to load resources. While the calling of this
/// interface is handled by the engine, we need the definition of the
/// resources to be defined by the impl.
pub trait ResourceLoader {
    fn load_textures(&self, texture_resources: &mut ResourceManager<Texture>);
    fn load_shaders(&self, shader_resources: &mut ResourceManager<ShaderProgram>);
    fn load_meshes(&self, mesh_resources: &mut ResourceManager<Mesh2D>);
    fn load_frame_buffers(&self, framebuffer_resources: &mut ResourceManager<Framebuffer>);
}

#[derive(Default)]
pub struct Framebuffer {
}

#[derive(Default)]
pub struct GameResources {
    texture_resources: ResourceManager<Texture>,
    shader_resouces: ResourceManager<ShaderProgram>,
    mesh_resources: ResourceManager<Mesh2D>,
    framebuffer_resources: ResourceManager<Framebuffer>,
}

pub struct GameManager {
    /// Times exactly how long a frame took.
    /// For now, all we need is a render timer, if we choose to create an 
    /// update thread, we will need an update_timer as well.
    pub render_timer: Stopwatch,

    window: Box<dyn WindowControl>,

    /// Holds the path from which the resource files should be loaded.
    pub res_path: String,

    /// Holds the global game resources loaded by the implementation.
    resources: GameResources
}

impl GameManager {
    /// Creates a new game manager from self defined settings.
    pub fn new(window_conf: GameConfig) -> Option<Self> {
        let window = window::window::GraphicsWindow::new(
            window_conf.xres,
            window_conf.yres,
            &window_conf.title
        );

        // Set the window clear color.
        Self::set_clear_color(window_conf.clear_color.r, window_conf.clear_color.g, window_conf.clear_color.b);

        Some(GameManager {
            render_timer: Stopwatch::new(),
            window: Box::new(window),
            res_path: String::from_str("./res").unwrap(),
            resources: GameResources::default()
        })
    }

    /// Loads global game resources from the implementation.
    pub fn load_game_resources(&mut self, resource_loader: &dyn ResourceLoader) {
        resource_loader.load_frame_buffers(&mut self.resources.framebuffer_resources);
        resource_loader.load_shaders(&mut self.resources.shader_resouces);
        resource_loader.load_textures(&mut self.resources.texture_resources);
        resource_loader.load_meshes(&mut self.resources.mesh_resources);
    }

    /// Loads the game manager from an app config file.
    pub fn from_conf(res_path: &str, config_file_name: &str) -> Option<Self> {
        // Test load img.
        let full_conf_name = res_path.to_string() + "/" + config_file_name;
        let config = load_user_config(&full_conf_name);

        Self::new(config)
    }

    pub fn terminate_program(&mut self) {
        self.window.close_window()
    }

    /// Sets the clear color of the active window.
    pub fn set_clear_color(r: f32, g: f32, b: f32) {
        unsafe {
            glClearColor(r, g, b, 1.0);
        }
    }

    /// Returns an immutable ref to the window.
    pub fn get_window(&self) -> &Box<dyn WindowControl> {
        &self.window
    }

    pub fn get_window_mut(&mut self) -> &mut Box<dyn WindowControl> {
        &mut self.window
    }
}


/// Loads an app config. When I learn how macros work, determine a way
/// to set the following attributes 
/// path: ex("/window/xres")
/// valid_types: ex("bool, Number") -> should be implemented as multiple struct members.
///     ex: Option<xpos_number>: i32,
///         Option<xpos_string>: String
///     This becomes important when setting it to a boolean value negates the 
///     need for an integer value. 
/// required: ex(true) -> simply tells the system whether the setting must be included in the file.
/// default_value: ex(1920)
/// ^^ TODO
fn load_user_config(app_config: &str) -> GameConfig {
    let mut config = GameConfig::default();

    let json_lexer = JsonLexer::new(app_config);

    match json_lexer {
        Ok(mut json_lexer) => {
            let user_config = parse_json(&mut json_lexer);

            match user_config {
                // At the base layer, we expect to see just an object.
                // It should be an object of its own with a window_config key holding all the info about the window.
                Some(JsonNode::Object(file_object)) => {
                    match file_object.get("window_config") { 
                        Some(JsonNode::Object(window_object)) => {
                            // Load all the window data.
                            if let Some(JsonNode::Number(xres)) = window_object.get("xres") {
                                config.xres = *xres.get() as u32;
                            }

                            if let Some(JsonNode::Number(yres)) = window_object.get("yres") {
                                config.yres = *yres.get() as u32;
                            }

                            if let Some(JsonNode::String(title)) = window_object.get("title") {
                                config.title = title.get().clone();
                            }
                        }
                        // The window config is not required.
                        _ => {}
                    }

                    // Load the graphics configuration.
                    match file_object.get("graphics") {
                        Some(JsonNode::Object(graphics_object)) => {
                            // Load the clear color attribute.
                            match graphics_object.get("clear_color") {
                                Some(JsonNode::Object(clear_color_object)) => {
                                    if let Some(JsonNode::Float(r)) = clear_color_object.get("r") {
                                        config.clear_color.r = *r.get() as f32;
                                    }

                                    if let Some(JsonNode::Float(g)) = clear_color_object.get("g") {
                                        config.clear_color.g = *g.get() as f32;
                                    }

                                    if let Some(JsonNode::Float(b)) = clear_color_object.get("b") {
                                        config.clear_color.b = *b.get() as f32;
                                    }
                                }
                                _ => {}
                            }
                        },
                        // If we don't have a graphics branch, don't try to load it.
                        _ => {}
                    }
                },
                _ => {}
            }
        }
        _ => {}
    }

    config
}
