use crate::motion_graphics::elements;
use crate::motion_graphics::elements::element::DrawInfo;
use skia_safe::{surfaces, Canvas, Color, ISize, Surface};
use std::cell::RefCell;
use vector2d::Vector2D;

pub struct Sequence{
    start_frame: usize,
    end_frame: usize,
    resolution: Vector2D<usize>,
    info: skia_safe::ImageInfo,
    elements: RefCell<Vec<Box<dyn elements::Element>>>,
    surface: Surface,
    draw_info: DrawInfo,
}

impl Sequence {
    pub fn new(width: usize, height: usize) -> Sequence {
        let size = ISize::new(width as i32, height as i32);
        let surface = surfaces::raster_n32_premul(size).expect("surface");
        let draw_info = DrawInfo{
            width : width as f32,
            height : height as f32,
        };

        Sequence {
            start_frame: 0,
            end_frame: 0,
            resolution: Vector2D::new(width, height),
            info: skia_safe::ImageInfo::new_a8(ISize::new(width as i32, height  as i32)),
            elements: RefCell::new(Vec::new()),
            surface,
            draw_info,
        }
    }

    pub fn push(&self, e: Box<dyn elements::Element>){
        self.elements.borrow_mut().push(e)
    }

    pub fn render_frame(&mut self, frame: usize) -> Vec<u8> {
        let mut canvas = self.surface.canvas();
        canvas.clear(Color::BLACK);

        for e in self.elements.borrow().iter(){
            match e.draw_on(frame, &mut canvas, &self.draw_info) {
                Ok(_) => {},
                Err(e) => {println!("{}",e)}
            }
        }

        let image = self.surface.image_snapshot();
        let pixelmap = image.peek_pixels().expect("pixelmap");
        let result = pixelmap.bytes().expect("bytes").to_vec();
        result
    }
}