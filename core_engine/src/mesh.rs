use std::mem::size_of;

use ogl33::*;

#[derive(Default)]
pub struct Mesh2D {
    draw_count: i32,
    vao: u32,
    vbos: Vec<u32>,
    vbo_dimensions: Vec<u32>
}

impl Mesh2D {
    pub fn new() -> Mesh2D {
        unsafe {
            let mut vao: GLuint = 0;
            glGenVertexArrays(1, &mut vao);

            Mesh2D { 
                draw_count: -1, 
                vao: vao as u32,
                vbos: Vec::<u32>::default(), 
                vbo_dimensions: Vec::<u32>::default() 
            }
        }
    }

    /// Adds a new float buffer to the VBO.
    /// Dimension are the number passed to the shader per render call. 
    /// 3 dims = a 3d vector.
    pub fn add_float_buffer(&mut self, data: Vec<f32>, dimensions: u32) {
        #[cfg(debug_assertions)] 
        assert_eq!(data.len() as u32 % dimensions, 0);

        // Set draw count to what we expect for a 2D mesh. We would hope that all attributes have
        // the same draw count, but we have to use them min.
        if self.draw_count != -1 {
            self.draw_count = u32::min(self.draw_count as u32, data.len() as u32 / dimensions) as i32;

            // In debug mode, we need to notify if all attributes aren't the same size.
            #[cfg(debug_assertions)]
            dbg!("Attribute added with a different draw_count");
        }
        else {
            self.draw_count = (data.len() as u32 / dimensions) as i32;
        }

        // Create and bind the vbo.
        unsafe { 
            glBindVertexArray(self.vao as u32);

            let mut vbo: GLuint = 0;
            glGenBuffers(1, &mut vbo);
            glBindBuffer(GL_ARRAY_BUFFER, vbo);

            // Write data to vbo.
            let index = self.vbos.len();
            let mut size = (self.draw_count as u32 * dimensions) as isize;
            size *= size_of::<f32>() as isize;

            glEnableVertexAttribArray(index as u32);
            glBufferData(GL_ARRAY_BUFFER, size,
                data.as_ptr() as *const c_void, GL_STATIC_DRAW);

            glVertexAttribPointer(index as GLuint, dimensions as i32, 
                GL_FLOAT, GL_FALSE, 0, 0 as *const c_void);

            // Unbind the buffer and append to list of vbos.
            glBindBuffer(GL_ARRAY_BUFFER, 0);

            self.vbos.push(vbo);
            self.vbo_dimensions.push(dimensions);
        }
    }
}

impl Drop for Mesh2D {
    fn drop(&mut self) {
        unsafe {
            // Free the vao and vbos.
            glBindVertexArray(self.vao);
            glDeleteBuffers(self.vbos.len() as i32, self.vbos.as_ptr());
            glDeleteVertexArrays(1, &self.vao);
        }
    }
}

pub trait DrawableMesh {
    fn render(&self);
}

impl DrawableMesh for Mesh2D {
    fn render(&self) {
        unsafe {
            glEnable(GL_BLEND);
            glDisable(GL_DEPTH_TEST);
            glDisable(GL_CULL_FACE);
            glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA);

            glBindVertexArray(self.vao as u32);

            glDrawArrays(GL_TRIANGLES, 0, self.draw_count);

            glDisable(GL_BLEND);
            glEnable(GL_DEPTH_TEST);
            glEnable(GL_CULL_FACE);
            glCullFace(GL_BACK);
        }
    }
}
