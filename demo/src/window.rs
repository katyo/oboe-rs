use glutin::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
    window::{Window, WindowBuilder},
    Api, ContextBuilder, GlRequest, PossiblyCurrent, WindowedContext,
};

pub trait App {
    fn start(&mut self);
    fn stop(&mut self);
}

pub fn run<A: App + 'static>(mut app: A) {
    let event_loop = EventLoop::new();

    let mut view_context: Option<ViewContext> = None;

    #[cfg(not(target_os = "android"))]
    {
        let window_builder = WindowBuilder::new().with_title("Rust oboe demo");
        view_context = ViewContext::new(&event_loop, window_builder).into();
    }

    event_loop.run(move |event, _event_loop, control_flow| {
        *control_flow = ControlFlow::Wait;
        log::trace!("winit event: {:?}", event);

        match event {
            Event::Resumed => {
                #[cfg(target_os = "android")]
                if view_context.is_none() && ndk_glue::native_window().is_some() {
                    let window_builder = WindowBuilder::new().with_title("Rust oboe demo");
                    view_context = ViewContext::new(_event_loop, window_builder).into();
                }

                app.start();
            }

            Event::Suspended => {
                app.stop();

                #[cfg(target_os = "android")]
                if view_context.is_some() && ndk_glue::native_window().is_some() {
                    view_context = None;
                }
            }

            Event::RedrawRequested(_) => {
                if let Some(view_context) = &mut view_context {
                    // draw
                    view_context.swap();
                }
            }

            Event::MainEventsCleared => {
                if let Some(view_context) = &mut view_context {
                    // draw
                    view_context.swap();
                    view_context.window().request_redraw();
                }
            }

            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,

                WindowEvent::Resized(physical_size) => {
                    if let Some(view_context) = &mut view_context {
                        view_context.resize(physical_size);
                    }
                }

                _ => (),
            },

            _ => (),
        }
    });
}

struct ViewContext {
    //gl: Gl,
    context: WindowedContext<PossiblyCurrent>,
}

impl ViewContext {
    pub fn new(el: &EventLoopWindowTarget<()>, wb: WindowBuilder) -> Self {
        let context = ContextBuilder::new()
            .with_gl(GlRequest::Specific(Api::OpenGlEs, (2, 0)))
            .with_gl_debug_flag(false)
            .with_srgb(false)
            .with_vsync(true)
            .build_windowed(wb, el)
            .unwrap();
        let context = unsafe { context.make_current().unwrap() };
        //let gl = Gl::load_with(|ptr| window_context.get_proc_address(ptr) as *const _);
        Self { context }
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.context.resize(size);
    }

    pub fn window(&self) -> &Window {
        self.context.window()
    }

    pub fn swap(&mut self) {
        self.context.swap_buffers().unwrap();
    }
}
