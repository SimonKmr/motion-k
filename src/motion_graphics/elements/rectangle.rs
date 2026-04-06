use skia_safe::{Canvas, Path, PathFillType, Point, Rect, RGB};
use vector2d::Vector2D;
use crate::motion_graphics::attributes::attribute::Attribute;
use crate::motion_graphics::elements::Element;
use crate::motion_graphics::elements::element::DrawInfo;

pub struct Rectangle {
    pub position: Box<dyn Attribute<Vector2D<f32>>>,
    pub size: Box<dyn Attribute<Vector2D<f32>>>,
    pub color: Box< dyn Attribute<RGB>>,
    pub is_antialias: bool,
}

impl Element for Rectangle{
    fn draw_on(&self, frame: usize, canvas: &Canvas, draw_info: &DrawInfo) -> Result<(), &'static str> {

        let position = self.position.get_frame(frame);
        let size = self.size.get_frame(frame);

        //create color
        let color = self.color.get_frame(frame);
        let mut paint: skia_safe::Paint = skia_safe::Paint::default();
        paint.set_anti_alias(self.is_antialias);
        paint.set_color(color);

        //draws on canvas
        let shape = Rect::new(position.x, position.y, size.x, size.y);
        canvas.draw_rect(shape, &paint);
        Ok(())
    }
}