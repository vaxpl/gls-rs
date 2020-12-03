use crate::{
    gl, prelude::*, Error, GLboolean, GLenum, GLfloat, GLint, GLuint, Matrix4, Vector2, Vector3,
    Vector4,
};

/// Variant container.
#[derive(Debug, Clone)]
pub enum Variant {
    Unknown,
    Bool(GLboolean),
    Int(GLint),
    UInt(GLuint),
    Float(GLfloat),
    Float2(Vector2),
    Float3(Vector3),
    Float4(Vector4),
    FloatV(Vec<GLfloat>),
    Matrix4(Matrix4),
}

impl From<f32> for Variant {
    fn from(val: f32) -> Self {
        Self::Float(val)
    }
}
impl From<(f32, f32)> for Variant {
    fn from(val: (f32, f32)) -> Self {
        Self::Float2(Vector2::new(val.0, val.1))
    }
}
