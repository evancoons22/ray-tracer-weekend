pub struct Color {
    pub r: i32,
    pub g: i32,
    pub b: i32,
}

impl Color { 
    //color from i32
    pub fn new(r: i32, g: i32, b: i32) -> Color { 
        Color { r, g, b }
    }
}

pub fn write_color(color: &Color) { 
    print!("{:?} {:?} {:?}\n", color.r, color.g, color.b);
}


