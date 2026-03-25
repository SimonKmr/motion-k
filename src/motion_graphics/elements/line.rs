use skia_safe::{Canvas, Point, RGB};
use vector2d::Vector2D;
use crate::motion_graphics::elements::Element;
use crate::motion_graphics::attributes::attribute::Attribute;

pub struct Line{
    pub position_offset: Box<dyn Attribute<Vector2D<f32>>>,
    pub points: Vec<Box<dyn Attribute<Vector2D<f32>>>>,
    pub start: Box<dyn Attribute<f32>>,
    pub end: Box<dyn Attribute<f32>>,
    pub width: Box<dyn Attribute<f32>>,
    pub color: Box< dyn Attribute<RGB>>,
    pub stroke_caps: skia_safe::paint::Cap,
    pub is_antialias: bool,
}

impl Line{
    pub fn new() -> Self{
        todo!()
    }
}

impl Element for Line {
    fn draw_on(&self, frame: usize, canvas: &Canvas) -> Result<(),&'static str>
    {
        if self.points.len() < 2 { return Err("Line must have at least two points"); }

        let color = self.color.get_frame(frame);
        let width = self.width.get_frame(frame);
        let start = self.start.get_frame(frame);
        let end = self.end.get_frame(frame);

        if start > end { return Err("Start is greater than End"); }

        let mut paint: skia_safe::Paint = skia_safe::Paint::default();
        paint.set_anti_alias(self.is_antialias);
        paint.set_stroke_cap(self.stroke_caps);
        paint.set_stroke_width(width);
        paint.set_color(color);

        let mut total_distance = 0.0;

        fn get_point_distance(point_1:Vector2D<f32>,point_2:Vector2D<f32>) -> f32{
            let x_diff = point_1.x - point_2.x;
            let y_diff = point_1.y - point_2.y;
            let x_squared = x_diff * x_diff;
            let y_squared = y_diff * y_diff;
            x_squared + y_squared
        }

        for i in 1..self.points.len() {
            let p1 = self.points[i-1].get_frame(frame);
            let p2 = self.points[i].get_frame(frame);
            total_distance += get_point_distance(p1,p2);
        }

        let mut p1_distance = 0.0;
        let mut p2_distance = 0.0;

        for i in 1..self.points.len() {
            let p1 = self.points[i-1].get_frame(frame);
            let p2 = self.points[i].get_frame(frame);

            let mut p1_x = p1.x;
            let mut p1_y = p1.y;
            let mut p2_x = p2.x;
            let mut p2_y = p2.y;

            let current_distance = get_point_distance(p1,p2) / total_distance;

            p2_distance += current_distance;

            if p1_distance > end {return Ok(())}

            if p1_distance < start
            {
                let d = (p2_distance - start) / current_distance;
                p1_x = p2.x + (p1.x - p2.x) * d;
                p1_y = p2.y + (p1.y - p2.y) * d;
            }

            if p2_distance > end {
                let d = (end - p1_distance) / current_distance;
                p2_x = p1.x + (p2.x - p1.x) * d;
                p2_y = p1.y + (p2.y - p1.y) * d;
            }

            p1_distance = p2_distance;
            let point_1 = Point::new(p1_x, p1_y);
            let point_2 = Point::new(p2_x, p2_y);
            canvas.draw_line(point_1,point_2,&paint);
        }
        Ok(())
    }
}