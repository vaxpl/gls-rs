use gl::{
    uniform, AutoBinder, Bindable, Buffer, ClipRect, ColorBuffer, GLint, GLsizei, GLsizeiptr,
    GLuint, Matrix4, Program, Shader, Vector4, VertexArray, VertexAttrib,
};
use glplus as gl;
use sdl2;

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 1);
    let window = video_subsystem
        .window("GlPlus - Triangle", 800, 600)
        .opengl()
        .resizable()
        .build()
        .unwrap();
    // create OpenGL context
    let _gl_context = window.gl_create_context().unwrap();
    // load OpenGL routines
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::ffi::c_void);

    // set clip rect (viewport)
    let (w, h) = window.size();
    let clip_rect = ClipRect::with_size(w as i32, h as i32);
    // set screen clear color
    let color_buffer = ColorBuffer::new();
    color_buffer.set_clear_color(Vector4::new(0.3, 0.3, 0.5, 1.0));

    let fs = include_str!("shaders/colored.frag");
    let vs = include_str!("shaders/colored.vert");
    let fs = Shader::from_frag_source(fs).unwrap();
    let vs = Shader::from_vert_source(vs).unwrap();
    let prog = Program::from_shaders(&[vs, fs]).unwrap();
    //let proj = gl::Matrix4::new_perspective(1.0, 65.0, 0.01, 10.0);
    let proj = Matrix4::new_orthographic(-1.0, 1.0, -1.0, 1.0, -2.0, 2.0);
    let position_aloc = prog.locate_attrib("a_position").unwrap();
    let color_aloc = prog.locate_attrib("a_color").unwrap_or(-1);
    let mvp_uloc = prog.locate_uniform("u_mvp").unwrap_or(-1);
    prog.set_uniform(mvp_uloc, uniform!(mat4(&proj)));

    let vertices: Vec<f32> = vec![
        // positions      // colors
        0.5, -0.5, 0.0, 1.0, 0.0, 0.0, // bottom right
        -0.5, -0.5, 0.0, 0.0, 1.0, 0.0, // bottom left
        0.0, 0.5, 0.0, 0.0, 0.0, 1.0, // top
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
        (6 * std::mem::size_of::<f32>()) as GLint,
        0,
    );

    let a_color = VertexAttrib::new(
        color_aloc as GLuint,
        3,
        gl::FLOAT,
        gl::FALSE,
        (6 * std::mem::size_of::<f32>()) as GLint,
        (3 * std::mem::size_of::<f32>()) as GLsizeiptr,
    );

    vao.enable_attrib(&a_position);
    vao.enable_attrib(&a_color);

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

        let _a = AutoBinder::new(vec![&clip_rect, &color_buffer, &prog, &vao]);

        gl::draw_arrays(
            gl::TRIANGLES, // mode
            0,             // starting index in the enabled arrays
            3,             // number of indices to be rendered
        );

        window.gl_swap_window();
    }
}
