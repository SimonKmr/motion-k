use std::cell::Cell;
use crate::attributes::attribute::Attribute;
use crate::attributes::type_extensions::InterpolationArithmetics;

pub struct InterpolatedAttribute<T: InterpolationArithmetics>{
    keyframes: Vec<Box<Keyframe<T>>>,
    info: CurrentFrameInfo
}

pub struct Keyframe<T : InterpolationArithmetics>{
    value: T,
    frame: u32,
    transition: dyn Fn(f32) -> f32,
}

struct CurrentFrameInfo{
    index : Cell<usize> ,
    diff : Cell<u32>,
    diff_percentage: Cell<f32>,
}


impl<T:InterpolationArithmetics> Attribute<T> for InterpolatedAttribute<T> {
    fn get_frame(&self, frame: u32) -> T {
        let k = &self.keyframes;

        //no values -> error
        if k.len() == 0{ panic!("Empty frame"); }
        //one value -> value
        if k.len() == 1 { return k[0].value; }

        let mut i  = (k.len() - 1);
        while k[i].frame > frame && i > 0 { i -= 1; }

        //
        if k.len() - 1 == i { return k[k.len() -1 ].value; }

        if (i != self.info.index.get()){
            self.info.index.set(i);
            self.info.diff.set(k[i+1].frame - k[i].frame);
            self.info.diff_percentage.set(1. / (self.info.diff.get() as f32))
        }

        let time_in = frame - k[i].frame;

        // if is on keyframe, return value of keyframe
        if time_in <= 0 { return k[i].value; }

        let time_in_percentage = time_in as f32 / self.info.diff_percentage.get();
        let value_diff = k[i + 1 ].value.subtract(&k[i].value);
        let value_mapped = (k[i].transition)(time_in_percentage);
        let result = k[i].value.add(&value_diff.multiply(&value_mapped));
        result
    }
}