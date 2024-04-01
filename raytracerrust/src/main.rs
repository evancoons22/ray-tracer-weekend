use raytracerrust::camera::*;
use raytracerrust::helper::*;

fn main() { 
    let width = 400;
    let aspect_ratio = 16.0 / 9.0;
    let cam = Camera::new(width, aspect_ratio);

    setup_ppm(&cam);
    cam.display();
}
