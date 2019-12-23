use crate::{
    raw, Error, GLbitfield, GLboolean, GLchar, GLenum, GLfloat, GLint, GLintptr, GLsizei,
    GLsizeiptr, GLuint,
};
use std::convert::TryInto;
use std::ffi::{CStr, CString};
use std::os::raw::c_void;
use std::string::String;
use std::vec::Vec;

/// Set the active program object for a program pipeline object.
///
/// # Parameters
///
/// * `pipeline` - Specifies the program pipeline object to set the active program object for.
/// * `program` - Specifies the program object to set as the active program pipeline object pipeline.
pub fn active_shader_program(pipeline: GLuint, program: GLuint) {
    unsafe { raw::ActiveShaderProgram(pipeline, program) }
}

/// Select active texture unit.
///
/// # Parameters
///
/// * `texture` - Specifies which texture unit to make active. The number of texture units is implementation dependent, but must be at least 80.
///   texture must be one of GL_TEXTUREi, where i ranges from zero to the value of GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS minus one.
///   The initial value is GL_TEXTURE0.
pub fn active_texture(texture: GLenum) {
    unsafe { raw::ActiveTexture(texture) }
}

/// Attaches a shader object to a program object.
///
/// # Parameters
///
/// * `program` - Specifies the program object to which a shader object will be attached.
/// * `shader` - Specifies the shader object that is to be attached.
pub fn attach_shader(program: GLuint, shader: GLuint) {
    unsafe { raw::AttachShader(program, shader) }
}

/// Associates a generic vertex attribute index with a named attribute variable.
///
/// # Parameters
///
/// * `program` - Specifies the handle of the program object in which the association is to be made.
/// * `index` - Specifies the index of the generic vertex attribute to be bound.
/// * `name` - Specifies a null terminated string containing the name of the vertex shader attribute variable to which index is to be bound.
pub fn bind_attrib_location<S: AsRef<str>>(program: GLuint, index: GLuint, name: S) {
    unsafe {
        raw::BindAttribLocation(
            program,
            index,
            CString::new(name.as_ref()).unwrap().as_ptr(),
        )
    }
}

/// Bind a named buffer object.
///
/// # Parameters
///
/// * `target` - Specifies the target to which the buffer object is bound, which must be one of the buffer binding targets.
/// * `buffer` - Specifies the name of a buffer object.
pub fn bind_buffer(target: GLenum, buffer: GLuint) {
    unsafe { raw::BindBuffer(target, buffer) }
}

pub fn bind_frame_buffer(target: GLenum, framebuffer: GLuint) {
    unsafe { raw::BindFramebuffer(target, framebuffer) }
}

pub fn bind_image_texture(
    unit: GLuint,
    texture: GLuint,
    level: GLint,
    layered: GLboolean,
    layer: GLint,
    access: GLenum,
    format: GLenum,
) {
    unsafe { raw::BindImageTexture(unit, texture, level, layered, layer, access, format) }
}

pub fn bind_render_buffer(target: GLenum, renderbuffer: GLuint) {
    unsafe { raw::BindRenderbuffer(target, renderbuffer) }
}

pub fn bind_texture(target: GLenum, texture: GLuint) {
    unsafe { raw::BindTexture(target, texture) }
}

pub fn bind_vertex_array(array: GLuint) {
    unsafe { raw::BindVertexArray(array) }
}

pub fn bind_vertex_buffer(bindingindex: GLuint, buffer: GLuint, offset: GLintptr, stride: GLsizei) {
    unsafe { raw::BindVertexBuffer(bindingindex, buffer, offset, stride) }
}

pub fn blend_color(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) {
    unsafe { raw::BlendColor(red, green, blue, alpha) }
}

pub fn blend_equation(mode: GLenum) {
    unsafe { raw::BlendEquation(mode) }
}

pub fn blend_equation_separate(mode_rgb: GLenum, mode_alpha: GLenum) {
    unsafe { raw::BlendEquationSeparate(mode_rgb, mode_alpha) }
}

pub fn blend_equation_separatei(buf: GLuint, mode_rgb: GLenum, mode_alpha: GLenum) {
    unsafe { raw::BlendEquationSeparatei(buf, mode_rgb, mode_alpha) }
}

pub fn blend_equationi(buf: GLuint, mode: GLenum) {
    unsafe { raw::BlendEquationi(buf, mode) }
}

