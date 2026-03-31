use crate::geo::map_generator::{Map, MapIO};
use crate::geo::style::MapStyleSettings;
use crate::ui::window::Window;
use motion_graphics::attributes::attribute::Attribute;
use motion_graphics::attributes::interpolated_attribute::InterpolatedAttribute;
use motion_graphics::attributes::type_extensions::InterpolationArithmetics;
use motion_graphics::elements::Element;
use motion_graphics::{elements, sequence};
use skia_safe::RGB;
use std::str::FromStr;
use std::time::SystemTime;
use vector2d::Vector2D;

mod ui;
mod motion_graphics;
mod geo;

fn main() {
    let width = 1280_u32;
    let height = 720_u32;
    println!("Started");
    let start_time = SystemTime::now();

    let mut sequence = sequence::Sequence::new(width as usize,height as usize);
    //let line = elements::line::Line::new().boxed();

    let mut l_vec: Vec<Box<dyn Attribute<Vector2D<f32>>>> = Vec::new();
    l_vec.push(Vector2D::new(10., 10.).into_bsa());
    l_vec.push(Vector2D::new(1000., 700.).into_bsa());
    l_vec.push(Vector2D::new(300., 300.).into_bsa());
    l_vec.push(Vector2D::new(80., 700.).into_bsa());

    let mut end = InterpolatedAttribute::new();
    end.add(0.0,0_usize);
    end.add(1.0,600_usize);

    let mut s_vec: Vec<Box<dyn Attribute<Vector2D<f32>>>> = Vec::new();
    s_vec.push(Vector2D::new(300., 300.).into_bsa());
    s_vec.push(Vector2D::new(700., 700.).into_bsa());
    s_vec.push(Vector2D::new(900., 600.).into_bsa());
    s_vec.push(Vector2D::new(80., 200.).into_bsa());

    let map_data = crate::geo::map_generator::MapIO::load(
        &String::from("osm-data\\arnsberg-regbez-260324.osm.pbf"),
        None
    );
    //MapIO::export_binary(String::from_str("test.bin").unwrap(),&map_data);
    //println!("Exported bin");

    //let map_data= MapIO::import_binary(String::from_str("test.bin").unwrap());

    let mut map_scale = InterpolatedAttribute::new();
    map_scale.add(8f32,0_usize);
    map_scale.add(8.1f32,100_usize);

    let map = Map{
        position: Vector2D::new(640f32, 360f32).into_bsa(),
        geo_position: Vector2D::new(51.41484f32, 8.39553f32).into_bsa(),
        scale: map_scale.boxed(),
        data: map_data,
        settings: MapStyleSettings::default(),
    };

    sequence.push(Box::new(map));

    let mut current_frame = 0;

    let mut window = Window::new(width, height);

    window.redraw_event.push(Box::new(move |buffer|{
        let mut bytes = sequence.render_frame(current_frame);
        for (i,byte_chunk) in bytes.chunks_exact_mut(4).enumerate() {
            //rgba -> bgra

            buffer[i*4+0] = byte_chunk[2]; //red -> blue
            buffer[i*4+1] = byte_chunk[1]; //green
            buffer[i*4+2] = byte_chunk[0]; //blue -> red
            buffer[i*4+3] = byte_chunk[3]; //alpha
        }

        current_frame += 1;
    }));

    window.run();

    //let bytes = sequence.render_frame(current_frame);
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


