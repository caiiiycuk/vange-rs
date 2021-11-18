#![allow(clippy::single_match)]
use vangers::{
    config,
    render::{ScreenTargets, DEPTH_FORMAT},
};

use futures::executor::{LocalPool, LocalSpawner, block_on};
use log::info;

use raw_window_handle::{ RawWindowHandle, HasRawWindowHandle, WebHandle };

struct WebWindow {
}

unsafe impl HasRawWindowHandle for WebWindow {
    fn raw_window_handle(&self) -> RawWindowHandle {
        RawWindowHandle::Web(WebHandle::empty())
    }
}

pub trait Application {
    // fn on_key(&mut self, input: event::KeyboardInput) -> bool;
    // fn on_mouse_wheel(&mut self, _delta: event::MouseScrollDelta) {}
    // fn on_cursor_move(&mut self, _position: (f64, f64)) {}
    // fn on_mouse_button(&mut self, _state: event::ElementState, _button: event::MouseButton) {}
    fn resize(&mut self, _device: &wgpu::Device, _extent: wgpu::Extent3d) {}
    fn reload(&mut self, device: &wgpu::Device);
    fn update(
        &mut self,
        device: &wgpu::Device,
        delta: f32,
        spawner: &LocalSpawner,
    ) -> Vec<wgpu::CommandBuffer>;
    fn draw(
        &mut self,
        device: &wgpu::Device,
        targets: ScreenTargets,
        spawner: &LocalSpawner,
    ) -> wgpu::CommandBuffer;
}

pub struct Harness {
    task_pool: LocalPool,
    // event_loop: EventLoop<()>,
    // window: Window,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub downlevel_caps: wgpu::DownlevelCapabilities,
    surface: wgpu::Surface,
    pub color_format: wgpu::TextureFormat,
    pub extent: wgpu::Extent3d,
    reload_on_focus: bool,
    depth_target: wgpu::TextureView,
}

pub struct HarnessOptions {
    pub title: &'static str,
    pub uses_level: bool,
}

impl Harness {
    pub fn init(options: HarnessOptions) -> (Self, config::Settings) {
        block_on(Harness::init_async(options))
    }

    pub async fn init_async(options: HarnessOptions) -> (Self, config::Settings) {
        info!("Loading the settings");
        let settings = config::Settings::load("config/settings.ron");
        let extent = wgpu::Extent3d {
            width: settings.window.size[0],
            height: settings.window.size[1],
            depth_or_array_layers: 1,
        };

        info!("Initializing the window");
        let instance = wgpu::Instance::new(settings.backend.to_wgpu());
        // let event_loop = EventLoop::new();
        let window = WebWindow {};
        let surface = unsafe { instance.create_surface(&window) };

        info!("Initializing the device:adapter");
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            }).await.expect("Unable to initialize GPU via the selected backend (adapter).");

        let downlevel_caps = adapter.get_downlevel_properties();
        let adapter_limits = adapter.limits();

        #[cfg(target_arch = "wasm32")]
        let mut limits = wgpu::Limits::downlevel_webgl2_defaults();

        #[cfg(not(target_arch = "wasm32"))]
        let mut limits = wgpu::Limits::downlevel_defaults();

        if options.uses_level {
            let desired_height = 16 << 10;
            limits.max_texture_dimension_2d =
                if adapter_limits.max_texture_dimension_2d < desired_height {
                    log::warn!(
                        "Adapter only supports {} texutre size",
                        adapter_limits.max_texture_dimension_2d
                    );
                    adapter_limits.max_texture_dimension_2d
                } else {
                    desired_height
                };
        }

        info!("Initializing the device:request");
        let (device, queue) = adapter.request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits,
                },
                if settings.render.wgpu_trace_path.is_empty() {
                    None
                } else {
                    Some(std::path::Path::new(&settings.render.wgpu_trace_path))
                },
            ).await.expect("Unable to initialize GPU via the selected backend (request).");

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface
                .get_preferred_format(&adapter)
                .unwrap_or(wgpu::TextureFormat::Bgra8UnormSrgb),
            width: extent.width,
            height: extent.height,
            present_mode: wgpu::PresentMode::Mailbox,
        };
        surface.configure(&device, &config);

        let depth_target = device
            .create_texture(&wgpu::TextureDescriptor {
                label: Some("Depth"),
                size: extent,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: DEPTH_FORMAT,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            })
            .create_view(&wgpu::TextureViewDescriptor::default());

        let harness = Harness {
            task_pool: LocalPool::new(),
            // event_loop,
            // window,
            device,
            downlevel_caps,
            queue,
            surface,
            color_format: config.format,
            extent,
            reload_on_focus: settings.window.reload_on_focus,
            depth_target,
        };

        (harness, settings)
    }

    pub fn main_loop<A: 'static + Application>(self, mut app: A) {
        use std::time;

        let mut last_time = time::Instant::now();

        let mut needs_reload = false;
        let Harness {
            mut task_pool,
            // event_loop,
            // window,
            device,
            queue,
            downlevel_caps: _,
            surface,
            color_format,
            mut extent,
            reload_on_focus,
            mut depth_target,
        } = self;
