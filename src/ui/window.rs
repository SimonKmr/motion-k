use pixels::{Pixels, SurfaceTexture};
use std::sync::Arc;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;

pub(crate) struct Window<'a>{
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) buffer: Vec<u8>,
    window: Arc<winit::window::Window>,
    frame: Pixels<'a>,
    event_loop: EventLoop<()>
}

impl Window<'_> {
    pub fn new(width: u32, height: u32) -> Self {

        let event_loop = EventLoop::new().unwrap();

        let t_window = Arc::new(WindowBuilder::new()
            .with_title("Motion-K")
            .with_inner_size(LogicalSize::new(width, height))
            .build(&event_loop).unwrap());

        let pixels = {
            let surface_texture = SurfaceTexture::new(width, height, Arc::clone(&t_window));
            Pixels::new(width, height, surface_texture)
        }.unwrap();

        Window{
            width,
            height,
            buffer: vec![0; (width * height * 4) as usize],
            frame: pixels,
            window: t_window,
            event_loop
        }
    }

    pub fn run(self){
        self.event_loop.run(move |event, elwt| {
            match event {
                // redraws when the OS asks
                Event::WindowEvent {
                    event: WindowEvent::RedrawRequested, ..
                } => {

                }

                // redraws every loop iteration (game loop style)
                Event::AboutToWait => {
                    self.window.request_redraw(); // triggers RedrawRequested
                }

                Event::WindowEvent {
                    event: WindowEvent::CloseRequested, ..
                } => {
                    elwt.exit();
                }

                _ => {}
            }
        }).unwrap();
    }
}



