use crate::render_pipeline::RenderPipelineHandler;
use crate::resource_manager::{ResourceManager, ResourceDestroy};
use crate::texture::{Texture};
use crate::mesh::{Mesh2D};
use crate::shader_program::{ShaderProgram};

use ogl33::glClearColor;
// External dependencies.
use timer::Stopwatch;
use window::window::*;
use serializers::json::lexer::*;
use serializers::json::parser::*;

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

impl ResourceDestroy for Framebuffer {
    fn destroy(&mut self) {
    }
}

/// Stores information loaded by the engine.
/// This may be sourced from the config files, or from a resource loader object.
/// Either way, all these objects can be globally accessed across the engine.
/// More resources can be loaded dynamically by calling the load resources method again with a separate resource loader.
pub struct GameResources {
    pub texture_resources: ResourceManager<Texture>,
    pub shader_resouces: ResourceManager<ShaderProgram>,
    pub mesh_resources: ResourceManager<Mesh2D>,
    pub framebuffer_resources: ResourceManager<Framebuffer>,

    /// Holds the path from which the resource files should be loaded.
    res_path: String
}

impl Default for GameResources {
    fn default() -> Self {
        GameResources { 
            texture_resources: ResourceManager::new("Textures"), 
            shader_resouces: ResourceManager::new("Shaders"), 
            mesh_resources: ResourceManager::new("Meshes"),
            framebuffer_resources: ResourceManager::new("Framebuffers"), 
            res_path: String::from("")
        }
    }
}

pub struct GameManager {
    /// Holds a render pipeline object.
    render_pipelines: Vec<Box<dyn RenderPipelineHandler>>,

    /// Holds the global game resources loaded by the implementation.
    pub resources: GameResources,

    /// The currently active render pipeline.
    active_pipeline: Option<usize>,

    /// Holds a control to the window.
    /// Note: the window is at the bottom of the list of members because Drop should be called last.
    /// Not doing so will result in invalid opengl calls.
    window: Box<dyn WindowControl>,

    /// Holds a controller for the keyboard and mouse input.
    input: Box<dyn MouseKeyboardInputControl> 
}

