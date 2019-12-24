use nalgebra as na;

pub mod prelude;

/// Foreign Function Interface of the OpenGL unsafe bindings.
pub mod raw;
pub use raw::{
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

pub mod shader;
pub use shader::*;

/// The shared library helper.
pub mod so;

pub mod texture;
pub use texture::*;

pub mod viewport;
pub use viewport::*;

pub type Matrix4 = na::Matrix4<raw::GLfloat>;
pub type Vector2 = na::Vector2<raw::GLfloat>;
pub type Vector3 = na::Vector3<raw::GLfloat>;
pub type Vector4 = na::Vector4<raw::GLfloat>;
