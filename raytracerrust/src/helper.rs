use crate::camera::Camera;

const PI: f32 = std::f32::consts::PI;

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

pub fn setup_ppm(camera: &Camera) { 
    print!("P3\n{:?} {:?}\n", camera.image_width, camera.image_height);
    print!("255\n");
}


pub fn to_int(f: f32) -> i32 { 
    f.round() as i32
}

pub struct Interval {
    pub t_min: f32,
    pub t_max: f32,
}

impl Interval { 
    pub fn new(t_min: f32, t_max: f32) -> Interval { 
        Interval { t_min, t_max }
    }

    //fn contains(&self, t: f32) -> bool { 
    //    t >= self.t_min && t <= self.t_max
    //}
    //

    pub fn clamp(&self, t: f32) -> f32 { 
        if t < self.t_min { 
            t
        } else if t > self.t_max { 
            t
        } else { 
            self.t_max
        }
    }

    pub fn surrounds(&self, x: f32) -> bool { 
        x >= self.t_min && x <= self.t_max
    }

    //fn union(&self, other: Interval) -> Interval { 
    //    Interval { t_min: self.t_min.min(other.t_min), t_max: self.t_max.max(other.t_max) }
    //}
} 

// const intervals empty and universe
pub const EMPTY: Interval = Interval { t_min: std::f32::INFINITY, t_max: -std::f32::INFINITY };
pub const UNIVERSE: Interval = Interval { t_min: -std::f32::INFINITY, t_max: std::f32::INFINITY };
