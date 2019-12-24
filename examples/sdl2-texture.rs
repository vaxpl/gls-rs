use gls::{
    prelude::*, uniform, AutoBinder, Buffer, ClearBuffers, GLint, GLsizei, GLsizeiptr, GLuint,
    Matrix4, Program, Shader, Texture, TextureLoader, Vector4, VertexArray, VertexAttrib, Viewport,
};
use sdl2;

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 1);
    let window = video_subsystem
        .window("Gls - Texture", 800, 600)
        .opengl()
        .resizable()
        .build()
        .unwrap();
    // create OpenGL context
    let _gl_context = window.gl_create_context().unwrap();
    // load OpenGL routines
    gls::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::ffi::c_void);

    // set cviewport
    let (w, h) = window.size();
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
        gls::raw::FLOAT,
        gls::raw::FALSE,
        (5 * std::mem::size_of::<f32>()) as GLint,
        0,
    );

    let a_texcoord = VertexAttrib::new(
        texcoord_aloc as GLuint,
        2,
        gls::raw::FLOAT,
        gls::raw::FALSE,
        (5 * std::mem::size_of::<f32>()) as GLint,
        (3 * std::mem::size_of::<f32>()) as GLsizeiptr,
    );

    vao.enable_attrib(&a_position);
    vao.enable_attrib(&a_texcoord);

    vbo.unbind();
    vao.unbind();

    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }

        let _a = AutoBinder::new(vec![&viewport, &clear_buffers, &prog, &texture, &vao]);

        gls::draw_arrays(
            gls::raw::TRIANGLES, // mode
            0,                   // starting index in the enabled arrays
            3,                   // number of indices to be rendered
        );

        window.gl_swap_window();
    }
}
