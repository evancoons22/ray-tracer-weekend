use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub trait Material {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Color<f32>, Ray)>;
    fn box_clone(&self) -> Box<dyn Material>;
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

impl Clone for Metal { 
    fn clone(&self) -> Metal {
        Metal { albedo: self.albedo, fuzz: self.fuzz }
    }
} 
impl Material for Metal {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Color<f32>, Ray)> {
        let reflected = ray.direction().unit_vector().reflect(rec.normal);
        let scattered = Ray::new(rec.p, reflected + self.fuzz * Vec3::random_in_unit_sphere());
        if scattered.direction().dot(rec.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
    fn box_clone(&self) -> Box<dyn Material> {
        Box::new(Metal { albedo: self.albedo, fuzz: self.fuzz })
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

impl Clone for Lambertian { 
    fn clone(&self) -> Lambertian {
        Lambertian { albedo: self.albedo}
    }
} 

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, rec: &HitRecord) -> Option<(Color<f32>, Ray)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        let scattered = Ray::new(rec.p, scatter_direction);
        Some((self.albedo, scattered))
    }
    fn box_clone(&self) -> Box<dyn Material> {
        Box::new(Lambertian { albedo: self.albedo })
    }
}

impl Clone for Box<dyn Material> {
    fn clone(&self) -> Box<dyn Material> {
        self.as_ref().box_clone()
    }
}
