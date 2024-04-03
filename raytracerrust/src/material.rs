use crate::color::Color;
use crate::hittable::HitRecord;
use crate::vec3::Vec3;
use crate::ray::Ray;


pub trait Material {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Color<f32>, Ray)>;
}

pub struct Metal {
    pub albedo: Color<f32>,
    pub fuzz: f32,
}
impl Metal {
    pub fn new(albedo: Color<f32>, fuzz: f32) -> Metal {
        Metal { albedo, fuzz }
    }
}
impl Material for Metal {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Color<f32>, Ray)> {
        let reflected = ray.direction().unit_vector().reflect(rec.normal);
        let scattered = Ray::new(rec.p, reflected + Vec3::random_in_unit_sphere());
        if scattered.direction().dot(rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

pub struct Lambertian {
    pub albedo: Color<f32>,
}
impl Lambertian {
    pub fn new(albedo: Color<f32>) -> Lambertian {
        Lambertian { albedo }
    }
}
impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Color<f32>, Ray)> {
        let scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        let scattered = Ray::new(rec.p, scatter_direction);
        Some((self.albedo, scattered))
    }
}