pub fn blend_func(sfactor: GLenum, dfactor: GLenum) {
    unsafe { raw::BlendFunc(sfactor, dfactor) }
}

/// Specify pixel arithmetic for RGB and alpha components separately.
pub fn blend_func_separate(
    sfactor_rgb: GLenum,
    dfactor_rgb: GLenum,
    sfactor_alpha: GLenum,
    dfactor_alpha: GLenum,
) {
    unsafe { raw::BlendFuncSeparate(sfactor_rgb, dfactor_rgb, sfactor_alpha, dfactor_alpha) }
}

pub fn blend_func_separatei(
    buf: GLuint,
    sfactor_rgb: GLenum,
    dfactor_rgb: GLenum,
    sfactor_alpha: GLenum,
    dfactor_alpha: GLenum,
) {
    unsafe { raw::BlendFuncSeparatei(buf, sfactor_rgb, dfactor_rgb, sfactor_alpha, dfactor_alpha) }
}

pub fn blend_funci(buf: GLuint, src: GLenum, dst: GLenum) {
    unsafe { raw::BlendFunci(buf, src, dst) }
}

/// Creates and initializes a buffer object's data store.
///
/// * `target` - Specifies the target to which the buffer object is bound for glBufferData, which must be one of the buffer binding targets.
/// * `buffer` - Specifies the name of the buffer object for glNamedBufferData function.
/// * `size` - Specifies the size in bytes of the buffer object's new data store.
/// * `data` - Specifies a pointer to data that will be copied into the data store for initialization, or NULL if no data is to be copied.
/// * `usage` - Specifies the expected usage pattern of the data store. The symbolic constant must be GL_STREAM_DRAW, GL_STREAM_READ, GL_STREAM_COPY, GL_STATIC_DRAW, GL_STATIC_READ, GL_STATIC_COPY, GL_DYNAMIC_DRAW, GL_DYNAMIC_READ, or GL_DYNAMIC_COPY.
pub fn buffer_data<T>(target: GLenum, size: GLsizeiptr, data: Option<&[T]>, usage: GLenum)
where
    T: Sized,
{
    match data {
        Some(v) => unsafe {
            raw::BufferData(
                target,
                (v.len() * std::mem::size_of::<T>()) as GLsizeiptr,
                v.as_ptr() as *const c_void,
                usage,
            );
        },
        None => unsafe {
            raw::BufferData(target, size, std::ptr::null(), usage);
        },
    }
}

pub fn buffer_sub_data<T>(target: GLenum, offset: GLintptr, data: &[T])
where
    T: Sized,
{
    unsafe {
        raw::BufferSubData(
            target,
            offset,
            (data.len() * std::mem::size_of::<T>()) as GLsizeiptr,
            data.as_ptr() as *const c_void,
        )
    }
}

pub fn check_framebuffer_status(target: GLenum) -> GLenum {
    unsafe { raw::CheckFramebufferStatus(target) }
}

pub fn clear(mask: GLbitfield) {
    unsafe { raw::Clear(mask) }
}

pub fn clear_color(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) {
    unsafe { raw::ClearColor(red, green, blue, alpha) }
}

pub fn clear_depthf(d: GLfloat) {
    unsafe { raw::ClearDepthf(d) }
}

pub fn clear_stencil(s: GLint) {
    unsafe { raw::ClearStencil(s) }
}

pub fn compile_shader(shader: GLuint) {
    unsafe { raw::CompileShader(shader) }
}

pub fn create_program() -> Result<GLuint, Error> {
    match unsafe { raw::CreateProgram() } {
        raw::NONE => Err(Error::new()),
        val => Ok(val),
    }
}

pub fn create_shader(type_: GLenum) -> Result<GLuint, Error> {
    match unsafe { raw::CreateShader(type_) } {
        raw::NONE => Err(Error::new()),
        val => Ok(val),
    }
}

pub fn create_shader_programv<T: AsRef<str>>(
    type_: GLenum,
    strings: &Vec<T>,
) -> Result<GLuint, Error> {
    let cv: Vec<CString> = strings
        .iter()
        .map(|s| CString::new(s.as_ref()).unwrap())
        .collect();
    let pv: Vec<*const GLchar> = cv.iter().map(|s| s.as_ptr()).collect();
    match unsafe { raw::CreateShaderProgramv(type_, pv.len().try_into().unwrap(), pv.as_ptr()) } {
        raw::NONE => Err(Error::new()),
        val => Ok(val),
    }
}

