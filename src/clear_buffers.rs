use crate::{gl, prelude::*, GLbitfield, GLfloat, GLint, Vector4};
use std::option::Option;

#[derive(Clone, Copy, Default, Debug)]
pub struct ClearBuffers {
    accum: Option<Vector4>,
    color: Option<Vector4>,
    color_index: Option<GLfloat>,
    depth: Option<GLfloat>,
    stencil: Option<GLint>,
}

impl ClearBuffers {
    pub fn new() -> Self {
        Self {
            accum: None,
            color: None,
            color_index: None,
            depth: None,
            stencil: None,
        }
    }

    #[cfg(feature = "gl4")]
    pub fn with_accum(mut self, accum: Option<Vector4>) -> Self {
        self.accum = accum;
        self
    }

    pub fn with_color(mut self, color: Option<Vector4>) -> Self {
        self.color = color;
        self
    }

    #[cfg(feature = "gl4")]
    pub fn with_color_index(mut self, color_index: Option<GLfloat>) -> Self {
        self.color_index = color_index;
        self
    }

    pub fn with_depth(mut self, depth: Option<GLfloat>) -> Self {
        self.depth = depth;
        self
    }

    pub fn with_stencil(mut self, stencil: Option<GLint>) -> Self {
        self.stencil = stencil;
        self
    }

    #[cfg(feature = "gl4")]
    pub fn set_accum(&mut self, accum: Vector4) {
        self.accum = Some(accum);
    }

    pub fn set_color(&mut self, color: Vector4) {
        self.color = Some(color);
    }

    #[cfg(feature = "gl4")]
    pub fn set_color_index(&mut self, color_index: GLfloat) {
        self.color_index = Some(color_index);
    }

    pub fn set_depth(&mut self, depth: GLfloat) {
        self.depth = Some(depth);
    }

    pub fn set_stencil(&mut self, stencil: GLint) {
        self.stencil = Some(stencil);
    }
}

impl Bindable for ClearBuffers {
    fn bind(&self) {
        let mut mask: GLbitfield = 0;
        #[cfg(feature = "gl4")]
        {
            if let Some(ref v) = self.accum {
                crate::clear_accum(v);
                mask |= gl::ACCUM_BUFFER_BIT;
            }
        }
        if let Some(v) = self.color {
            crate::clear_color(v.x, v.y, v.z, v.w);
            mask |= gl::COLOR_BUFFER_BIT;
        }
        #[cfg(feature = "gl4")]
        {
            if let Some(v) = self.index {
                crate::clear_index(v);
                mask |= gl::COLOR_BUFFER_BIT;
            }
        }
        if let Some(v) = self.depth {
            crate::clear_depthf(v);
            mask |= gl::DEPTH_BUFFER_BIT;
        }
        if let Some(v) = self.stencil {
            crate::clear_stencil(v);
            mask |= gl::STENCIL_BUFFER_BIT;
        }
        if mask != 0 {
            crate::clear(mask);
        }
    }
    fn unbind(&self) {}
}
