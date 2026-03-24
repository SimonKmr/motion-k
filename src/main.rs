use crate::attributes::attribute::Attribute;
use crate::attributes::interpolated_attribute::InterpolatedAttribute;
use crate::attributes::type_extensions::InterpolationArithmetics;
use crate::elements::Element;
use skia_safe::RGB;
use std::time::SystemTime;
use vector2d::Vector2D;
use crate::ui::window::Window;

mod elements;
mod sequence;
mod attributes;
mod ui;

fn main() {
    let width = 1280_u32;
    let height = 720_u32;
    println!("Started");
    let start_time = SystemTime::now();

    let mut sequence = sequence::Sequence::new(width as usize,height as usize);
    //let line = elements::line::Line::new().boxed();

    let mut l2_vec : Vec<Box<dyn Attribute<Vector2D<f32>>>> = Vec::new();
    l2_vec.push(Vector2D::new(10.,10.).into_bsa());
    l2_vec.push(Vector2D::new(1000.,900.).into_bsa());
    l2_vec.push(Vector2D::new(300.,300.).into_bsa());
    l2_vec.push(Vector2D::new(80.,700.).into_bsa());


    let mut end = InterpolatedAttribute::new();
    end.add(0.0,0_usize);
    end.add(1.0,600_usize);

    let line2 = elements::line::Line
    {
        points: l2_vec,
        start: 0.0_f32.into_bsa(),
        end: end.boxed(),
        width: 50_f32.into_bsa(),
        color: RGB{ r: 200, g: 200, b:200 }.into_bsa(),
        is_antialias: true,
        stroke_caps: skia_safe::paint::Cap::Round
    };
    sequence.push(line2.boxed());

    let current_frame = 0;

    let window = Window::new(1280,720);
    window.run();

    let bytes = sequence.render_frame(current_frame);
    //let frame = pixels.frame_mut();
    //for i in 0..bytes.len(){
    //    frame[i] = bytes[i];
    //}
    // write to frame buffer
    //pixels.render().unwrap();
    //current_frame += 1;

    //if current_frame > 600 {
    //    elwt.exit();
    //}



    let end_time = SystemTime::now();
    let duration_time = end_time.duration_since(start_time).unwrap().as_secs();
    println!("Done! {}",duration_time)

}