pub fn cull_face(mode: GLenum) {
    unsafe { raw::CullFace(mode) }
}

pub fn delete_buffers(buffers: &[GLuint]) {
    unsafe { raw::DeleteBuffers(buffers.len() as GLsizei, buffers.as_ptr()) }
}

pub fn delete_framebuffers(framebuffers: &[GLuint]) {
    unsafe { raw::DeleteFramebuffers(framebuffers.len() as GLsizei, framebuffers.as_ptr()) }
}

pub fn delete_program(program: GLuint) {
    unsafe { raw::DeleteProgram(program) }
}

pub fn delete_shader(shader: GLuint) {
    unsafe { raw::DeleteShader(shader) }
}

pub fn delete_textures(textures: &[GLuint]) {
    unsafe { raw::DeleteTextures(textures.len() as GLsizei, textures.as_ptr()) }
}

pub fn delete_vertex_arrays(arrays: &[GLuint]) {
    unsafe { raw::DeleteVertexArrays(arrays.len() as GLsizei, arrays.as_ptr()) }
}

pub fn depth_func(func: GLenum) {
    unsafe { raw::DepthFunc(func) }
}

pub fn depth_mask(flag: GLboolean) {
    unsafe { raw::DepthMask(flag) }
}

pub fn detach_shader(program: GLuint, shader: GLuint) {
    unsafe { raw::DetachShader(program, shader) }
}

pub fn disable(cap: GLenum) {
    unsafe { raw::Disable(cap) }
}

pub fn disable_vertex_attrib_array(index: GLuint) {
    unsafe { raw::DisableVertexAttribArray(index) }
}

pub fn draw_arrays(mode: GLenum, first: GLint, count: GLsizei) {
    unsafe { raw::DrawArrays(mode, first, count) }
}

pub fn draw_buffers(bufs: &[GLenum]) {
    unsafe { raw::DrawBuffers(bufs.len() as GLsizei, bufs.as_ptr()) }
}

pub fn draw_elements(mode: GLenum, count: GLsizei, type_: GLenum, indices: GLsizeiptr) {
    unsafe { raw::DrawElements(mode, count, type_, indices as *const c_void) }
}

pub fn enable(cap: GLenum) {
    unsafe { raw::Enable(cap) }
}

pub fn enable_vertex_attrib_array(index: GLuint) {
    unsafe { raw::EnableVertexAttribArray(index) }
}

pub fn finish() {
    unsafe { raw::Finish() }
}

pub fn flush() {
    unsafe { raw::Flush() }
}

pub fn framebuffer_texture(target: GLenum, attachment: GLenum, texture: GLuint, level: GLint) {
    unsafe { raw::FramebufferTexture(target, attachment, texture, level) }
}

pub fn framebuffer_texture2d(
    target: GLenum,
    attachment: GLenum,
    textarget: GLenum,
    texture: GLuint,
    level: GLint,
) {
    unsafe { raw::FramebufferTexture2D(target, attachment, textarget, texture, level) }
}

pub fn front_face(mode: GLenum) {
    unsafe { raw::FrontFace(mode) }
}

pub fn new_buffer() -> GLuint {
    let mut buffers: [GLuint; 1] = [0];
    gen_buffers(&mut buffers);
    buffers[0]
}

pub fn gen_buffers(buffers: &mut [GLuint]) {
    unsafe { raw::GenBuffers(buffers.len() as GLsizei, buffers.as_mut_ptr()) }
}

pub fn gen_framebuffers(framebuffers: &mut [GLuint]) {
    unsafe { raw::GenFramebuffers(framebuffers.len() as GLsizei, framebuffers.as_mut_ptr()) }
}

pub fn new_texture() -> GLuint {
    let mut textures: [GLuint; 1] = [0];
    gen_textures(&mut textures);
    textures[0]
}

pub fn gen_textures(textures: &mut [GLuint]) {
    unsafe { raw::GenTextures(textures.len() as GLsizei, textures.as_mut_ptr()) }
}

pub fn new_vertex_array() -> GLuint {
    let mut arrays: [GLuint; 1] = [0];
    gen_vertex_arrays(&mut arrays);
    arrays[0]
}

pub fn gen_vertex_arrays(arrays: &mut [GLuint]) {
    unsafe { raw::GenVertexArrays(arrays.len() as GLsizei, arrays.as_mut_ptr()) }
}

