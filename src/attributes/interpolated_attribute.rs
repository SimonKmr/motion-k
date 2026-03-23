use std::cell::Cell;
use crate::attributes::attribute::Attribute;
use crate::attributes::type_extensions::InterpolationArithmetics;

pub struct InterpolatedAttribute<T: InterpolationArithmetics>{
    keyframes: Vec<Box<Keyframe<T>>>,
}

pub struct Keyframe<T : InterpolationArithmetics>{
    value: T,
    frame: usize,
    transition: Box<dyn Fn(f32) -> f32> ,
}

impl<T: InterpolationArithmetics> InterpolatedAttribute<T>{
    pub fn new() -> Self{
        InterpolatedAttribute{
            keyframes : Vec::new(),
        }
    }

    pub fn add(&mut self, value: T, frame: usize){
        let x = Box::new(Keyframe::new(value,frame));
        self.keyframes.push(x);
    }

    pub fn add_t(&mut self, value: T, frame: usize){
        let x = Box::new(Keyframe::new(value,frame));
        self.keyframes.push(x);
    }

    pub fn boxed(self) -> Box<Self>{
        Box::new(self)
    }
}


impl<T:InterpolationArithmetics> Attribute<T> for InterpolatedAttribute<T> {
    fn get_frame(&self, frame: usize) -> T {
        let k = &self.keyframes;

        //no values -> error
        if k.len() == 0{ panic!("Empty keyframes"); }
        //one value -> value
        if k.len() == 1 { return k[0].value; }

        let mut i  = k.len() - 1;
        while k[i].frame > frame && i > 0 { i -= 1; }

        if k.len() - 1 == i { return k[k.len() -1 ].value; }

        let time_in = frame - k[i].frame;

        // if is on keyframe, return value of keyframe
        if time_in <= 0 { return k[i].value; }

        let p_value = k[i].value;
        let n_value = k[i + 1 ].value;
        let value_diff = n_value.subtract(&p_value);

        let time_in_percentage = time_in as f32 / (k[i+1].frame - k[i].frame) as f32;
        let value_mapped = (k[i].transition)(time_in_percentage);
        let result = k[i].value.add(&value_diff.multiply(&value_mapped));
        result
    }
}