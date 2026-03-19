use skia_safe::{Canvas, RGB};
use vector2d::Vector2D;
use crate::elements::Element;
use crate::attributes::attribute::Attribute;

pub struct Line{
    points: Vec<Box<dyn Attribute<Vector2D<f32>>>>,
    start: Box<dyn Attribute<f32>>,
    end: Box<dyn Attribute<f32>>,
    width: Box<dyn Attribute<f32>>,
    rgb: Box< dyn Attribute<RGB>>,
    stroke_caps: skia_safe::paint::Cap,
    is_antialias: bool,
}

impl Element for Line {
    fn draw_on(&self, frame: u32, canvas: &Canvas) -> Result<(),&'static str>
    {
        if self.points.len() < 2 { return Err("Line must have at least two points"); }

        let start = self.start.get_frame(frame);
        let end = self.end.get_frame(frame);

        if start >= end { return Err("Start is greater than End"); }

        todo!()
    }
}