pub fn generate_mipmap(target: GLenum) {
    unsafe { raw::GenerateMipmap(target) }
}

pub fn get_active_attrib(program: GLuint, index: GLuint) -> Result<(String, GLenum, GLint), Error> {
    let mut name: [u8; 256] = [0; 256];
    let buf_size: GLsizei = 255;
    let mut length: GLsizei = 0;
    let mut size: GLint = 0;
    let mut type_: GLenum = 0;
    unsafe {
        raw::GetActiveAttrib(
            program,
            index,
            buf_size,
            &mut length,
            &mut size,
            &mut type_,
            name.as_mut_ptr() as *mut GLchar,
        )
    }
    match length {
        0 => Err(Error::new()),
        _ => Ok((
            CStr::from_bytes_with_nul(&name)
                .unwrap()
                .to_string_lossy()
                .into_owned(),
            type_,
            size,
        )),
    }
}

pub fn get_active_uniform(
    program: GLuint,
    index: GLuint,
) -> Result<(String, GLenum, GLint), Error> {
    let mut name: [u8; 256] = [0; 256];
    let buf_size: GLsizei = 255;
    let mut length: GLsizei = 0;
    let mut size: GLint = 0;
    let mut type_: GLenum = 0;
    unsafe {
        raw::GetActiveUniform(
            program,
            index,
            buf_size,
            &mut length,
            &mut size,
            &mut type_,
            name.as_mut_ptr() as *mut GLchar,
        )
    }
    match length {
        0 => Err(Error::new()),
        _ => Ok((
            CStr::from_bytes_with_nul(&name)
                .unwrap()
                .to_string_lossy()
                .into_owned(),
            type_,
            size,
        )),
    }
}

pub fn get_attached_shaders(program: GLuint, max_count: GLsizei) -> Result<Vec<GLuint>, Error> {
    let mut count: GLsizei = 0;
    let mut shaders: Vec<GLuint> = vec![0, max_count as u32];
    unsafe { raw::GetAttachedShaders(program, max_count, &mut count, shaders.as_mut_ptr()) }
    match count {
        0 => Err(Error::new()),
        v => {
            shaders.truncate(v as usize);
            Ok(shaders)
        }
    }
}

pub fn get_attrib_location<T: AsRef<str>>(program: GLuint, name: T) -> Result<GLint, Error> {
    let name = CString::new(name.as_ref()).unwrap();
    let val = unsafe { raw::GetAttribLocation(program, name.as_ptr()) };
    match val {
        -1 => Err(Error::new()),
        x => Ok(x),
    }
}

pub fn get_error() -> GLenum {
    unsafe { raw::GetError() }
}

pub fn get_floatv(pname: GLenum, data: &mut [GLfloat]) {
    unsafe { raw::GetFloatv(pname, data.as_mut_ptr()) }
}

pub fn get_integerv(pname: GLenum, data: &mut [GLint]) {
    unsafe { raw::GetIntegerv(pname, data.as_mut_ptr()) }
}

pub fn get_pointerv(pname: GLenum) -> *mut c_void {
    let ptr: *mut c_void = std::ptr::null_mut();
    unsafe {
        raw::GetPointerv(pname, &ptr);
    }
    ptr
}

pub fn get_program_binary(program: GLuint) -> Result<(GLenum, Vec<u8>), Error> {
    let buf_size = get_programiv(program, raw::PROGRAM_BINARY_LENGTH);
    let mut data: Vec<u8> = vec![0; (buf_size + 2) as usize];
    let mut length: GLsizei = 0;
    let mut binary_format: GLenum = 0;
    unsafe {
        raw::GetProgramBinary(
            program,
            data.len() as GLsizei,
            &mut length,
            &mut binary_format,
            data.as_mut_ptr() as *mut c_void,
        );
    }
    match length {
        0 => Err(Error::new()),
        x => {
            data.truncate(x as usize);
            Ok((binary_format, data))
        }
    }
}

pub fn get_program_info_log(program: GLuint) -> Result<String, Error> {
    let buf_size = get_programiv(program, raw::INFO_LOG_LENGTH);
    let mut info: Vec<u8> = vec![0; buf_size as usize];
    let mut length: GLsizei = 0;
    unsafe {
        raw::GetProgramInfoLog(
            program,
            info.len() as GLsizei,
            &mut length,
            info.as_mut_ptr() as *mut GLchar,
        );
    }
    match length {
        0 => Err(Error::new()),
        _ => Ok(CStr::from_bytes_with_nul(&info)
            .unwrap()
            .to_string_lossy()
            .into_owned()),
    }
}

