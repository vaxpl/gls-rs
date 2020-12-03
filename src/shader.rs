use crate::{
    gl, prelude::*, Error, GLenum, GLfloat, GLint, GLuint, Matrix4, Vector2, Vector3, Vector4, Variant,
};
use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Provides storage for a name and value pair.
///
/// This maps to a shader uniform.
#[derive(Debug, Clone)]
pub struct Parameter {
    name: &'static str,
    value: Variant,
}

impl Parameter {
    /// Construct a parameter used for shader program.
    pub fn new<T>(name: &'static str, value: T) -> Self where T: Into<Variant> {
        Self {
            name,
            value: value.into(),
        }
    }

    /// Applies the parameter to specified shader program.
    pub fn apply_to(&self, prog: &Program) {
        if let Ok(loc) = prog.locate_uniform(self.name) {
            match self.value {
                Variant::Int(v) => prog.set_uniform(loc, UniformValue::Int(v)),
                Variant::UInt(v) => prog.set_uniform(loc, UniformValue::UnsignedInt(v)),
                Variant::Float(v) => prog.set_uniform(loc, UniformValue::Float(v)),
                Variant::Float2(ref v) => prog.set_uniform(loc, UniformValue::Float2(v)),
                Variant::Float3(ref v) => prog.set_uniform(loc, UniformValue::Float3(v)),
                Variant::Float4(ref v) => prog.set_uniform(loc, UniformValue::Float4(v)),
                Variant::FloatV(ref v) => prog.set_uniform(loc, UniformValue::FloatV(v)),
                Variant::Matrix4(ref v) => prog.set_uniform(loc, UniformValue::Matrix4(v)),
                _ => {},
            }
        }
    }

    /// Returns the name of the parameter.
    pub fn name(&self) -> &'static str {
        self.name
    }

    /// Returns the value of the parameter.
    pub fn value<'r, T>(&'r self) -> T where T: From<&'r Variant> {
        T::from(&self.value)
    }

    /// Specifies the name of the parameter.
    pub fn set_name(&mut self, name: &'static str) {
        self.name = name;
    }

    /// Specifies the value of the parameter.
    pub fn set_value<T>(&mut self, value: T) where T: Into<Variant> {
        self.value = value.into();
    }
}

