use crate::{
    prelude::*,
    raw as gl,
    raw::{GLenum, GLint, GLsizei, GLuint},
};

#[derive(Clone, Copy, Debug)]
pub enum TextureFilter {
    Nearest = gl::NEAREST as isize,
    Linear = gl::LINEAR as isize,
}

impl Default for TextureFilter {
    fn default() -> Self {
        TextureFilter::Nearest
    }
}

#[derive(Clone, Copy, Debug)]
pub enum TextureFormat {
    Alpha = gl::ALPHA as isize,
    Luminance = gl::LUMINANCE as isize,
    LuminanceAlpha = gl::LUMINANCE_ALPHA as isize,
    R8 = gl::R8 as isize,
    R8SNorm = gl::R8_SNORM as isize,
    #[cfg(feature = "gl4")]
    R16 = gl::R16 as isize,
    R16F = gl::R16F as isize,
    #[cfg(feature = "gl4")]
    R16SNorm = gl::R16_SNORM as isize,
    Red = gl::RED as isize,
    Rg = gl::RG as isize,
    Rgb = gl::RGB as isize,
    Rgb8 = gl::RGB8 as isize,
    Rgba = gl::RGBA as isize,
}

impl Default for TextureFormat {
    fn default() -> Self {
        TextureFormat::Rgba
    }
}

#[derive(Clone, Copy, Debug)]
pub enum TextureTarget {
    Texture2D = gl::TEXTURE_2D as isize,
    Texture2DArray = gl::TEXTURE_2D_ARRAY as isize,
    Texture3D = gl::TEXTURE_3D as isize,
    #[cfg(feature = "gl4")]
    Texture3DArray = gl::TEXTURE_3D_ARRAY as isize,
    #[cfg(feature = "gl4")]
    ProxyTexture2D = gl::PROXY_TEXTURE_2D as isize,
    #[cfg(feature = "gl4")]
    ProxyTexture2DArray = gl::PROXY_TEXTURE_2D_ARRAY as isize,
    #[cfg(feature = "gl4")]
    ProxyTexture3D = gl::PROXY_TEXTURE_3D as isize,
    #[cfg(feature = "gl4")]
    ProxyTexture3DArray = gl::PROXY_TEXTURE_3D_ARRAY as isize,
    CubeMapPositiveX = gl::TEXTURE_CUBE_MAP_POSITIVE_X as isize,
    CubeMapNegativeX = gl::TEXTURE_CUBE_MAP_NEGATIVE_X as isize,
}

impl Default for TextureTarget {
    fn default() -> Self {
        TextureTarget::Texture2D
    }
}

#[derive(Clone, Copy, Debug)]
pub enum TextureTexel {
    // Byte
    S8 = gl::BYTE as isize,
    U8 = gl::UNSIGNED_BYTE as isize,
    #[cfg(feature = "gl4")]
    U8_332 = gl::UNSIGNED_BYTE_3_3_2 as isize,
    #[cfg(feature = "gl4")]
    U8_233_REV = gl::UNSIGNED_BYTE_2_3_3_REV as isize,
    // Float
    F16 = gl::HALF_FLOAT as isize,
    F32 = gl::FLOAT as isize,
    // Short
    S16 = gl::SHORT as isize,
    U16_565 = gl::UNSIGNED_SHORT_5_6_5 as isize,
    #[cfg(feature = "gl4")]
    U16_565Rev = gl::UNSIGNED_SHORT_5_6_5_REV as isize,
    U16_4444 = gl::UNSIGNED_SHORT_4_4_4_4 as isize,
    #[cfg(feature = "gl4")]
    U16_4444Rev = gl::UNSIGNED_SHORT_4_4_4_4_REV as isize,
    U16_5551 = gl::UNSIGNED_SHORT_5_5_5_1 as isize,
    #[cfg(feature = "gl4")]
    U16_1555Rev = gl::UNSIGNED_SHORT_1_5_5_5_REV as isize,
    // Int
    #[cfg(feature = "gl4")]
    U32_8888 = gl::UNSIGNED_INT_8_8_8_8 as isize,
    #[cfg(feature = "gl4")]
    U32_8888Rev = gl::UNSIGNED_INT_8_8_8_8_REV as isize,
    #[cfg(feature = "gl4")]
    U32_10_10_10_2 = gl::UNSIGNED_INT_10_10_10_2 as isize,
    #[cfg(feature = "gl4")]
    U32_2_10_10_10Rev = gl::GL_UNSIGNED_INT_2_10_10_10_REV as isize,
}

