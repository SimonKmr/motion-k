pub trait Attribute<T> {
    fn get_frame(&self, frame: u32) -> T;
}