use crate::vec3::Vec3;

pub struct Viewport {
    pub width: i32,
    pub height: i32,
    pub du: f32,
    pub dv: f32,
    pub center: Vec3,
}