#[derive(Copy, Clone, Debug)]
pub enum UniformValue<'a> {
    Unknown,
    Int(GLint),
    UnsignedInt(GLuint),
    Float(GLfloat),
    Float2(&'a Vector2),
    Float3(&'a Vector3),
    Float4(&'a Vector4),
    FloatV(&'a [GLfloat]),
    Matrix4(&'a Matrix4),
}

#[macro_export]
macro_rules! uniform {
    (float($v:expr)) => {
        $crate::UniformValue::Float($v)
    };
    (fv($v:expr)) => {
        $crate::UniformValue::FloatV($v)
    };
    (int($v:expr)) => {
        $crate::UniformValue::Int($v)
    };
    (mat4($v:expr)) => {
        $crate::UniformValue::Matrix4($v)
    };
    (vec2($v:expr)) => {
        $crate::UniformValue::Float2($v)
    };
    (vec2($v0:expr,$v1:expr)) => {
        $crate::UniformValue::Float2(&Vector2::new($v0, $v1))
    };
    (vec3($v:expr)) => {
        $crate::UniformValue::Float3($v)
    };
    (vec3($v0:expr,$v1:expr,$v2:expr)) => {
        $crate::UniformValue::Float3(&Vector3::new($v0, $v1, $v2))
    };
    (vec4($v:expr)) => {
        $crate::UniformValue::Float4($v)
    };
    (vec4($v0:expr,$v1:expr,$v2:expr,$v3:expr)) => {
        $crate::UniformValue::Float4(&Vector4::new($v0, $v1, $v2, $v3))
    };
}

#[derive(Clone, Default, Debug)]
pub struct Program {
    id: GLuint,
}

impl Program {
    /// Build a program from a list of the shader files.
    pub fn from_files<T>(files: &[T]) -> Result<Program, String>
    where
        T: AsRef<str> + std::fmt::Debug,
    {
        let shaders: Vec<_> = files
            .iter()
            .map(|f| Shader::from_file(f.as_ref()).unwrap())
            .collect();
        Program::from_shaders(&shaders[0..])
    }

    /// Build a program from a list of the pre-compiled shaders.
    pub fn from_shaders(shaders: &[Shader]) -> Result<Program, String> {
        let program_id = crate::create_program().unwrap();
        for shader in shaders {
            crate::attach_shader(program_id, shader.id());
        }
        crate::link_program(program_id);
        match crate::get_programiv(program_id, gl::LINK_STATUS) {
            0 => Err(crate::get_program_info_log(program_id).unwrap()),
            _ => {
                for shader in shaders {
                    crate::detach_shader(program_id, shader.id());
                }
                crate::use_program(program_id);
                Ok(Program { id: program_id })
            }
        }
    }

    /// Build a program from a list of the sources.
    pub fn from_sources<T>(sources: &[(T, GLenum)]) -> Result<Program, String>
    where
        T: AsRef<str> + std::fmt::Debug,
    {
        let shaders: Vec<_> = sources
            .iter()
            .map(|(s, k)| Shader::from_source(s, *k).unwrap())
            .collect();
        Program::from_shaders(&shaders[0..])
    }

    /// Returns the Id of the program.
    pub fn id(&self) -> GLuint {
        self.id
    }

    /// Bind the attribute with `name` to specified `location`.
    pub fn bind_attrib<T>(&self, name: T, location: GLuint)
    where
        T: AsRef<str>,
    {
        crate::bind_attrib_location(self.id, location, name)
    }

    /// Returns the location of the attribute with `name`.
    pub fn locate_attrib<T>(&self, name: T) -> Result<GLint, Error>
    where
        T: AsRef<str>,
    {
        crate::get_attrib_location(self.id, name)
    }

    /// Returns the location of the uniform with `name`.
    pub fn locate_uniform<T>(&self, name: T) -> Result<GLint, Error>
    where
        T: AsRef<str>,
    {
        crate::get_uniform_location(self.id, name)
    }

    /// Update the `value` of the uniform with specified `location`.
    pub fn set_uniform(&self, location: GLint, value: UniformValue) {
        match value {
            UniformValue::Int(v) => crate::uniform1i(location, v),
            UniformValue::Float(v) => crate::uniform1f(location, v),
            UniformValue::Float2(v) => crate::uniform2f(location, v.x, v.y),
            UniformValue::Float3(v) => crate::uniform3f(location, v.x, v.y, v.z),
            UniformValue::Float4(v) => crate::uniform4f(location, v.x, v.y, v.z, v.w),
            UniformValue::Matrix4(v) => crate::uniform_matrix4fv(location, gl::FALSE, v.as_slice()),
            _ => unimplemented!(),
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        crate::delete_program(self.id);
    }
}

impl Bindable for Program {
    fn bind(&self) {
        crate::use_program(self.id);
    }

    fn unbind(&self) {
        crate::use_program(0);
    }
}

#[derive(Clone, Default, Debug)]
pub struct Shader {
    id: GLuint,
}

impl Shader {
    pub fn detect_kind<T>(path: T) -> Result<GLenum, String>
    where
        T: AsRef<Path>,
    {
        let path = path.as_ref().to_str().unwrap();
        if path.ends_with(".vs")
            || path.ends_with(".vert")
            || path.ends_with("vs.glsl")
            || path.ends_with("vert.glsl")
        {
            return Ok(gl::VERTEX_SHADER);
        }
        if path.ends_with(".fs")
            || path.ends_with(".frag")
            || path.ends_with("fs.glsl")
            || path.ends_with("frag.glsl")
        {
            return Ok(gl::FRAGMENT_SHADER);
        }
        Err(format!("Unknown Shader Type: {}!", path))
    }

    pub fn from_bytes(bytes: &[u8], kind: GLenum) -> Result<Shader, String> {
        let source = String::from_utf8_lossy(bytes);
        Shader::from_source(source, kind)
    }

    pub fn from_file<T>(path: T) -> Result<Shader, String>
    where
        T: AsRef<Path>,
    {
        let kind = Shader::detect_kind(path.as_ref()).unwrap();
        let mut file = File::open(path.as_ref()).unwrap();
        let mut source = String::new();
        let _size = file.read_to_string(&mut source).unwrap();
        Shader::from_source(source, kind)
    }

    pub fn from_source<T>(source: T, kind: GLenum) -> Result<Shader, String>
    where
        T: AsRef<str>,
    {
        let id = crate::create_shader(kind).unwrap();
        crate::shader_source(id, source.as_ref());
        crate::compile_shader(id);
        match crate::get_shaderiv(id, gl::COMPILE_STATUS) {
            0 => Err(crate::get_shader_info_log(id).unwrap()),
            _ => Ok(Shader { id }),
        }
    }

    pub fn from_vert_source<T>(source: T) -> Result<Shader, String>
    where
        T: AsRef<str>,
    {
        Shader::from_source(source, gl::VERTEX_SHADER)
    }

    pub fn from_frag_source<T>(source: T) -> Result<Shader, String>
    where
        T: AsRef<str>,
    {
        Shader::from_source(source, gl::FRAGMENT_SHADER)
    }

    pub fn id(&self) -> GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        crate::delete_shader(self.id);
    }
}