impl Default for TextureTexel {
    fn default() -> Self {
        TextureTexel::U8
    }
}

#[derive(Clone, Copy, Debug)]
pub enum TextureWrap {
    ClampToEdge = gl::CLAMP_TO_EDGE as isize,
    MirroredRepeat = gl::MIRRORED_REPEAT as isize,
    Repeat = gl::REPEAT as isize,
}

impl Default for TextureWrap {
    fn default() -> Self {
        TextureWrap::ClampToEdge
    }
}

#[derive(Clone, Copy, Debug)]
pub struct TextureLoadOptions<'a> {
    path: Option<&'a str>,
    bytes: Option<&'a [u8]>,
    target: TextureTarget,
    level: usize,
    internal_format: TextureFormat,
    width: usize,
    height: usize,
    format: TextureFormat,
    texel: TextureTexel,
    min_filter: TextureFilter,
    mag_filter: TextureFilter,
    wrap_s: TextureWrap,
    wrap_t: TextureWrap,
    gen_mipmaps: bool,
}

impl<'a> TextureLoadOptions<'a> {
    pub fn from_bytes_rgb(bytes: &'a [u8], width: usize, height: usize) -> Self {
        let mut options: TextureLoadOptions = Default::default();
        options.bytes = Some(bytes);
        options.internal_format = TextureFormat::Rgb8;
        options.width = width;
        options.height = height;
        options.format = TextureFormat::Rgb;
        options
    }

    pub fn from_bytes_rgba(bytes: &'a [u8], width: usize, height: usize) -> Self {
        let mut options: TextureLoadOptions = Default::default();
        options.bytes = Some(bytes);
        options.internal_format = TextureFormat::Rgba;
        options.width = width;
        options.height = height;
        options.format = TextureFormat::Rgba;
        options
    }
}

impl<'a> Default for TextureLoadOptions<'a> {
    fn default() -> Self {
        Self {
            path: None,
            bytes: None,
            target: TextureTarget::Texture2D,
            level: 0,
            internal_format: TextureFormat::Rgba,
            width: 0,
            height: 0,
            texel: TextureTexel::U8,
            format: TextureFormat::Rgba,
            min_filter: TextureFilter::Nearest,
            mag_filter: TextureFilter::Nearest,
            wrap_s: TextureWrap::ClampToEdge,
            wrap_t: TextureWrap::ClampToEdge,
            gen_mipmaps: false,
        }
    }
}

pub struct TextureLoader<'a> {
    options: TextureLoadOptions<'a>,
}

impl<'a> TextureLoader<'a> {
    pub fn default() -> Self {
        Self {
            options: Default::default(),
        }
    }

    pub fn with_bytes(mut self, bytes: &'a [u8]) -> Self {
        self.options.bytes = Some(bytes);
        self
    }

    pub fn with_2d(mut self) -> Self {
        self.options.target = TextureTarget::Texture2D;
        self
    }

    pub fn with_3d(mut self) -> Self {
        self.options.target = TextureTarget::Texture3D;
        self
    }

    pub fn with_size(mut self, width: usize, height: usize) -> Self {
        self.options.width = width;
        self.options.height = height;
        self
    }

    pub fn with_texel(mut self, texel: TextureTexel) -> Self {
        self.options.texel = texel;
        self
    }

    pub fn with_min_nearest(mut self) -> Self {
        self.options.min_filter = TextureFilter::Nearest;
        self
    }

    pub fn with_min_linear(mut self) -> Self {
        self.options.min_filter = TextureFilter::Linear;
        self
    }

    pub fn with_mag_nearest(mut self) -> Self {
        self.options.mag_filter = TextureFilter::Nearest;
        self
    }

