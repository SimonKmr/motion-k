use skia_safe::PaintStyle::Stroke;
use skia_safe::RGB;
use vector2d::Vector2D;
use crate::attributes::type_extensions::InterpolationArithmetics;
use crate::attributes::{attribute, static_attribute};
use crate::elements::Element;
use crate::attributes::attribute::Attribute;
use crate::attributes::static_attribute::StaticAttribute;

mod elements;
mod sequence;
mod attributes;

fn main() {
    println!("Hello, world!");
    let mut sequence = sequence::Sequence::new(1080,1080);
    //let line = elements::line::Line::new().boxed();

    let mut l2_vec : Vec<Box<dyn Attribute<Vector2D<f32>>>> = Vec::new();
    l2_vec.push(Vector2D::new(10_f32,10_f32).into_bsa());
    l2_vec.push(Vector2D::new(1000_f32,900_f32).into_bsa());
    l2_vec.push(Vector2D::new(300_f32,300_f32).into_bsa());
    l2_vec.push(Vector2D::new(80_f32,700_f32).into_bsa());

    let line2 = elements::line::Line
    {
        points: l2_vec,
        start: 0.0_f32.into_bsa(),
        end: 1.0_f32.into_bsa(),
        width: 50_f32.into_bsa(),
        color: RGB{ r: 200, g: 200, b:200 }.into_bsa(),
        is_antialias: true,
        stroke_caps: skia_safe::paint::Cap::Round
    };
    sequence.push(line2.boxed());

    let bytes = sequence.render_frame(100);
    println!("{:?}", bytes);

}
