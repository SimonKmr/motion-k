use crate::motion_graphics::elements;
use crate::motion_graphics::elements::element::DrawInfo;
use skia_safe::{surfaces, Color, ISize};
use std::cell::RefCell;
use vector2d::Vector2D;

pub struct Sequence{
    start_frame: usize,
    end_frame: usize,
    resolution: Vector2D<usize>,
    info: skia_safe::ImageInfo,
    elements: RefCell<Vec<Box<dyn elements::Element>>>,
}

impl Sequence {
    pub fn new(width: usize, height: usize) -> Sequence {
        Sequence {
            start_frame: 0,
            end_frame: 0,
            resolution: Vector2D::new(width, height),
            info: skia_safe::ImageInfo::new_a8(ISize::new(width as i32, height  as i32)),
            elements: RefCell::new(Vec::new()),
        }
    }

    pub fn push(&self, e: Box<dyn elements::Element>){
        self.elements.borrow_mut().push(e)
    }

    pub fn render_frame<'a>(&mut self, frame: usize) -> Vec<u8> {
        let size = ISize::new(self.resolution.x as i32, self.resolution.y as i32);
        let mut surface = surfaces::raster_n32_premul(size).expect("surface");
        let mut canvas = surface.canvas();

        canvas.clear(Color::BLACK);
        let draw_info = DrawInfo{
            width : self.resolution.x as f32,
            height : self.resolution.y as f32,
        };

        for e in self.elements.borrow().iter(){
            match e.draw_on(frame, &mut canvas, &draw_info) {
                Ok(_) => {},
                Err(e) => {println!("{}",e)}
            }
        }

        let image =surface.image_snapshot();
        let pixelmap = image.peek_pixels().expect("pixelmap");
        let result = pixelmap.bytes().expect("bytes").to_vec();
        result
    }
}

