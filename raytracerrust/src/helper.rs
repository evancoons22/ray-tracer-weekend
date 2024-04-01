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
