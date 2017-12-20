extern crate cgmath;
extern crate env_logger;
extern crate getopts;
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
#[macro_use]
extern crate log;
extern crate time;
extern crate vangers;

mod app;

use vangers::{config, render};

fn main() {
    use std::env;
    env_logger::init().unwrap();

    let settings = config::Settings::load("config/settings.toml");

    let win_builder = glutin::WindowBuilder::new()
        .with_title(settings.window.title.clone())
        .with_dimensions(settings.window.size[0], settings.window.size[1]);
    let context_build = glutin::ContextBuilder::new()
        .with_gl_profile(glutin::GlProfile::Core)
        .with_vsync(true);
    let mut event_loop = glutin::EventsLoop::new();
    let (window, mut device, mut factory, main_color, main_depth) =
        gfx_window_glutin::init::<render::ColorFormat, render::DepthFormat>(
            win_builder,
            context_build,
            &event_loop,
        );

    let args: Vec<_> = env::args().collect();
    let mut options = getopts::Options::new();
    options
        .parsing_style(getopts::ParsingStyle::StopAtFirstFree)
        .optflag("h", "help", "print this help menu");

    let matches = options.parse(&args[1 ..]).unwrap();
    if matches.opt_present("h") || matches.free.len() != 1 {
        println!("Vangers model viewer");
        let brief = format!("Usage: {} [options] <path_to_model>", args[0]);
        println!("{}", options.usage(&brief));
        return;
    }

    let path = &matches.free[0];
    let mut app = app::ResourceView::new(path, &settings, main_color, main_depth, &mut factory);

    let mut encoder = gfx::Encoder::from(factory.create_command_buffer());
    let mut last_time = time::precise_time_s() as f32;
    let mut running = true;

    while running {
        use gfx::Device;
        use glutin::GlContext;

        let delta = time::precise_time_s() as f32 - last_time;

        event_loop.poll_events(|event| {
            if let glutin::Event::WindowEvent { event, .. } = event {
                if !app.react(event, delta, &mut factory) {
                    running = false;
                }
            }
        });
        app.draw(&mut encoder);

        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
        last_time += delta;
    }
}