/* 
        event_loop.run(move |event, _, control_flow| {
            let _ = window;
            *control_flow = ControlFlow::Poll;
            task_pool.run_until_stalled();

            match event {
                event::Event::WindowEvent {
                    event: event::WindowEvent::Resized(size),
                    ..
                } => {
                    info!("Resizing to {:?}", size);
                    extent = wgpu::Extent3d {
                        width: size.width,
                        height: size.height,
                        depth_or_array_layers: 1,
                    };
                    let config = wgpu::SurfaceConfiguration {
                        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                        format: color_format,
                        width: size.width,
                        height: size.height,
                        present_mode: wgpu::PresentMode::Mailbox,
                    };
                    surface.configure(&device, &config);
                    depth_target = device
                        .create_texture(&wgpu::TextureDescriptor {
                            label: Some("Depth"),
                            size: extent,
                            mip_level_count: 1,
                            sample_count: 1,
                            dimension: wgpu::TextureDimension::D2,
                            format: DEPTH_FORMAT,
                            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                        })
                        .create_view(&wgpu::TextureViewDescriptor::default());
                    app.resize(&device, extent);
                }
                event::Event::WindowEvent { event, .. } => match event {
                    event::WindowEvent::Focused(false) => {
                        needs_reload = reload_on_focus;
                    }
                    event::WindowEvent::Focused(true) if needs_reload => {
                        info!("Reloading shaders");
                        app.reload(&device);
                        needs_reload = false;
                    }
                    event::WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    }
                    event::WindowEvent::KeyboardInput { input, .. } => {
                        if !app.on_key(input) {
                            *control_flow = ControlFlow::Exit;
                        }
                    }
                    event::WindowEvent::MouseWheel { delta, .. } => app.on_mouse_wheel(delta),
                    event::WindowEvent::CursorMoved { position, .. } => {
                        app.on_cursor_move(position.into())
                    }
                    event::WindowEvent::MouseInput { state, button, .. } => {
                        app.on_mouse_button(state, button)
                    }
                    _ => {}
                },
                event::Event::MainEventsCleared => {
                    let spawner = task_pool.spawner();

                    let mut delta: f32 = 16.0;

                    #[cfg(not(target_arch = "wasm32"))] {
                        let duration = time::Instant::now() - last_time;
                        last_time += duration;
                        delta = duration.as_secs() as f32 + duration.subsec_nanos() as f32 * 1.0e-9;
                    }

                    let update_command_buffers = app.update(&device, delta, &spawner);
                    if !update_command_buffers.is_empty() {
                        queue.submit(update_command_buffers);
                    }

                    match surface.get_current_texture() {
                        Ok(frame) => {
                            let view = frame
                                .texture
                                .create_view(&wgpu::TextureViewDescriptor::default());
                            let targets = ScreenTargets {
                                extent,
                                color: &view,
                                depth: &depth_target,
                            };
                            let render_command_buffer = app.draw(&device, targets, &spawner);
                            queue.submit(Some(render_command_buffer));
                            frame.present();
                        }
                        Err(_) => {}
                    };

                    profiling::finish_frame!();
                }
                _ => (),
            }
        });
        */
        let mut render_frame = move || {
            info!("Render frame");
            let spawner = task_pool.spawner();

            let duration = time::Instant::now() - last_time;
            last_time += duration;
            let mut delta = duration.as_secs() as f32 + duration.subsec_nanos() as f32 * 1.0e-9;

            delta = 16.0;

            let update_command_buffers = app.update(&device, delta, &spawner);
            if !update_command_buffers.is_empty() {
                queue.submit(update_command_buffers);
            }

            match surface.get_current_texture() {
                Ok(frame) => {
                    let view = frame
                        .texture
                        .create_view(&wgpu::TextureViewDescriptor::default());
                    let targets = ScreenTargets {
                        extent,
                        color: &view,
                        depth: &depth_target,
                    };
                    let render_command_buffer = app.draw(&device, targets, &spawner);
                    queue.submit(Some(render_command_buffer));
                    frame.present();
                }
                Err(_) => {}
            };
        };

        render_frame();
        // set_main_loop(render_frame);

         panic!("This is not an error, avoid desturction");
    }
}

#[allow(non_camel_case_types)]
type em_callback_func = unsafe extern fn();
extern {
    fn emscripten_set_main_loop(func : em_callback_func, fps : std::os::raw::c_int, simulate_infinite_loop : std::os::raw::c_int);
}

thread_local!(static MAIN_LOOP_CALLBACK: std::cell::RefCell<*mut std::os::raw::c_void> = std::cell::RefCell::new(std::ptr::null_mut()));

pub fn set_main_loop<F>(callback : F) where F : FnMut() {
    MAIN_LOOP_CALLBACK.with(|log| {
            *log.borrow_mut() = &callback as *const _ as *mut std::os::raw::c_void;
            });

    unsafe { emscripten_set_main_loop(wrapper::<F>, 0, 1); }

    unsafe extern "C" fn wrapper<F>() where F : FnMut() {
        MAIN_LOOP_CALLBACK.with(|z| {
            let closure = *z.borrow_mut() as *mut F;
            (*closure)();
        });
    }
}
