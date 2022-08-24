use image::GenericImageView;
use ogl33::*;

use crate::resource_manager::ResourceDestroy;

#[derive(Default, Clone)]
pub struct Texture {
    diffuse_id: GLuint
}

impl Texture {
    pub fn texture_id(&self) -> u32 {
        return self.diffuse_id;
    }

    /// Binds the texture to a specified index.
    pub fn bind(&self, index: i32) {
        unsafe {
            glActiveTexture(GL_TEXTURE0 + index as GLuint);
            glBindTexture(GL_TEXTURE_2D, self.diffuse_id);
        }
    }

    /// Loads the texture from a file.
    pub fn open(texture_path: &str) -> Result<Self, image::ImageError> {
        let img = image::open(texture_path);

        return match img {
            Ok(img) => {
                // Load it into a texture object and return.
                unsafe {
                    let mut texture: GLuint = 0;
                    ogl33::glGenTextures(1, &mut texture);
                    ogl33::glBindTexture(GL_TEXTURE_2D, texture);

                    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_REPEAT as GLint);	
                    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_REPEAT as GLint);
                    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR_MIPMAP_LINEAR as GLint);
                    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR as GLint);

                    // For simplicity, we will assume there are either four or three channels.
                    // All RGB(A)
                    if img.color().channel_count() == 4 {
                        glTexImage2D(GL_TEXTURE_2D, 0, GL_RGBA as GLint, 
                            img.dimensions().0 as i32, img.dimensions().1 as i32, 0, 
                            GL_RGB, GL_UNSIGNED_BYTE, img.as_bytes().as_ptr() as *const c_void);
                    }
                    else {
                        glTexImage2D(GL_TEXTURE_2D, 0, GL_RGB as GLint, 
                            img.dimensions().0 as i32, img.dimensions().1 as i32, 0, 
                            GL_RGB, GL_UNSIGNED_BYTE, img.as_bytes().as_ptr() as *const c_void);
                    }

                    glGenerateMipmap(GL_TEXTURE_2D);
                    glBindTexture(GL_TEXTURE_2D, 0);

                    Ok(Texture {
                        diffuse_id: texture 
                    })
                }
            },
            Err(err) => Err(err)
        }
    }
}

impl ResourceDestroy for Texture {
    fn destroy(&mut self) {
        unsafe {
            glDeleteTextures(1, &self.diffuse_id);
        }
    }
}