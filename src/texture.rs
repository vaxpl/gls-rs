#![allow(clippy::upper_case_acronyms)]
#[cfg(any(feature = "gles1", feature = "gles2", feature = "gles3"))]
use crate::GLeglImageOES;
use crate::{
    gl,
    gl::{GLenum, GLint, GLsizei, GLuint},
    prelude::*,
    Finalizer,
};
use std::cell::Cell;
use std::fmt::Debug;

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
    #[cfg(any(feature = "gles1", feature = "gles2", feature = "gles3"))]
    TextureExternalOES = gl::TEXTURE_EXTERNAL_OES as isize,
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
    TextureCubeMapPositiveX = gl::TEXTURE_CUBE_MAP_POSITIVE_X as isize,
    TextureCubeMapNegativeX = gl::TEXTURE_CUBE_MAP_NEGATIVE_X as isize,
    TextureCubeMapPositiveY = gl::TEXTURE_CUBE_MAP_POSITIVE_Y as isize,
    TextureCubeMapNegativeY = gl::TEXTURE_CUBE_MAP_NEGATIVE_Y as isize,
    TextureCubeMapPositiveZ = gl::TEXTURE_CUBE_MAP_POSITIVE_Z as isize,
    TextureCubeMapNegativeZ = gl::TEXTURE_CUBE_MAP_NEGATIVE_Z as isize,
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
pub struct TextureLoadOptions<'b> {
    path: Option<&'b str>,
    bytes: Option<&'b [u8]>,
    #[cfg(any(feature = "gles1", feature = "gles2", feature = "gles3"))]
    egl_image: Option<GLeglImageOES>,
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
    allocate_storage: bool,
}

impl<'b> TextureLoadOptions<'b> {
    pub fn from_bytes_rgb(bytes: &'b [u8], width: usize, height: usize) -> Self {
        Self {
            bytes: Some(bytes),
            internal_format: TextureFormat::Rgb8,
            width,
            height,
            format: TextureFormat::Rgb,
            ..Default::default()
        }
    }

    pub fn from_bytes_rgba(bytes: &'b [u8], width: usize, height: usize) -> Self {
        Self {
            bytes: Some(bytes),
            internal_format: TextureFormat::Rgba,
            width,
            height,
            format: TextureFormat::Rgba,
            ..Default::default()
        }
    }
}

impl<'b> Default for TextureLoadOptions<'b> {
    fn default() -> Self {
        Self {
            path: None,
            bytes: None,
            #[cfg(any(feature = "gles1", feature = "gles2", feature = "gles3"))]
            egl_image: None,
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
            allocate_storage: false,
        }
    }
}

pub struct TextureLoader<'a, 'b> {
    options: TextureLoadOptions<'b>,
    finalizer: Cell<Option<TextureFinalizer<'a>>>,
}

impl<'a, 'b> TextureLoader<'a, 'b> {
    pub fn default() -> Self {
        Self {
            options: Default::default(),
            finalizer: Cell::new(None),
        }
    }

    pub fn with_bytes(&mut self, bytes: &'b [u8]) -> &mut Self {
        self.options.bytes = Some(bytes);
        self
    }

    #[cfg(any(feature = "gles1", feature = "gles2", feature = "gles3"))]
    pub fn with_egl_image(&mut self, egl_image: GLeglImageOES) -> &mut Self {
        self.options.egl_image = Some(egl_image);
        self.options.target = TextureTarget::TextureExternalOES;
        self
    }

    pub fn with_2d(&mut self) -> &mut Self {
        self.options.target = TextureTarget::Texture2D;
        self
    }

    pub fn with_3d(&mut self) -> &mut Self {
        self.options.target = TextureTarget::Texture3D;
        self
    }

    pub fn with_size(&mut self, width: usize, height: usize) -> &mut Self {
        self.options.width = width;
        self.options.height = height;
        self
    }

    pub fn with_internal_format(&mut self, internal_format: TextureFormat) -> &mut Self {
        self.options.internal_format = internal_format;
        self
    }

    pub fn with_format(&mut self, format: TextureFormat) -> &mut Self {
        self.options.format = format;
        self
    }

    pub fn with_texel(&mut self, texel: TextureTexel) -> &mut Self {
        self.options.texel = texel;
        self
    }

    pub fn with_min_nearest(&mut self) -> &mut Self {
        self.options.min_filter = TextureFilter::Nearest;
        self
    }

    pub fn with_min_linear(&mut self) -> &mut Self {
        self.options.min_filter = TextureFilter::Linear;
        self
    }

    pub fn with_mag_nearest(&mut self) -> &mut Self {
        self.options.mag_filter = TextureFilter::Nearest;
        self
    }

    pub fn with_mag_linear(&mut self) -> &mut Self {
        self.options.mag_filter = TextureFilter::Linear;
        self
    }

    pub fn with_nearest(&mut self) -> &mut Self {
        self.options.min_filter = TextureFilter::Nearest;
        self.options.mag_filter = TextureFilter::Nearest;
        self
    }

    pub fn with_linear(&mut self) -> &mut Self {
        self.options.min_filter = TextureFilter::Linear;
        self.options.mag_filter = TextureFilter::Linear;
        self
    }

