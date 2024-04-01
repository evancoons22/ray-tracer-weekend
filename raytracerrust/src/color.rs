pub struct Color {
    pub r: i32,
    pub g: i32,
    pub b: i32,
}

pub struct ColorFloat {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl From<Color> for ColorFloat {
    fn from(color: Color) -> Self {
        ColorFloat {
            r: color.r as f32 / 255.99,
            g: color.g as f32 / 255.99,
            b: color.b as f32 / 255.99,
        }
    }
}

impl Color  { 
    pub fn write_color(&self) {
        print!("{:?} {:?} {:?}\n", self.r, self.g, self.b);
    }
    pub fn new(r: i32, g: i32, b: i32) -> Color { 
        Color { r, g, b }
    }
}

impl ColorFloat  { 
    pub fn write_color(&self) {
        let ir = (self.r * 255.99) as i32;
        let ig = (self.g * 255.99) as i32;
        let ib = (self.b * 255.99) as i32;
        print!("{:?} {:?} {:?}\n", ir, ig, ib);
    }
    pub fn new(r: f32, g: f32, b: f32) -> ColorFloat { 
        ColorFloat { r, g, b }
    }
}



