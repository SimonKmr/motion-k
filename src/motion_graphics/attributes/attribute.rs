pub trait Attribute<T>: CloneAttribute<T> {
    fn get_frame(&self, frame: usize) -> T;
}

trait CloneAttribute<T> {
    fn clone_box(&self) -> Box<dyn Attribute<T>>;
}

impl<A, T> CloneAttribute<T> for A
where
    A: Attribute<T> + Clone + 'static,
{
    fn clone_box(&self) -> Box<dyn Attribute<T>> {
        Box::new(self.clone())
    }
}

impl<T> Clone for Box<dyn Attribute<T>> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}