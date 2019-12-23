use crate::prelude::Bindable;

#[derive(Clone, Copy, Default, Debug)]
pub struct Viewport {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl Viewport {
    pub fn with_offset(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self { x, y, w, h }
    }
    pub fn with_size(w: i32, h: i32) -> Self {
        Self { x: 0, y: 0, w, h }
    }

    pub fn get_offset(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn set_offset(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    pub fn get_size(&self) -> (i32, i32) {
        (self.w, self.h)
    }

    pub fn set_size(&mut self, w: i32, h: i32) {
        self.w = w;
        self.h = h;
    }
}

impl Bindable for Viewport {
    fn bind(&self) {
        if (self.w * self.h) > 0 {
            crate::viewport(self.x, self.y, self.w, self.h);
        }
    }
}