pub fn get_programiv(program: GLuint, pname: GLenum) -> GLint {
    let mut params: GLint = 0;
    unsafe {
        raw::GetProgramiv(program, pname, &mut params);
    }
    params
}

pub fn get_shader_info_log(shader: GLuint) -> Result<String, Error> {
    let buf_size = get_shaderiv(shader, raw::INFO_LOG_LENGTH);
    let mut info: Vec<u8> = vec![0; buf_size as usize];
    let mut length: GLsizei = 0;
    unsafe {
        raw::GetShaderInfoLog(
            shader,
            info.len() as GLsizei,
            &mut length,
            info.as_mut_ptr() as *mut GLchar,
        );
    }
    match length {
        0 => Err(Error::new()),
        _ => Ok(CStr::from_bytes_with_nul(&info)
            .unwrap()
            .to_string_lossy()
            .into_owned()),
    }
}

pub fn get_shader_source(shader: GLuint) -> Result<String, Error> {
    let buf_size = get_shaderiv(shader, raw::SHADER_SOURCE_LENGTH);
    let mut source: Vec<u8> = vec![0; (buf_size + 2) as usize];
    let mut length: GLsizei = 0;
    unsafe {
        raw::GetShaderSource(
            shader,
            source.len() as GLsizei,
            &mut length,
            source.as_mut_ptr() as *mut GLchar,
        );
    }
    match length {
        0 => Err(Error::new()),
        _ => Ok(CStr::from_bytes_with_nul(&source)
            .unwrap()
            .to_string_lossy()
            .into_owned()),
    }
}

/// Return a parameter from a shader object.
pub fn get_shaderiv(shader: GLuint, pname: GLenum) -> GLint {
    let mut params: GLint = 0;
    unsafe {
        raw::GetShaderiv(shader, pname, &mut params);
    }
    params
}

pub fn get_string(name: GLenum) -> Result<String, Error> {
    let s: *const GLchar = unsafe { raw::GetString(name) as *const GLchar };
    match s.is_null() {
        true => Err(Error::new()),
        false => unsafe { Ok(CStr::from_ptr(s).to_string_lossy().into_owned()) },
    }
}

pub fn get_stringi(name: GLenum, index: GLuint) -> Result<String, Error> {
    let s: *const GLchar = unsafe { raw::GetStringi(name, index) as *const GLchar };
    match s.is_null() {
        true => Err(Error::new()),
        false => unsafe { Ok(CStr::from_ptr(s).to_string_lossy().into_owned()) },
    }
}

pub fn get_tex_parameterfv(target: GLenum, pname: GLenum, params: &mut [GLfloat]) {
    unsafe { raw::GetTexParameterfv(target, pname, params.as_mut_ptr()) }
}

pub fn get_tex_parameteriv(target: GLenum, pname: GLenum, params: &mut [GLint]) {
    unsafe { raw::GetTexParameteriv(target, pname, params.as_mut_ptr()) }
}

pub fn get_uniform_location<S: AsRef<str>>(program: GLuint, name: S) -> Result<GLint, Error> {
    let name = CString::new(name.as_ref()).unwrap();
    match unsafe { raw::GetUniformLocation(program, name.as_ptr()) } {
        -1 => Err(Error::new()),
        other => Ok(other),
    }
}

pub fn get_uniformfv(program: GLuint, location: GLint, params: &mut [GLfloat]) {
    unsafe { raw::GetUniformfv(program, location, params.as_mut_ptr()) }
}

pub fn get_uniformiv(program: GLuint, location: GLint, params: &mut [GLint]) {
    unsafe { raw::GetUniformiv(program, location, params.as_mut_ptr()) }
}

pub fn hint(target: GLenum, mode: GLenum) {
    unsafe {
        raw::Hint(target, mode);
    }
}

pub fn line_width(width: GLfloat) {
    unsafe {
        raw::LineWidth(width);
    }
}

pub fn link_program(program: GLuint) {
    unsafe {
        raw::LinkProgram(program);
    }
}

