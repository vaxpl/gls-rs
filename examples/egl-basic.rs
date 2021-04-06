#[cfg(target_arch = "aarch64")]
use egls::{self, EnvironmentBuilder};
#[cfg(target_arch = "aarch64")]
use gls::{self, gl};
#[cfg(target_arch = "aarch64")]
use rand::Rng;
#[cfg(target_arch = "aarch64")]
use std::thread::sleep;
#[cfg(target_arch = "aarch64")]
use std::time::Duration;

#[cfg(target_arch = "aarch64")]
fn main() {
    let (w, h) = (1920, 1080);
    let env = EnvironmentBuilder::defaults()
        .with_samples(4)
        .with_window_size(w as usize, h as usize)
        .build();

    gls::load_with(|s| egls::get_proc_address(s));

    let mut rng = rand::thread_rng();

    for _a in 0..60 {
        let r = rng.gen_range(0.0, 1.0);
        let g = rng.gen_range(0.0, 1.0);
        let b = rng.gen_range(0.0, 1.0);
        gls::clear_color(r, g, b, 1.0);
        gls::clear(gl::COLOR_BUFFER_BIT);
        env.swap_buffers();
        sleep(Duration::from_millis(15));
    }
}

#[cfg(not(target_arch = "aarch64"))]
fn main() {}
