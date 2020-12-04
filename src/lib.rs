use nalgebra as na;
#[allow(unused_imports)]
use serde::{Serialize, Deserialize};

pub mod prelude;

/// Foreign Function Interface of the OpenGL unsafe bindings.
pub mod gl;
#[cfg(any(feature = "gles1", feature = "gles2", feature = "gles3"))]
pub use gl::GLeglImageOES;
pub use gl::{
    load_with, GLbitfield, GLboolean, GLchar, GLenum, GLfloat, GLint, GLintptr, GLsizei,
    GLsizeiptr, GLubyte, GLuint, GLvoid,
};

pub mod apis;
pub use apis::*;

/// Error Object for OpenGL.
pub mod error;
pub use error::Error;

pub mod auto_binder;
pub use auto_binder::*;

pub mod buffer;
pub use buffer::*;

pub mod clear_buffers;
pub use clear_buffers::*;

pub mod framebuffer;
pub use framebuffer::*;

pub mod shader;
pub use shader::*;

/// The shared library helper.
pub mod so;

pub mod texture;
pub use texture::*;

pub mod variant;
pub use variant::*;

pub mod viewport;
pub use viewport::*;

/// Callback before object prepare to drop.
pub type Finalizer<'a, T> = Box<dyn Fn(&T) + 'a>;

pub type Isometry3 = na::Isometry3<GLfloat>;
pub type Orthographic3 = na::Orthographic3<GLfloat>;
pub type Perspective3 = na::Perspective3<GLfloat>;
pub type Matrix4 = na::Matrix4<GLfloat>;
pub type Transform3 = na::Transform3<GLfloat>;
pub type Vector2 = na::Vector2<GLfloat>;
pub type Vector3 = na::Vector3<GLfloat>;
pub type Vector4 = na::Vector4<GLfloat>;