/// Replaces the source code in a shader object.
///
/// # Parameters
///
/// * `shader` - Specifies the handle of the shader object whose source code is to be replaced.
/// * `source` - Specifies source code to be loaded into the shader.
pub fn shader_source<S: AsRef<str>>(shader: GLuint, source: S) {
    let src = CString::new(source.as_ref()).unwrap();
    let string = src.as_ptr();
    unsafe {
        raw::ShaderSource(shader, 1, &string as *const *const GLchar, std::ptr::null());
    }
}

pub fn scissor(x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
    unsafe { raw::Scissor(x, y, width, height) }
}

pub fn tex_image2d<T>(
    target: GLenum,
    level: GLint,
    internalformat: GLint,
    width: GLsizei,
    height: GLsizei,
    border: GLint,
    format: GLenum,
    type_: GLenum,
    pixels: Option<&[T]>,
) where
    T: Sized,
{
    match pixels {
        Some(v) => unsafe {
            raw::TexImage2D(
                target,
                level,
                internalformat,
                width,
                height,
                border,
                format,
                type_,
                v.as_ptr() as *const core::ffi::c_void,
            );
        },
        None => unsafe {
            raw::TexImage2D(
                target,
                level,
                internalformat,
                width,
                height,
                border,
                format,
                type_,
                std::ptr::null(),
            );
        },
    }
}

pub fn tex_image3d<T>(
    target: GLenum,
    level: GLint,
    internalformat: GLint,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
    border: GLint,
    format: GLenum,
    type_: GLenum,
    pixels: Option<&[T]>,
) where
    T: Sized,
{
    match pixels {
        Some(v) => unsafe {
            raw::TexImage3D(
                target,
                level,
                internalformat,
                width,
                height,
                depth,
                border,
                format,
                type_,
                v.as_ptr() as *const core::ffi::c_void,
            );
        },
        None => unsafe {
            raw::TexImage3D(
                target,
                level,
                internalformat,
                width,
                height,
                depth,
                border,
                format,
                type_,
                std::ptr::null(),
            );
        },
    }
}

pub fn tex_parameterf(target: GLenum, pname: GLenum, param: GLfloat) {
    unsafe {
        raw::TexParameterf(target, pname, param);
    }
}

pub fn tex_parameterfv(target: GLenum, pname: GLenum, params: &[GLfloat]) {
    unsafe {
        raw::TexParameterfv(target, pname, params.as_ptr());
    }
}

pub fn tex_parameteri(target: GLenum, pname: GLenum, param: GLint) {
    unsafe {
        raw::TexParameteri(target, pname, param);
    }
}

pub fn tex_parameteriv(target: GLenum, pname: GLenum, params: &[GLint]) {
    unsafe {
        raw::TexParameteriv(target, pname, params.as_ptr());
    }
}

/// Specify a two-dimensional texture subimage.
pub fn tex_sub_image2d<T>(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    format: GLenum,
    type_: GLenum,
    pixels: &[T],
) where
    T: Sized,
{
    unsafe {
        raw::TexSubImage2D(
            target,
            level,
            xoffset,
            yoffset,
            width,
            height,
            format,
            type_,
            pixels.as_ptr() as *const core::ffi::c_void,
        );
    }
}

pub fn tex_sub_image3d<T>(
    target: GLenum,
    level: GLint,
    xoffset: GLint,
    yoffset: GLint,
    zoffset: GLint,
    width: GLsizei,
    height: GLsizei,
    depth: GLsizei,
    format: GLenum,
    type_: GLenum,
    pixels: &[T],
) where
    T: Sized,
{
    unsafe {
        raw::TexSubImage3D(
            target,
            level,
            xoffset,
            yoffset,
            zoffset,
            width,
            height,
            depth,
            format,
            type_,
            pixels.as_ptr() as *const core::ffi::c_void,
        );
    }
}

pub fn uniform1f(location: GLint, v0: GLfloat) {
    unsafe {
        raw::Uniform1f(location, v0);
    }
}

pub fn uniform1fv(location: GLint, va: &[GLfloat]) {
    unsafe {
        raw::Uniform1fv(location, va.len() as GLsizei, va.as_ptr());
    }
}

pub fn uniform1i(location: GLint, v0: GLint) {
    unsafe {
        raw::Uniform1i(location, v0);
    }
}

pub fn uniform1iv(location: GLint, va: &[GLint]) {
    unsafe {
        raw::Uniform1iv(location, va.len() as GLsizei, va.as_ptr());
    }
}

