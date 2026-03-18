use crate::attributes::interpolated_attribute::InterpolatedAttribute;


pub(crate) trait InterpolationArithmetics: Copy {
    fn subtract(self, other: &Self) -> Self;
    fn multiply(self, other:&f32 ) -> Self;
    fn add(self, other: &Self) -> Self;
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
}