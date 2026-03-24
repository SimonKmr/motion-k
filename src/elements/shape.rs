use skia_safe::{Canvas, Path, PathFillType, Point, RGB};
use vector2d::Vector2D;
use crate::attributes::attribute::Attribute;
use crate::elements::Element;

pub struct Shape {
    pub points: Vec<Box<dyn Attribute<Vector2D<f32>>>>,
    pub color: Box< dyn Attribute<RGB>>,
    pub is_antialias: bool,
}

impl Element for Shape{
    fn draw_on(&self, frame: usize, canvas: &Canvas) -> Result<(), &'static str> {

        if self.points.len() < 3 { return Err("Shape must have at least three points"); }

        //vec2d -> sk_point
        let mut sk_points : Vec<Point> = Vec::new();
        for vec2d in &self.points{
            let vec2d = vec2d.get_frame(frame);
            let sk_point = Point::new(vec2d.x, vec2d.y);
            sk_points.push(sk_point);
        }
        let sk_shape = Path::polygon(&sk_points,true,PathFillType::default(),false);

        //create color
        let color = self.color.get_frame(frame);
        let mut paint: skia_safe::Paint = skia_safe::Paint::default();
        paint.set_anti_alias(self.is_antialias);
        paint.set_color(color);

        //draws on canvas
        canvas.draw_path(&sk_shape,&paint);
        Ok(())
    }
}