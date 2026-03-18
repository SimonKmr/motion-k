use crate::attributes::attribute::Attribute;

pub struct StaticAttribute<T:Copy>{
    value: T,
}

impl<T:Copy> StaticAttribute<T>{
    pub fn new(value: T) -> StaticAttribute<T>{
        StaticAttribute{
            value,
        }
    }
}

impl<T:Copy> Attribute<T> for StaticAttribute<T> {
    fn get_frame(&self, frame: u32) -> T {
        self.value
    }
}