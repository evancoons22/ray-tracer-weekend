use std::fmt::Display;
use std::ops::Mul;
use std::ops::Add;

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

impl Mul<f32> for Color<f32> {
    type Output = Color<f32>;

    fn mul(self, t: f32) -> Color<f32> {
        Color {
            r: self.r * t,
            g: self.g * t,
            b: self.b * t,
        }
    }
}

impl Add for Color<f32> {
    type Output = Color<f32>;

    fn add(self, other: Color<f32>) -> Color<f32> {
        Color {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl Color<f32> { 
    pub fn scale_color(self, samples_per_pixel: f32) -> Color<f32> { 
        Color  {
           r: self.r / samples_per_pixel,
           g: self.g / samples_per_pixel,
           b: self.b / samples_per_pixel,
        }
    }
}


impl Display for Color<f32> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // divide the color by the number of samples
        write!(f, "{:?} {:?} {:?}\n", (self.r * 255.99) as i32, (self.g * 255.99) as i32, (self.b * 255.99) as i32)
    }
}


impl Color<f32> {
    pub fn new(r: f32, g: f32, b: f32) -> Color<f32> {
        Color { r, g, b }
    }
}


