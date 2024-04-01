use std::fmt::Display;

pub struct Color<T> {
    pub r: T,
    pub g: T,
    pub b: T,
}

impl Display for Color<i32> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?} {:?} {:?}\n", self.r, self.g, self.b)
    }
}

impl Display for Color<f32> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?} {:?} {:?}\n", (self.r * 255.99) as i32, (self.g * 255.99) as i32, (self.b * 255.99) as i32)
    }
}


impl Color<f32> {
    pub fn new(r: f32, g: f32, b: f32) -> Color<f32> {
        Color { r, g, b }
    }
}


