pub trait Attribute<T> {
    fn get_frame(&self, frame: usize) -> T;
}