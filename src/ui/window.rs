use std::sync::Arc;
use pixels::{Pixels, SurfaceTexture};
use pixels::wgpu::naga::back::msl::sampler::Coord::Pixel;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::KeyCode;
use winit::window::WindowAttributes;
pub(crate) struct Window{
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) buffer: Vec<u8>
}



impl Window {
    pub fn new(width: u32, height: u32) -> Self {
        Window{
            width,
            height,
            buffer: vec![0; (width * height * 4) as usize]
        }
    }
    pub fn start(&self) {
        let event_loop = EventLoop::new().unwrap();
        let window = {
            let size = LogicalSize::new(self.width,self.height);
            #[allow(deprecated)]
            Arc::new(
                event_loop.create_window(
                    WindowAttributes::new()
                        .with_title("Motion K")
                        .with_inner_size(size)
                        .with_min_inner_size(size)
                ).unwrap()
            )
        };

        let mut pixels = {
            let window_size = window.inner_size();
            let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
            Pixels::new(self.width, self.height, surface_texture)
        };

        #[allow(deprecated)]
        let res = event_loop.run(|event, event_loop|{
            match event {
                Event::NewEvents(_) => {}
                Event::WindowEvent { event, .. } =>
                    {
                        if event == WindowEvent::RedrawRequested {

                            let mut px = pixels.as_mut().unwrap();
                            let mut bytes = px.frame_mut();
                            for (index, byte) in bytes.iter_mut().enumerate(){
                                *byte = self.buffer[index];
                            }
                            if let err = px.render() {
                                return;
                            }
                        }

                    }
                Event::DeviceEvent { .. } => {}
                Event::UserEvent(_) => {}
                Event::Suspended => {}
                Event::Resumed => {window.request_redraw();}
                Event::AboutToWait => {}
                Event::LoopExiting => {}
                Event::MemoryWarning => {}
            }
        });
    }

    pub fn update(& mut self, buffer: &Vec<u8>){
        for i in 0..buffer.len(){
            self.buffer[i] = buffer[i];
        }
    }
}
