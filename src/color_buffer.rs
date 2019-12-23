use crate::{prelude::*, raw, Vector4};
use std::cell::Cell;
use std::option::Option;

#[derive(Clone)]
pub struct ColorBuffer {
    clear_color: Cell<Option<Vector4>>,
}

impl ColorBuffer {
    pub fn new() -> Self {
        ColorBuffer {
            clear_color: Cell::new(None),
        }
    }

    pub fn set_clear_color(&self, color: Vector4) {
        if let Some(v) = self.clear_color.get() {
            if v != color {
                self.clear_color.set(Some(color));
            }
        } else {
            self.clear_color.set(Some(color));
        }
    }

    pub fn set_default_blend_func(&self) {
        crate::blend_func(raw::SRC_ALPHA, raw::ONE_MINUS_SRC_ALPHA);
    }

    pub fn clear(&self) {
        crate::clear(raw::COLOR_BUFFER_BIT);
    }

    pub fn enable_blend(&self) {
        crate::enable(raw::BLEND);
    }

    pub fn disable_blend(&self) {
        crate::disable(raw::BLEND);
    }
}

impl Bindable for ColorBuffer {
    fn bind(&self) {
        if let Some(color) = self.clear_color.get() {
            crate::clear_color(color.x, color.y, color.z, color.w);
            crate::clear(raw::COLOR_BUFFER_BIT);
        }
    }
    fn unbind(&self) {}
}
