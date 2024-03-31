use raytracerrust::vec3::*;
use raytracerrust::camera::*;
use raytracerrust::helper::*;
use raytracerrust::color::*;

fn main() { 
    let cam = Camera {
        pixel00_loc: Vec3::new(-2.0, 1.0, 1.0),
        look_from: Vec3::new(0.0, 0.0, -1.0),
        look_at: Vec3::new(0.0, 0.0, 0.0),
        top_left_corner: Vec3::new(-2.0, 1.0, 1.0),
        horizontal: Vec3::new(4.0, 0.0, 0.0),
        vertical: Vec3::new(0.0, -2.0, 0.0),
        image_width: 256,
        image_height: 256,
        max_depth: 10,
        aspect_ratio: 16.0 / 9.0,
    };
    // setup
    setup_ppm(&cam);
    fill_background_color(cam.image_width, cam.image_height, Color::new(150, 52, 235));
}
