use skia_safe::RGB;
use vector2d::Vector2D;
use crate::attributes::static_attribute::StaticAttribute;

pub(crate) trait InterpolationArithmetics: Copy {
    fn subtract(self, other: &Self) -> Self;
    fn multiply(self, other:&f32 ) -> Self;
    fn add(self, other: &Self) -> Self;
    fn into_bsa(self) -> Box<StaticAttribute<Self>>;
}

impl InterpolationArithmetics for f32 {
    fn subtract(self, other: &Self) -> Self {
        self - other
    }

    fn multiply(self, other: &f32) -> Self {
        self * other
    }

    fn add(self, other: &Self) -> Self {
        self + other
    }

    fn into_bsa(self) -> Box<StaticAttribute<Self>> {
        Box::new(StaticAttribute::<Self>::new(self))
    }
}

impl InterpolationArithmetics for u8 {
    fn subtract(self, other: &Self) -> Self {
        self - other
    }

    fn multiply(self, other: &f32) -> Self {
        ((self as f32)* other) as u8
    }

    fn add(self, other: &Self) -> Self {
        self + other
    }

    fn into_bsa(self) -> Box<StaticAttribute<Self>>{
        Box::new(StaticAttribute::<Self>::new(self))
    }
}

impl InterpolationArithmetics for u32 {
    fn subtract(self, other: &Self) -> Self {
        self - other
    }

    fn multiply(self, other: &f32) -> Self {
        ((self as f32)* other) as u32
    }

    fn add(self, other: &Self) -> Self {
        self + other
    }

    fn into_bsa(self) -> Box<StaticAttribute<Self>>{
        Box::new(StaticAttribute::<Self>::new(self))
    }
}

impl InterpolationArithmetics for i32 {
    fn subtract(self, other: &Self) -> Self {
        self - other
    }

    fn multiply(self, other: &f32) -> Self {
        ((self as f32)* other) as i32
    }

    fn add(self, other: &Self) -> Self {
        self + other
    }

    fn into_bsa(self) -> Box<StaticAttribute<Self>>{
        Box::new(StaticAttribute::<Self>::new(self))
    }
}

impl InterpolationArithmetics for RGB{
    fn subtract(self, other: &Self) -> Self {
        let r = self.r - other.r;
        let g = self.g - other.g;
        let b = self.b - other.b;
        RGB { r, g, b }
    }

    fn multiply(self, other: &f32) -> Self {
        let r = ((self.r as f32) * other) as u8;
        let g = ((self.g as f32) * other) as u8;
        let b = ((self.b as f32) * other) as u8;
        RGB { r, g, b }
    }

    fn add(self, other: &Self) -> Self {
        let r = self.r + other.r;
        let g = self.g + other.g;
        let b = self.b + other.b;
        RGB { r, g, b }
    }

    fn into_bsa(self) -> Box<StaticAttribute<Self>>{
        Box::new(StaticAttribute::<Self>::new(self))
    }
}

impl InterpolationArithmetics for Vector2D<f32>{
    fn subtract(self, other: &Self) -> Self {
        let x = self.x - other.x;
        let y = self.y - other.y;
        Vector2D::new(x,y)
    }

    fn multiply(self, other: &f32) -> Self {
        let x = self.x * other;
        let y = self.y * other;
        Vector2D::new(x,y)
    }

    fn add(self, other: &Self) -> Self {
        let x = self.x + other.x;
        let y = self.y + other.y;
        Vector2D::new(x,y)
    }

    fn into_bsa(self) -> Box<StaticAttribute<Self>>{
        Box::new(StaticAttribute::<Self>::new(self))
    }
}

impl InterpolationArithmetics for Vector2D<u32>{
    fn subtract(self, other: &Self) -> Self {
        let x = self.x - other.x;
        let y = self.y - other.y;
        Vector2D::new(x,y)
    }

    fn multiply(self, other: &f32) -> Self {
        let x = (self.x as f32 * other) as u32;
        let y = (self.y  as f32 * other) as u32;
        Vector2D::new(x,y)
    }

    fn add(self, other: &Self) -> Self {
        let x = self.x + other.x;
        let y = self.y + other.y;
        Vector2D::new(x,y)
    }

    fn into_bsa(self) -> Box<StaticAttribute<Self>>{
        Box::new(StaticAttribute::<Self>::new(self))
    }
}