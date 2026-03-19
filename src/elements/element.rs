pub trait Element {
    fn draw_on(&self, frame:u32, canvas: &skia_safe::canvas::Canvas) -> Result<(),&'static str>;
}