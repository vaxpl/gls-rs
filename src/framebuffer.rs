use crate::gl::{GLenum, GLuint};
use crate::prelude::*;
use crate::{gl, Texture, TextureFormat, TextureLoader};

/// Framebuffer wrapper.
#[derive(Debug)]
pub struct Framebuffer {
    id: GLuint,
}

impl Framebuffer {
    /// Create a Framebuffer Wrapper.
    /// # Note
    /// The object is unbinded after generated.
    pub fn new() -> Self {
        let mut ids: [GLuint; 1] = [0];
        crate::gen_framebuffers(&mut ids);
        crate::bind_framebuffer(gl::FRAMEBUFFER, ids[0]);
        Self { id: ids[0] }
    }

    /// Returns the FrameBuffer object of the screen surface.
    pub fn with_screen() -> Self {
        crate::bind_framebuffer(gl::FRAMEBUFFER, 0);
        Self { id: 0 }
    }

    /// Returns the Id of the Framebuffer.
    pub fn id(&self) -> GLuint {
        self.id
    }

    /// Set color attachment with `texture`.
    pub fn set_color_texture(&self, texture: GLuint) {
        if self.id != 0 {
            // self.bind();
            crate::framebuffer_texture2d(
                gl::FRAMEBUFFER,
                gl::COLOR_ATTACHMENT0,
                gl::TEXTURE_2D,
                texture,
                0,
            );
            // self.unbind();
        }
    }

    /// Set depth attachment with `texture`.
    pub fn set_depth_texture(&self, texture: GLuint) {
        if self.id != 0 {
            // self.bind();
            crate::framebuffer_texture2d(
                gl::FRAMEBUFFER,
                gl::DEPTH_ATTACHMENT,
                gl::TEXTURE_2D,
                texture,
                0,
            );
            // self.unbind();
        }
    }

    /// Set stencil attachment with `texture`.
    pub fn set_stencil_texture(&self, texture: GLuint) {
        if self.id != 0 {
            self.bind();
            crate::framebuffer_texture2d(
                gl::FRAMEBUFFER,
                gl::STENCIL_ATTACHMENT,
                gl::TEXTURE_2D,
                texture,
                0,
            );
            self.unbind();
        }
    }
}

impl Default for Framebuffer {
    fn default() -> Self {
        Self::with_screen()
    }
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        if self.id != 0 {
            self.unbind();
            crate::delete_framebuffers(&[self.id]);
        }
    }
}

impl Bindable for Framebuffer {
    fn bind(&self) {
        crate::bind_framebuffer(gl::FRAMEBUFFER, self.id);
    }

    fn bind_at(&self, target: u32) {
        crate::bind_framebuffer(target as GLenum, self.id);
    }

    fn unbind(&self) {
        crate::bind_framebuffer(gl::FRAMEBUFFER, 0);
    }

    fn unbind_at(&self, target: u32) {
        crate::bind_framebuffer(target as GLenum, 0);
    }
}

#[derive(Debug)]
pub enum FramebufferAttachment {
    NoAttachment,
    CombinedDepthStencil,
    Depth,
}

/// Framebuffer object.
#[derive(Debug)]
pub struct FramebufferObject<'a> {
    width: usize,
    height: usize,
    texture_format: TextureFormat,
    attachment: FramebufferAttachment,
    fb: Framebuffer,
    texture: Texture<'a>,
}

impl<'a> FramebufferObject<'a> {
    pub fn new(width: usize, height: usize, texture_format: TextureFormat) -> Self {
        let fb = Framebuffer::new();
        let texture = TextureLoader::default()
            .with_size(width, height)
            .with_internal_format(texture_format)
            .with_format(texture_format)
            .with_linear()
            .with_allocate_storage()
            .load()
            .unwrap();
        fb.set_color_texture(texture.id());
        // crate::framebuffer_texture2d(
        //     gl::FRAMEBUFFER,
        //     gl::COLOR_ATTACHMENT0,
        //     gl::TEXTURE_2D,
        //     texture.id(),
        //     0,
        // );
        Self {
            width,
            height,
            texture_format,
            attachment: FramebufferAttachment::NoAttachment,
            fb,
            texture,
        }
    }

    pub fn with_texture_rgba(width: usize, height: usize) -> Self {
        Self::new(width, height, TextureFormat::Rgba)
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn framebuffer(&self) -> &Framebuffer {
        &self.fb
    }

    pub fn texture(&self) -> &Texture<'a> {
        &self.texture
    }
}

impl<'a> Bindable for FramebufferObject<'a> {
    fn bind(&self) {
        self.fb.bind();
        self.texture.bind();
    }

    fn bind_at(&self, slot: u32) {
        self.fb.bind();
        self.texture.bind_at(slot);
    }

    fn unbind(&self) {
        self.texture.unbind();
        self.fb.unbind();
    }

    fn unbind_at(&self, slot: u32) {
        self.texture.unbind_at(slot);
        self.fb.unbind();
    }
}