    pub fn with_mag_linear(mut self) -> Self {
        self.options.mag_filter = TextureFilter::Linear;
        self
    }

    pub fn with_nearest(mut self) -> Self {
        self.options.min_filter = TextureFilter::Nearest;
        self.options.mag_filter = TextureFilter::Nearest;
        self
    }

    pub fn with_linear(mut self) -> Self {
        self.options.min_filter = TextureFilter::Linear;
        self.options.mag_filter = TextureFilter::Linear;
        self
    }

    pub fn with_gen_mipmaps(mut self) -> Self {
        self.options.gen_mipmaps = true;
        self
    }

    pub fn load(self) -> Result<Texture, String> {
        Texture::load(self.options)
    }
}

#[derive(Clone, Default, Debug)]
pub struct Texture {
    id: GLuint,
    target: TextureTarget,
}

impl Drop for Texture {
    fn drop(&mut self) {
        crate::delete_textures(&[self.id]);
    }
}

impl Texture {
    pub fn load<'a>(options: TextureLoadOptions<'a>) -> Result<Texture, String> {
        let texture = Texture {
            id: crate::new_texture(),
            target: options.target,
        };
        texture.update(options)?;
        Ok(texture)
    }

    pub fn set_min_filter(&self, filter: TextureFilter) {
        crate::tex_parameteri(
            self.target as GLenum,
            gl::TEXTURE_MIN_FILTER,
            filter as GLint,
        );
    }

    pub fn set_mag_filter(&self, filter: TextureFilter) {
        crate::tex_parameteri(
            self.target as GLenum,
            gl::TEXTURE_MAG_FILTER,
            filter as GLint,
        );
    }

    pub fn set_filters(&self, min_filter: TextureFilter, mag_filter: TextureFilter) {
        crate::tex_parameteri(
            self.target as GLenum,
            gl::TEXTURE_MIN_FILTER,
            min_filter as GLint,
        );
        crate::tex_parameteri(
            self.target as GLenum,
            gl::TEXTURE_MAG_FILTER,
            mag_filter as GLint,
        );
    }

    pub fn set_wrap_s(&self, wrap_s: TextureWrap) {
        crate::tex_parameteri(self.target as GLenum, gl::TEXTURE_WRAP_S, wrap_s as GLint);
    }

    pub fn set_wrap_t(&self, wrap_t: TextureWrap) {
        crate::tex_parameteri(self.target as GLenum, gl::TEXTURE_WRAP_T, wrap_t as GLint);
    }

    pub fn set_wraps(&self, wrap_s: TextureWrap, wrap_t: TextureWrap) {
        crate::tex_parameteri(self.target as GLenum, gl::TEXTURE_WRAP_S, wrap_s as GLint);
        crate::tex_parameteri(self.target as GLenum, gl::TEXTURE_WRAP_T, wrap_t as GLint);
    }

    pub fn update<'a>(&self, options: TextureLoadOptions<'a>) -> Result<(), String> {
        crate::bind_texture(self.target as GLenum, self.id);

        // https://www.khronos.org/opengl/wiki/Common_Mistakes

        self.set_filters(options.min_filter, options.mag_filter);
        self.set_wraps(options.wrap_s, options.wrap_t);

        if let Some(bytes) = options.bytes {
            crate::tex_image2d(
                self.target as GLenum,
                options.level as GLint,
                options.internal_format as GLint,
                options.width as GLsizei,
                options.height as GLsizei,
                0,
                options.format as GLuint,
                options.texel as GLuint,
                Some(bytes),
            );
        }

        if options.gen_mipmaps {
            crate::generate_mipmap(self.target as GLenum);
        }

        crate::bind_texture(self.target as GLenum, 0);

        Ok(())
    }
}

impl Bindable for Texture {
    fn bind(&self) {
        crate::bind_texture(self.target as GLenum, self.id);
    }

    fn bind_at(&self, index: u32) {
        crate::active_texture(gl::TEXTURE0 + index);
        self.bind();
    }

    fn unbind(&self) {
        crate::bind_texture(self.target as GLenum, 0);
    }
}
