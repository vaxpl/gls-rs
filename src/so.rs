use libc;

/// Shared library helper for Unix platforms.
///
/// **Note**: The loaded library will auto unload when the object dropped.
#[derive(Clone, Debug)]
pub struct SharedObject {
    handle: *mut std::os::raw::c_void,
}

impl SharedObject {
    /// Load the shared library.
    ///
    /// # Parameters
    ///
    /// * `file_name` - Which library to load into current process.
    pub fn load(file_name: &'static str) -> Self {
        let file_name = std::ffi::CString::new(file_name).expect("libEGL.so");
        Self {
            handle: unsafe { libc::dlopen(file_name.as_ptr(), 2) },
        }
    }

    /// Return the address of the procedure by name.
    ///
    /// # Parameters
    ///
    /// * `proc_name` - Which procedure to find.
    pub fn get_proc_address(&self, proc_name: &'static str) -> *const std::os::raw::c_void {
        let proc_name = std::ffi::CString::new(proc_name).unwrap();
        unsafe { libc::dlsym(self.handle, proc_name.as_ptr()) }
    }
}

impl Drop for SharedObject {
    fn drop(&mut self) {
        if self.handle != std::ptr::null_mut() {
            unsafe {
                libc::dlclose(self.handle);
            }
            self.handle = std::ptr::null_mut();
        }
    }
}
