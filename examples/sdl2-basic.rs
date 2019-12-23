use gls;
use sdl2;

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let window = video_subsystem
        .window("GlPlus - Basic", 800, 600)
        .opengl()
        .resizable()
        .build()
        .unwrap();
    // Create OpenGL context
    let _gl_context = window.gl_create_context().unwrap();
    // Load OpenGL routines
    gls::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::ffi::c_void);
    // Set screen clear color
    gls::clear_color(0.3, 0.3, 0.5, 1.0);

    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }
        gls::clear(gls::raw::COLOR_BUFFER_BIT);
        window.gl_swap_window();
    }
}
