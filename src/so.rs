#[cfg(windows)]
use winapi::{
    shared::minwindef::HMODULE,
    um::libloaderapi::{FreeLibrary, GetProcAddress, LoadLibraryA},
};

#[cfg(unix)]
use libc::{dlclose, dlopen, dlsym};
use std::ffi::CString;
use std::os::raw::c_void;

/// Shared library helper for Unix platforms.
///
/// **Note**: The loaded library will auto unload when the object dropped.
#[derive(Clone, Debug)]
pub struct SharedObject {
    #[cfg(windows)]
    handle: HMODULE,
    #[cfg(unix)]
    handle: *mut c_void,
}

impl SharedObject {
    /// Load the shared library.
    ///
    /// # Parameters
    ///
    /// * `file_name` - Which library to load into current process.
    #[cfg(windows)]
    pub fn load<T>(path: T) -> Self
    where
        T: AsRef<str>,
    {
        let cstr = CString::new(path.as_ref()).expect("opengl32.dll");
        Self {
            handle: unsafe { LoadLibraryA(cstr.as_ptr()) },
        }
    }
    #[cfg(unix)]
    pub fn load<T>(path: T) -> Self
    where
        T: AsRef<str>,
    {
        let cstr = CString::new(path.as_ref()).expect("libEGL.so");
        Self {
            handle: unsafe { dlopen(cstr.as_ptr(), 2) },
        }
    }

    /// Return the address of the procedure by name.
    ///
    /// # Parameters
    ///
    /// * `name` - Which procedure to find.
    pub fn get_proc_address<T>(&self, name: T) -> *const c_void
    where
        T: AsRef<str>,
    {
        let cstr = CString::new(name.as_ref()).unwrap();
        #[cfg(windows)]
        unsafe {
            GetProcAddress(self.handle, cstr.as_ptr()) as *const c_void
        }
        #[cfg(unix)]
        unsafe {
            dlsym(self.handle, cstr.as_ptr())
        }
    }
}

impl Drop for SharedObject {
    fn drop(&mut self) {
        if self.handle != std::ptr::null_mut() {
            unsafe {
                #[cfg(windows)]
                FreeLibrary(self.handle);
                #[cfg(unix)]
                dlclose(self.handle);
            }
            self.handle = std::ptr::null_mut();
        }
    }
}
