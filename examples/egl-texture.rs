#[cfg(target_arch = "aarch64")]
use egls::EnvironmentBuilder;
#[cfg(target_arch = "aarch64")]
use gls::{
    gl, prelude::*, uniform, AutoBinder, Buffer, ClearBuffers, GLint, GLsizeiptr, GLuint, Matrix4,
    Program, Shader, TextureLoader, Vector4, VertexArray, VertexAttrib, Viewport,
};

#[cfg(target_arch = "aarch64")]
fn main() {
    let (w, h) = (1920, 1080);
    let env = EnvironmentBuilder::defaults()
        .with_samples(4)
        .with_window_size(w as usize, h as usize)
        .build();

    gls::load_with(|s| egls::get_proc_address(s));

    // set viewport
    let viewport = Viewport::with_size(w as i32, h as i32);
    // set screen clear color
    let clear_buffers = ClearBuffers::new().with_color(Some(Vector4::new(0.3, 0.3, 0.5, 1.0)));

    let fs = include_str!("shaders/textured.frag");
    let vs = include_str!("shaders/textured.vert");
    let fs = Shader::from_frag_source(fs).unwrap();
    let vs = Shader::from_vert_source(vs).unwrap();
    let prog = Program::from_shaders(&[vs, fs]).unwrap();
    //let proj = gl::Matrix4::new_perspective(1.0, 65.0, 0.01, 10.0);
    let proj = Matrix4::new_orthographic(-1.0, 1.0, -1.0, 1.0, -2.0, 2.0);
    let position_aloc = prog.locate_attrib("a_position").unwrap();
    let texcoord_aloc = prog.locate_attrib("a_texcoord").unwrap_or(-1);
    let mvp_uloc = prog.locate_uniform("u_mvp").unwrap_or(-1);
    let texture_uloc = prog.locate_uniform("u_texture").unwrap_or(-1);
    prog.set_uniform(mvp_uloc, uniform!(mat4(&proj)));
    prog.set_uniform(texture_uloc, uniform!(int(0)));

    let texture = TextureLoader::default()
        .with_bytes(include_bytes!("images/cube-map-400x300.data"))
        .with_size(400, 300)
        .with_linear()
        .load()
        .unwrap();

    let vertices: Vec<f32> = vec![
        // positions      // texcords
        0.5, -0.5, 0.0, 1.0, 1.0, // bottom right
        -0.5, -0.5, 0.0, 0.0, 1.0, // bottom left
        0.0, 0.5, 0.0, 0.5, 0.0, // top
    ];

    // set up vertex buffer object
    let vbo = Buffer::new_array();
    vbo.bind();
    vbo.static_draw_data(&vertices[..]);
    vbo.unbind();

    // set up vertex array object
    let vao = VertexArray::new();
    vao.bind();
    vbo.bind();

    let a_position = VertexAttrib::new(
        position_aloc as GLuint,
        3,
        gl::FLOAT,
        gl::FALSE,
        (5 * std::mem::size_of::<f32>()) as GLint,
        0,
    );

    let a_texcoord = VertexAttrib::new(
        texcoord_aloc as GLuint,
        2,
        gl::FLOAT,
        gl::FALSE,
        (5 * std::mem::size_of::<f32>()) as GLint,
        (3 * std::mem::size_of::<f32>()) as GLsizeiptr,
    );

    vao.enable_attrib(&a_position);
    vao.enable_attrib(&a_texcoord);

    vbo.unbind();
    vao.unbind();

    for _a in 0..60 {
        let _a = AutoBinder::new(vec![&viewport, &clear_buffers, &prog, &texture, &vao]);

        gls::draw_arrays(
            gl::TRIANGLES, // mode
            0,             // starting index in the enabled arrays
            3,             // number of indices to be rendered
        );

        env.swap_buffers();
    }
}

#[cfg(not(target_arch = "aarch64"))]
fn main() {}