impl GameManager {
    /// Creates a new game manager from self defined settings.
    pub fn new(window_conf: GameConfig) -> Option<Self> {
        let window = window::window::GraphicsWindow::new(&window_conf);

        Some(GameManager {
            window: Box::new(window),
            resources: GameResources::default(),
            render_pipelines: Vec::<Box<dyn RenderPipelineHandler>>::default(),
            active_pipeline: None,
            input: Box::new(MouseKeyboardInput::new())
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
        let full_conf_name = res_path.to_string() + "/" + config_file_name;
        
        let json_lexer = JsonLexer::new(&full_conf_name);

        match json_lexer {
            Ok(mut json_lexer) => {
                let user_config = parse_json(&mut json_lexer);

                if let Some(user_config) = user_config {
                    let config = load_user_config(&user_config);
                    let engine = Self::new(config);

                    // Load game resources.
                    match engine {
                        Some(mut game_manager) => {
                            game_manager.resources.res_path = res_path.to_string();

                            // Load data from the "resources" object into the resource manager.
                            parse_config_resources(&user_config, &mut game_manager.resources);
                            return Some(game_manager);
                        }
                        None => return None
                    }
                }

                None 
            }
            _ => {
                println!("Unable to load config file: {}", &full_conf_name);
                None
            }
        }
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

    pub fn add_render_pipeline(&mut self, pipeline: Box<dyn RenderPipelineHandler>) {
        self.render_pipelines.push(pipeline);

        match self.active_pipeline {
            Some(_) => {},
            None => self.set_active_render_pipeline(0)
        }
    }

    pub fn init(&mut self) {
        for pipeline in self.render_pipelines.iter_mut() {
            pipeline.init();
        }
    }

    pub fn set_active_render_pipeline(&mut self, index: usize) {
        self.active_pipeline = Some(index);
    }
    
    pub fn update(&mut self) -> bool {
        let should_close = self.window.update_window();
        self.input.update_input();

        match self.active_pipeline {
            Some(active) => {
                let render_pipeline = self.render_pipelines.get_mut(active);

                match render_pipeline {
                    Some(render_pipeline) => {
                        render_pipeline.update(&self.input, 0.0);
                        render_pipeline.prepare();
                        render_pipeline.render();
                    },
                    _ => {}
                }
            },
            None => {}
        }

        return should_close;
    }
}

/// Parses resources from the config file into named game resources.
fn parse_config_resources(user_config: &JsonNode, game_resources: &mut GameResources) {
    match user_config {
        JsonNode::Object(entire_object) => {
            match entire_object.get("resources") {
                Some(JsonNode::Object(resources_object)) => {
                    // Load textures.
                    match resources_object.get("textures") {
                        Some(JsonNode::Object(textures_object)) => {
                            load_textures(textures_object, &mut game_resources.texture_resources, &game_resources.res_path);
                        }
                        _ => {}
                    }

                    // Load shaders.
                    match resources_object.get("shaders") {
                        Some(JsonNode::Object(shaders_object)) => {
                            load_shaders(shaders_object, &mut game_resources.shader_resouces, &game_resources.res_path);
                        }
                        _ => {}
                    }
                    // Load framebuffers.
                    // Load models.
                }
                _ => {}
            }
        },
        // No object specified: nothing to load.
        _ => {}
    }
}

/// load textures in textures_object into memory with the given name.
fn load_textures(textures_object: &JsonObject, texture_resources: &mut ResourceManager<Texture>, res_path: &str) {
    let stopwatch = Stopwatch::new();

    for (texture, path) in textures_object.iter() {
        match path {
            JsonNode::String(texture_file_path) => {
                // Load the texture, otherwise warning.
                let texture_path = res_path.to_string() + "/" + texture_file_path.get();
                let loaded_texture = Texture::open(&texture_path);

                match loaded_texture {
                    Ok(loaded_texture) => {
                        // Transfer ownership of the loaded texture to the registry.
                        texture_resources.add_registry(texture, loaded_texture);
                        //println!("Loaded texture: {} {}", texture, texture_path);
                    }
                    _ => {
                        println!("Failed to load texture: {} {}", texture, texture_file_path.get());
                    }
                }
            },
            _ => {}
        }
    }

    println!("Loaded game textures in {} seconds", stopwatch.elapsed_seconds());
}

fn load_shaders(shaders_object: &JsonObject, shader_resources: &mut ResourceManager<ShaderProgram>, res_path: &str) {
    let stopwatch = Stopwatch::new();

    for (shader_name, shader_data) in shaders_object.iter() {
        match shader_data {
            JsonNode::Object(shader_data_object) => {
                // Load the shader program if we can.
                if let Some(JsonNode::String(vertex_shader_path)) = shader_data_object.get("vertex") {
                    if let Some(JsonNode::String(fragment_shader_path)) = shader_data_object.get("fragment") {
                        let vs_shader = res_path.to_string() + "/" + vertex_shader_path.get();
                        let fs_shader = res_path.to_string() + "/" + fragment_shader_path.get();
                        let shader_program = ShaderProgram::open_shaders(&vs_shader, &fs_shader);

                        // If we successfully load a shader, attempt to load the next one.
                        match shader_program {
                            Ok(shader_program) => {
                                shader_resources.add_registry(shader_name, shader_program);
                                continue;
                            }
                            _ => {}
                        }
                    }
                }
            }
            _ => {}
        }

        println!("Expected JsonObject with \"vertex\": path, and \"fragment\": path in loaded shader definition in {}", shader_name);
    }

    println!("Loaded game shaders in {} seconds", stopwatch.elapsed_seconds());
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
fn load_user_config(user_config: &JsonNode) -> GameConfig {
    let mut config = GameConfig::default();

    match user_config {
        // At the base layer, we expect to see just an object.
        // It should be an object of its own with a window_config key holding all the info about the window.
        JsonNode::Object(file_object) => {
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

                    if let Some(JsonNode::Bool(resizable)) = window_object.get("resizable") {
                        config.resizable = *resizable.get();
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

                    match graphics_object.get("vsync") {
                        Some(JsonNode::Bool(vsync_setting)) => {
                            config.vsync = *vsync_setting.get()
                        },
                        _ => {}
                    }
                },
                // If we don't have a graphics branch, don't try to load it.
                _ => {}
            }
        },
        _ => {}
    }

    config
}
