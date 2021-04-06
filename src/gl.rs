#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]
#![allow(dead_code)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::unused_unit)]
#![allow(clippy::upper_case_acronyms)]

include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));

// Re-Export types
pub use types::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::so::SharedObject;

    #[test]
    fn test_load_with() {
        let so = SharedObject::load("libGLESv2.so");
        load_with(|s| so.get_proc_address(s));
        unsafe {
            Viewport(0, 0, 100, 100);
            ClearColor(0.0, 0.0, 0.0, 1.0);
            Clear(COLOR_BUFFER_BIT | DEPTH_BUFFER_BIT);
        }
    }
}
