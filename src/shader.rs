use crate::{Bindable, Error, GLenum, GLfloat, GLint, GLuint, Matrix4, Vector2, Vector3, Vector4};
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Copy, Clone, Debug)]
pub enum UniformValue<'a> {
    Unknown,
    Int(GLint),
    UnsignedInt(GLuint),
    Float(GLfloat),
    FLoat2(&'a Vector2),
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
    (vec3($v:expr)) => {
        $crate::UniformValue::Float3($v)
    };
    (vec4($v:expr)) => {
        $crate::UniformValue::Float4($v)
    };
}

#[derive(Clone, Default, Debug)]
pub struct Program {
    id: GLuint,
}

impl Program {
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

    pub fn from_shaders(shaders: &[Shader]) -> Result<Program, String> {
        let program_id = crate::create_program().unwrap();
        for shader in shaders {
            crate::attach_shader(program_id, shader.id());
        }
        crate::link_program(program_id);
        match crate::get_programiv(program_id, crate::LINK_STATUS) {
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

    pub fn id(&self) -> GLuint {
        self.id
    }

    pub fn locate_attrib<T>(&self, name: T) -> Result<GLint, Error>
    where
        T: AsRef<str>,
    {
        crate::get_attrib_location(self.id, name)
    }

    pub fn locate_uniform<T>(&self, name: T) -> Result<GLint, Error>
    where
        T: AsRef<str>,
    {
        crate::get_uniform_location(self.id, name)
    }

    pub fn set_uniform(&self, location: GLint, value: UniformValue) {
        match value {
            UniformValue::Int(v) => crate::uniform1i(location, v),
            UniformValue::Float(v) => crate::uniform1f(location, v),
            UniformValue::Float3(v) => crate::uniform3f(location, v.x, v.y, v.z),
            UniformValue::Matrix4(v) => {
                crate::uniform_matrix4fv(location, crate::FALSE, v.as_slice())
            }
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
            return Ok(crate::VERTEX_SHADER);
        }
        if path.ends_with(".fs")
            || path.ends_with(".frag")
            || path.ends_with("fs.glsl")
            || path.ends_with("frag.glsl")
        {
            return Ok(crate::FRAGMENT_SHADER);
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
        match crate::get_shaderiv(id, crate::COMPILE_STATUS) {
            0 => Err(crate::get_shader_info_log(id).unwrap()),
            _ => Ok(Shader { id }),
        }
    }

    pub fn from_vert_source<T>(source: T) -> Result<Shader, String>
    where
        T: AsRef<str>,
    {
        Shader::from_source(source, crate::VERTEX_SHADER)
    }

    pub fn from_frag_source<T>(source: T) -> Result<Shader, String>
    where
        T: AsRef<str>,
    {
        Shader::from_source(source, crate::FRAGMENT_SHADER)
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
