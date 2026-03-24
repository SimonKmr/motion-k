use vector2d::Vector2D;
use crate::motion_graphics::attributes::static_attribute::StaticAttribute;

pub trait Element {
    fn draw_on(&self, frame:usize, canvas: &skia_safe::canvas::Canvas) -> Result<(),&'static str>;
    fn boxed(self) -> Box<dyn Element>
    where Self: Sized + 'static,{
        Box::new(self)
    }
}