    pub fn with_gen_mipmaps(&mut self) -> &mut Self {
        self.options.gen_mipmaps = true;
        self
    }

    pub fn with_allocate_storage(&mut self) -> &mut Self {
        self.options.allocate_storage = true;
        self
    }

    pub fn with_finalizer<F>(&mut self, finalizer: F) -> &mut Self
    where
        F: Fn(&Texture<'a>) + 'a,
    {
        self.finalizer.replace(Some(Box::new(finalizer)));
        self
    }

    pub fn load(&self) -> Result<Texture<'a>, String> {
        Texture::load(self.options, self.finalizer.replace(None))
    }
}

type TextureFinalizer<'a> = Finalizer<'a, Texture<'a>>;

#[derive(Default)]
pub struct Texture<'a> {
    id: GLuint,
    target: TextureTarget,
    width: usize,
    height: usize,
    finalizer: Option<TextureFinalizer<'a>>,
}

impl<'a> Debug for Texture<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Texture")
            .field("id", &self.id)
            .field("target", &self.target)
            .field("width", &self.width)
            .field("height", &self.height)
            .finish()
    }
}

impl<'a> Drop for Texture<'a> {
    fn drop(&mut self) {
        if let Some(ref f) = self.finalizer.take() {
            (f)(self);
        }
        crate::delete_textures(&[self.id]);
        self.id = 0;
    }
}

impl<'a> Texture<'a> {
    /// Construct a texture with loading options.
    /// # Note
    /// Auto bind the texture when loaded.
    pub fn load<'b>(
        options: TextureLoadOptions<'b>,
        finalizer: Option<TextureFinalizer<'a>>,
    ) -> Result<Texture<'a>, String> {
        let texture = Texture {
            id: crate::new_texture(),
            target: options.target,
            width: options.width,
            height: options.height,
            finalizer,
        };
        texture.bind();
        texture.update(options)?;
        Ok(texture)
    }

    /// Set min filter parameter of the Texture.
    pub fn set_min_filter(&self, filter: TextureFilter) {
        crate::tex_parameteri(
            self.target as GLenum,
            gl::TEXTURE_MIN_FILTER,
            filter as GLint,
        );
    }

    /// Set mag filter parameter of the Texture.
    pub fn set_mag_filter(&self, filter: TextureFilter) {
        crate::tex_parameteri(
            self.target as GLenum,
            gl::TEXTURE_MAG_FILTER,
            filter as GLint,
        );
    }

    /// Set filter parameters of the Texture.
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

    /// Set wrap S parameter of the Texture.
    pub fn set_wrap_s(&self, wrap_s: TextureWrap) {
        crate::tex_parameteri(self.target as GLenum, gl::TEXTURE_WRAP_S, wrap_s as GLint);
    }

    /// Set wrap T parameter of the Texture.
    pub fn set_wrap_t(&self, wrap_t: TextureWrap) {
        crate::tex_parameteri(self.target as GLenum, gl::TEXTURE_WRAP_T, wrap_t as GLint);
    }

    /// Set wrapping parameters of the Texture.
    pub fn set_wraps(&self, wrap_s: TextureWrap, wrap_t: TextureWrap) {
        crate::tex_parameteri(self.target as GLenum, gl::TEXTURE_WRAP_S, wrap_s as GLint);
        crate::tex_parameteri(self.target as GLenum, gl::TEXTURE_WRAP_T, wrap_t as GLint);
    }

    /// Update contents and attributes with TextureLoadOptions.
    pub fn update(&self, options: TextureLoadOptions<'_>) -> Result<(), String> {
        // crate::bind_texture(self.target as GLenum, self.id);

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
        } else if options.allocate_storage {
            crate::tex_image2d(
                self.target as GLenum,
                options.level as GLint,
                options.internal_format as GLint,
                options.width as GLsizei,
                options.height as GLsizei,
                0,
                options.format as GLuint,
                options.texel as GLuint,
                None::<&[u8]>,
            );
        }

        #[cfg(any(feature = "gles1", feature = "gles2", feature = "gles3"))]
        {
            if let Some(egl_image) = options.egl_image {
                crate::egl_image_target_texture_2d_oes(self.target as GLenum, egl_image);
            }
        }

        if options.gen_mipmaps {
            crate::generate_mipmap(self.target as GLenum);
        }

        // crate::bind_texture(self.target as GLenum, 0);

        Ok(())
    }

    /// Update contents of the texture with EGLImage.
    /// # Note
    /// Must be binded before call the routine.
    #[cfg(any(feature = "gles1", feature = "gles2", feature = "gles3"))]
    pub fn update_with_egl_image(&self, egl_image: GLeglImageOES) {
        // crate::bind_texture(self.target as GLenum, self.id);
        crate::egl_image_target_texture_2d_oes(self.target as GLenum, egl_image);
        // crate::bind_texture(self.target as GLenum, 0);
    }

    /// Returns the Id of the Texture.
    pub fn id(&self) -> GLuint {
        self.id
    }

    /// Returns the Target of the Texture.
    pub fn target(&self) -> TextureTarget {
        self.target
    }
}

impl<'a> Bindable for Texture<'a> {
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

    fn unbind_at(&self, _slot: u32) {
        self.unbind();
        crate::active_texture(gl::TEXTURE0);
    }
}
