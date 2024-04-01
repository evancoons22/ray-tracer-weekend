use crate::vec3::Vec3;
use crate::ray::Ray;
use crate::color::*;

pub struct Camera {
    // camera
    pub look_from: Vec3,
    pub look_at: Vec3,

    // viewport
    pub image_width: i32,
    pub image_height: i32,
    pub viewport_height: f32,
    pub viewport_width: f32,
    pub horizontal: Vec3,
    pub top_left_corner: Vec3,
    pub vertical: Vec3,
    pub aspect_ratio: f32,
    pub pixel00_loc: Vec3,

    // params
    pub max_depth: i32,
}

impl Camera {
    pub fn ray_color(ray: &Ray) -> ColorFloat { 
        return ColorFloat::new(0.0, 0.0, 0.0);
    }

    pub fn new(image_width: i32, aspect_ratio: f32) -> Camera {

        // calculate image_height and make sure it is > 1, else, set to 1
        let image_height = if aspect_ratio > 1.0 {
            (image_width as f32 / aspect_ratio) as i32
        } else {
            (image_width as f32 / aspect_ratio).ceil() as i32
        };

        // don't use aspect ratio to calculate viewport height and width
        // this is because we may have lost some precision when calculating image_height by rounding
        // want this to be as accurate as possible
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f32 / image_height as f32); 

        Camera {
            pixel00_loc: Vec3::new(-2.0, 1.0, 1.0),
            look_from: Vec3::new(0.0, 0.0, -1.0),
            look_at: Vec3::new(0.0, 0.0, 0.0),
            top_left_corner: Vec3::new(0.0, 0.0, 0.0),
            horizontal: Vec3::new(0.0, 0.0, 0.0),
            vertical: Vec3::new(0.0, 0.0, 0.0),
            image_width: 256,
            image_height,
            viewport_height,
            viewport_width,
            max_depth: 10,
            aspect_ratio: 16.0 / 9.0,
        }
    }

}