pub fn uniform1ui(location: GLint, v0: GLuint) {
    unsafe {
        raw::Uniform1ui(location, v0);
    }
}

pub fn uniform1uiv(location: GLint, va: &[GLuint]) {
    unsafe {
        raw::Uniform1uiv(location, va.len() as GLsizei, va.as_ptr());
    }
}

pub fn uniform2f(location: GLint, v0: GLfloat, v1: GLfloat) {
    unsafe {
        raw::Uniform2f(location, v0, v1);
    }
}

pub fn uniform2fv(location: GLint, va: &[[GLfloat; 2usize]]) {
    unsafe {
        raw::Uniform2fv(location, va.len() as GLsizei, va.as_ptr() as *const GLfloat);
    }
}

pub fn uniform2i(location: GLint, v0: GLint, v1: GLint) {
    unsafe {
        raw::Uniform2i(location, v0, v1);
    }
}

pub fn uniform2iv(location: GLint, va: &[[GLint; 2usize]]) {
    unsafe {
        raw::Uniform2iv(location, va.len() as GLsizei, va.as_ptr() as *const GLint);
    }
}

pub fn uniform2ui(location: GLint, v0: GLuint, v1: GLuint) {
    unsafe {
        raw::Uniform2ui(location, v0, v1);
    }
}

pub fn uniform2uiv(location: GLint, va: &[[GLuint; 2usize]]) {
    unsafe {
        raw::Uniform2uiv(location, va.len() as GLsizei, va.as_ptr() as *const GLuint);
    }
}

pub fn uniform3f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat) {
    unsafe {
        raw::Uniform3f(location, v0, v1, v2);
    }
}

pub fn uniform3fv(location: GLint, va: &[[GLfloat; 3usize]]) {
    unsafe {
        raw::Uniform3fv(location, va.len() as GLsizei, va.as_ptr() as *const GLfloat);
    }
}

pub fn uniform3i(location: GLint, v0: GLint, v1: GLint, v2: GLint) {
    unsafe {
        raw::Uniform3i(location, v0, v1, v2);
    }
}

pub fn uniform3iv(location: GLint, va: &[[GLint; 3usize]]) {
    unsafe {
        raw::Uniform3iv(location, va.len() as GLsizei, va.as_ptr() as *const GLint);
    }
}

pub fn uniform3ui(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint) {
    unsafe {
        raw::Uniform3ui(location, v0, v1, v2);
    }
}

pub fn uniform3uiv(location: GLint, va: &[[GLuint; 3usize]]) {
    unsafe {
        raw::Uniform3uiv(location, va.len() as GLsizei, va.as_ptr() as *const GLuint);
    }
}

pub fn uniform4f(location: GLint, v0: GLfloat, v1: GLfloat, v2: GLfloat, v3: GLfloat) {
    unsafe {
        raw::Uniform4f(location, v0, v1, v2, v3);
    }
}

pub fn uniform4fv(location: GLint, va: &[[GLfloat; 4usize]]) {
    unsafe {
        raw::Uniform4fv(location, va.len() as GLsizei, va.as_ptr() as *const GLfloat);
    }
}

pub fn uniform4i(location: GLint, v0: GLint, v1: GLint, v2: GLint, v3: GLint) {
    unsafe {
        raw::Uniform4i(location, v0, v1, v2, v3);
    }
}

pub fn uniform4iv(location: GLint, va: &[[GLint; 4usize]]) {
    unsafe {
        raw::Uniform4iv(location, va.len() as GLsizei, va.as_ptr() as *const GLint);
    }
}

pub fn uniform4ui(location: GLint, v0: GLuint, v1: GLuint, v2: GLuint, v3: GLuint) {
    unsafe {
        raw::Uniform4ui(location, v0, v1, v2, v3);
    }
}

pub fn uniform4uiv(location: GLint, va: &[[GLuint; 4usize]]) {
    unsafe {
        raw::Uniform4uiv(location, va.len() as GLsizei, va.as_ptr() as *const GLuint);
    }
}

pub fn uniform_matrix2fv(location: GLint, transpose: GLboolean, va: &[GLfloat]) {
    unsafe {
        raw::UniformMatrix2fv(location, (va.len() / 4) as GLsizei, transpose, va.as_ptr());
    }
}

