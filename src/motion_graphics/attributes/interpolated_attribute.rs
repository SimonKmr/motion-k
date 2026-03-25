use crate::motion_graphics::attributes::attribute::Attribute;
use crate::motion_graphics::attributes::type_extensions::InterpolationArithmetics;
#[derive(Clone)]
pub struct InterpolatedAttribute<T: InterpolationArithmetics>{
    keyframes: Vec<Box<Keyframe<T>>>,
}

#[derive(Clone)]
pub struct Keyframe<T : InterpolationArithmetics + Clone>{
    value: T,
    frame: usize,
    transition: fn(f32) -> f32,
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

impl<T : InterpolationArithmetics> Keyframe<T>{
    pub fn new(value: T, frame: usize) -> Self{
        Keyframe{
            value,
            frame,
            transition: |x| x
        }
    }

    ///
    /// Creates a new keyframe with a transition
    ///
    /// This function allows you to create a Keyframe and specify every field,
    /// including a function to map the linear progression of the keyframe onto a non-linear progression.
    ///
    /// ```rust
    /// let x = Keyframe::new(0.1,42_usize);
    /// ```
    ///
    pub fn new_t(value: T, frame: usize, transition: fn(f32) -> f32) -> Self{
        Keyframe{
            value,
            frame,
            transition
        }
    }

    pub fn boxed(self) -> Box<Self>{
        Box::new(self)
    }
}

impl<T:InterpolationArithmetics + 'static> Attribute<T> for InterpolatedAttribute<T> {
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