use skia_safe::{Canvas, Path, PathFillType, Point, RGB};
use vector2d::Vector2D;
use crate::motion_graphics::attributes::attribute::Attribute;
use crate::motion_graphics::elements::Element;
use crate::motion_graphics::elements::element::DrawInfo;

pub struct Shape {
    pub position_offset: Box<dyn Attribute<Vector2D<f32>>>,
    pub points: Vec<Box<dyn Attribute<Vector2D<f32>>>>,
    pub color: Box< dyn Attribute<RGB>>,
    pub is_antialias: bool,
}

impl Element for Shape{
    fn draw_on(&self, frame: usize, canvas: &Canvas, draw_info: &DrawInfo) -> Result<(), &'static str> {

        if self.points.len() < 3 { return Err("Shape must have at least three points"); }

        let position_offset = self.position_offset.get_frame(frame);
        //vec2d -> sk_point
        let mut sk_points : Vec<Point> = Vec::new();
        for vec2d in self.points.iter(){
            let vec2d = vec2d.get_frame(frame);
            let x = vec2d.x + position_offset.x;
            let y = vec2d.y + position_offset.y;
            let sk_point = Point::new(x, y);
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