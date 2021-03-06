use crate::{gl, prelude::*, GLboolean, GLenum, GLint, GLsizei, GLsizeiptr, GLuint};

#[derive(Clone, Default, Debug)]
pub struct Buffer {
    buffer_type: GLuint,
    vbo: GLuint,
}

impl Buffer {
    pub fn new_array() -> Buffer {
        Self::new(gl::ARRAY_BUFFER)
    }

    pub fn new_element_array() -> Buffer {
        Self::new(gl::ELEMENT_ARRAY_BUFFER)
    }

    pub fn new_draw_indirect() -> Buffer {
        Self::new(gl::DRAW_INDIRECT_BUFFER)
    }

    pub fn new(buffer_type: GLuint) -> Buffer {
        let vbo = crate::new_buffer();
        crate::bind_buffer(buffer_type, vbo);
        Buffer { buffer_type, vbo }
    }

    pub fn static_draw_data<T>(&self, data: &[T])
    where
        T: Sized,
    {
        crate::buffer_data(
            self.buffer_type, // target
            -1,               // size of data in bytes
            Some(data),       // pointer to data
            gl::STATIC_DRAW,  // usage
        );
    }

    pub fn stream_draw_data<T>(&self, data: &[T])
    where
        T: Sized,
    {
        crate::buffer_data(
            self.buffer_type, // target
            -1,               // size of data in bytes
            Some(data),       // pointer to data
            gl::STREAM_DRAW,  // usage
        );
    }

    pub fn stream_draw_data_null<T>(&self, size: usize)
    where
        T: Sized,
    {
        crate::buffer_data::<T>(
            self.buffer_type,                                // target
            (size * std::mem::size_of::<T>()) as GLsizeiptr, // size of data in bytes
            None,                                            // pointer to data
            gl::STREAM_DRAW,                                 // usage
        );
    }

    #[cfg(feature = "gl4")]
    pub unsafe fn map_buffer_range_write_invalidate<'r, T>(
        &self,
        offset: usize,
        size: usize,
    ) -> Option<MappedBuffer<'r, T>>
    where
        T: Sized,
    {
        let ptr = crate::map_buffer_range(
            self.buffer_type,                                  // target
            (offset * std::mem::size_of::<T>()) as GLsizeiptr, // offset
            (size * std::mem::size_of::<T>()) as GLsizeiptr,   //  length
            gl::MAP_WRITE_BIT | gl::MAP_INVALIDATE_RANGE_BIT,  // usage
        );
        if ptr == ::std::ptr::null_mut() {
            return None;
        }
        return Some(MappedBuffer {
            buffer_type: self.buffer_type,
            data: ::std::slice::from_raw_parts_mut(ptr as *mut T, size),
            position: 0,
        });
    }

    pub fn update<T>(&self, data: &[T])
    where
        T: Sized,
    {
        crate::buffer_sub_data(
            self.buffer_type, // target
            0,                // offset into the buffer
            data,             // pointer to data
        );
    }

    pub fn update_partial<T>(&self, offset: isize, data: &[T])
    where
        T: Sized,
    {
        crate::buffer_sub_data(
            self.buffer_type, // target
            offset,           // offset into the buffer
            data,             // pointer to data
        );
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        crate::delete_buffers(&[self.vbo]);
    }
}

impl Bindable for Buffer {
    fn bind(&self) {
        crate::bind_buffer(self.buffer_type, self.vbo);
    }

    fn unbind(&self) {
        crate::bind_buffer(self.buffer_type, 0);
    }
}

#[cfg(feature = "gl4")]
pub struct MappedBuffer<'a, DataT: 'a> {
    buffer_type: GLuint,
    data: &'a mut [DataT],
    position: usize,
}

#[cfg(feature = "gl4")]
impl<'a, DataT: 'a> MappedBuffer<'a, DataT> {
    pub fn clear(&mut self) {
        self.position = 0;
    }

    pub fn push(&mut self, data: DataT) {
        if self.position < self.data.len() {
            *unsafe { self.data.get_unchecked_mut(self.position) } = data;
            self.position += 1;
        }
    }
}

#[cfg(feature = "gl4")]
impl<'a, DataT: 'a> ::std::ops::Deref for MappedBuffer<'a, DataT> {
    type Target = [DataT];

    fn deref(&self) -> &Self::Target {
        self.data
    }
}

#[cfg(feature = "gl4")]
impl<'a, DataT: 'a> ::std::ops::DerefMut for MappedBuffer<'a, DataT> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data
    }
}

#[cfg(feature = "gl4")]
impl<'a, DataT: 'a> Drop for MappedBuffer<'a, DataT> {
    fn drop(&mut self) {
        crate::unmap_buffer(self.buffer_type);
    }
}

#[derive(Clone, Copy, Debug)]
pub struct VertexAttrib {
    location: GLuint,
    components: GLint,
    data_type: GLenum,
    normalized: GLboolean,
    stride: GLsizei,
    offset: GLsizeiptr,
}

impl VertexAttrib {
    pub fn new(
        location: GLuint,
        components: GLint,
        data_type: GLenum,
        normalized: GLboolean,
        stride: GLsizei,
        offset: GLsizeiptr,
    ) -> Self {
        Self {
            location,
            components,
            data_type,
            normalized,
            stride,
            offset,
        }
    }
}

impl Bindable for VertexAttrib {
    fn bind(&self) {
        crate::enable_vertex_attrib_array(self.location);
        crate::vertex_attrib_pointer(
            self.location,
            self.components,
            self.data_type,
            self.normalized,
            self.stride,
            self.offset,
        );
    }
    fn unbind(&self) {
        crate::disable_vertex_attrib_array(self.location);
    }
}

#[derive(Clone, Default, Debug)]
pub struct VertexArray {
    vao: GLuint,
}

impl VertexArray {
    pub fn new() -> VertexArray {
        VertexArray {
            vao: crate::new_vertex_array(),
        }
    }

    pub fn enable_attrib(&self, attr: &VertexAttrib) {
        crate::enable_vertex_attrib_array(attr.location);
        crate::vertex_attrib_pointer(
            attr.location,
            attr.components,
            attr.data_type,
            attr.normalized,
            attr.stride,
            attr.offset,
        );
    }

    pub fn disable_attrib(self, attr: &VertexAttrib) {
        crate::disable_vertex_attrib_array(attr.location);
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        crate::delete_vertex_arrays(&[self.vao]);
    }
}

impl Bindable for VertexArray {
    fn bind(&self) {
        crate::bind_vertex_array(self.vao);
    }

    fn unbind(&self) {
        crate::bind_vertex_array(0);
    }
}
