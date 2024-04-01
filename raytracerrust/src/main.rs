use raytracerrust::camera::*;
use raytracerrust::helper::*;
use raytracerrust::color::*;

fn main() { 
    let width = 256;
    let aspect_ratio = 16.0 / 9.0;
    let cam = Camera::new(width, aspect_ratio);

    setup_ppm(&cam);
    fill_background_color(cam.image_width, cam.image_height, ColorFloat::new(0.5, 0.7, 1.0));
}
