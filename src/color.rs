use std::ops;

#[derive(Clone,PartialEq,Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl ops::Add<Color> for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b
        }
    }
}

impl ops::Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        Color {
            r: (self * rhs.r as f64) as u8,
            g: (self * rhs.g as f64) as u8,
            b: (self * rhs.b as f64) as u8
        }
    }
}