pub fn uniform_matrix2x3fv(location: GLint, transpose: GLboolean, va: &[GLfloat]) {
    unsafe {
        raw::UniformMatrix2x3fv(location, (va.len() / 6) as GLsizei, transpose, va.as_ptr());
    }
}

pub fn uniform_matrix2x4fv(location: GLint, transpose: GLboolean, va: &[GLfloat]) {
    unsafe {
        raw::UniformMatrix2x4fv(location, (va.len() / 8) as GLsizei, transpose, va.as_ptr());
    }
}

pub fn uniform_matrix3fv(location: GLint, transpose: GLboolean, va: &[GLfloat]) {
    unsafe {
        raw::UniformMatrix3fv(location, (va.len() / 9) as GLsizei, transpose, va.as_ptr());
    }
}

pub fn uniform_matrix3x2fv(location: GLint, transpose: GLboolean, va: &[GLfloat]) {
    unsafe {
        raw::UniformMatrix3x2fv(location, (va.len() / 6) as GLsizei, transpose, va.as_ptr());
    }
}

pub fn uniform_matrix3x4fv(location: GLint, transpose: GLboolean, va: &[GLfloat]) {
    unsafe {
        raw::UniformMatrix3x4fv(location, (va.len() / 12) as GLsizei, transpose, va.as_ptr());
    }
}

pub fn uniform_matrix4fv(location: GLint, transpose: GLboolean, va: &[GLfloat]) {
    unsafe {
        raw::UniformMatrix4fv(location, (va.len() / 16) as GLsizei, transpose, va.as_ptr());
    }
}

pub fn uniform_matrix4x2fv(location: GLint, transpose: GLboolean, va: &[GLfloat]) {
    unsafe {
        raw::UniformMatrix4x2fv(location, (va.len() / 8) as GLsizei, transpose, va.as_ptr());
    }
}

pub fn uniform_matrix4x3fv(location: GLint, transpose: GLboolean, va: &[GLfloat]) {
    unsafe {
        raw::UniformMatrix4x3fv(location, (va.len() / 12) as GLsizei, transpose, va.as_ptr());
    }
}

pub fn use_program(program: GLuint) {
    unsafe {
        raw::UseProgram(program);
    }
}

pub fn vertex_attrib1f(index: GLuint, x: GLfloat) {
    unsafe {
        raw::VertexAttrib1f(index, x);
    }
}

pub fn vertex_attrib1fv(index: GLuint, va: &[GLfloat]) {
    unsafe {
        raw::VertexAttrib1fv(index, va.as_ptr());
    }
}

pub fn vertex_attrib2f(index: GLuint, x: GLfloat, y: GLfloat) {
    unsafe {
        raw::VertexAttrib2f(index, x, y);
    }
}

pub fn vertex_attrib2fv(index: GLuint, va: &[GLfloat]) {
    unsafe {
        raw::VertexAttrib2fv(index, va.as_ptr());
    }
}

pub fn vertex_attrib3f(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat) {
    unsafe {
        raw::VertexAttrib3f(index, x, y, z);
    }
}

pub fn vertex_attrib3fv(index: GLuint, va: &[GLfloat]) {
    unsafe {
        raw::VertexAttrib3fv(index, va.as_ptr());
    }
}

pub fn vertex_attrib4f(index: GLuint, x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat) {
    unsafe {
        raw::VertexAttrib4f(index, x, y, z, w);
    }
}

pub fn vertex_attrib4fv(index: GLuint, va: &[GLfloat]) {
    unsafe {
        raw::VertexAttrib4fv(index, va.as_ptr());
    }
}

pub fn vertex_attrib_pointer(
    index: GLuint,
    size: GLint,
    type_: GLenum,
    normalized: GLboolean,
    stride: GLsizei,
    pointer: GLsizeiptr,
) {
    unsafe {
        raw::VertexAttribPointer(
            index,
            size,
            type_,
            normalized,
            stride,
            pointer as *const c_void,
        );
    }
}

pub fn viewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
    unsafe {
        raw::Viewport(x, y, width, height);
    }
}

#[cfg(test)]
mod tests {
    use super::so::SharedObject;
    use super::*;

    #[test]
    fn test_load_with() {
        let so = SharedObject::load("libGLESv2.so");
        load_with(|s| so.get_proc_address(s));
        assert_eq!(get_error(), raw::NO_ERROR);
    }
}
