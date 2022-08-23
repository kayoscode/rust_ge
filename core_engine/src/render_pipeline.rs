/// Renders an individual stage in the render pipeline with its own isolated 
/// state. An object implementing this should be given to the engine at
/// the application launch time. This handles the program's execution flow.
pub trait RenderPipelineHandler {
    fn render(&self) {
        self.prepare();
        self.execute();
    }

    fn init(&self);
    fn prepare(&self);
    fn execute(&self);
}