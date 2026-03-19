use std::cell::{Cell, RefCell};
use crate::elements;
use skia_safe::{surfaces, Color, ISize};
use vector2d::Vector2D;

pub struct Sequence{
    start_frame: u32,
    end_frame: u32,
    resolution: Vector2D<i32>,
    info: skia_safe::ImageInfo,
    elements: RefCell<Vec<Box<dyn elements::Element>>>,
}

impl Sequence {
    pub fn new(width: i32, height: i32) -> Sequence {
        Sequence {
            start_frame: 0,
            end_frame: 0,
            resolution: Vector2D::new(width, height),
            info: skia_safe::ImageInfo::new_a8(skia_safe::ISize::new(width, height)),
            elements: RefCell::new(Vec::new()),
        }
    }

    pub fn push(&self, e: Box<dyn elements::Element>){
        self.elements.borrow_mut().push(e)
    }

    pub fn render_frame<'a>(&mut self, frame: u32) -> Vec<u8> {
        let size = ISize::new(self.resolution.x, self.resolution.y);
        let mut surface = surfaces::raster_n32_premul(size).expect("surface");
        let mut canvas = surface.canvas();

        canvas.clear(Color::BLACK);

        for e in self.elements.borrow().iter(){
            match e.draw_on(frame, &mut canvas) {
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

