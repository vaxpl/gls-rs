use gl_generator::{Api, Fallbacks, GlobalGenerator, Profile, Registry};
use std::env;
use std::fs::File;
use std::path::Path;

fn main() {
    let mut api = Api::Gl;
    let mut version = (2, 0);
    let mut extensions: Vec<&str> = Vec::new();
    if cfg!(feature = "gl3") {
        version = (3, 3);
    }
    if cfg!(feature = "gl4") {
        version = (4, 5);
    }
    if cfg!(feature = "gles1") {
        api = Api::Gles1;
        version = (1, 0);
    }
    if cfg!(feature = "gles2") {
        api = Api::Gles2;
        version = (2, 0);
    }
    if cfg!(feature = "gles3") {
        api = Api::Gles2;
        version = (3, 2);
    }
    if cfg!(any(feature = "gles1", feature = "gles2", feature = "gles3")) {
        extensions.push("GL_OES_EGL_image");
        extensions.push("GL_OES_EGL_image_external");
        extensions.push("GL_EXT_YUV_target");
    }

    let dest = env::var("OUT_DIR").unwrap();
    let mut file = File::create(&Path::new(&dest).join("gl_bindings.rs")).unwrap();
    Registry::new(api, version, Profile::Core, Fallbacks::All, extensions)
        .write_bindings(GlobalGenerator, &mut file)
        .unwrap();
}
