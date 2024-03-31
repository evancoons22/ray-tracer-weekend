use crate::vec3::Vec3;

pub struct Camera {
    pub look_from: Vec3,
    pub look_at: Vec3,
    pub top_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub image_width: i32,
    pub image_height: i32,
    pub max_depth: i32,
    pub aspect_ratio: f32,
    pub pixel00_loc: Vec3,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            pixel00_loc: Vec3::new(-2.0, 1.0, 1.0),
            look_from: Vec3::new(0.0, 0.0, -1.0),
            look_at: Vec3::new(0.0, 0.0, 0.0),
            top_left_corner: Vec3::new(0.0, 0.0, 0.0),
            horizontal: Vec3::new(0.0, 0.0, 0.0),
            vertical: Vec3::new(0.0, 0.0, 0.0),
            image_width: 256,
            image_height: 256,
            max_depth: 10,
            aspect_ratio: 16.0 / 9.0,
        }
    }

    pub fn initialize() -> Camera {
        Camera {
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
        }
    }
}


