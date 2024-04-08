use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::Vec3;

//pub trait Material {
//    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Color<f32>, Ray)>;
//    fn box_clone(&self) -> Box<dyn Material>;
//}

pub trait Material: Send + Sync {
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

pub struct Dielectric {
    pub ir: f32,
}
impl Dielectric {
    pub fn new(ir: f32) -> Dielectric {
        Dielectric { ir }
    }
}
impl Clone for Dielectric { 
    fn clone(&self) -> Dielectric {
        Dielectric { ir: self.ir }
    }
}
impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<(Color<f32>, Ray)> {
        let refraction_ratio = if rec.front_face { 1.0 / self.ir } else { self.ir };
        let unit_direction = ray.direction().unit_vector();
        let cos_theta = (-unit_direction).dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract || reflectance(cos_theta, refraction_ratio) > rand::random::<f32>() {
            unit_direction.reflect(rec.normal)
        } else {
            let etai_over_etat = if rec.front_face { self.ir } else { 1.0 / self.ir };
            unit_direction.refract(rec.normal, etai_over_etat)
        };
        let scattered = Ray::new(rec.p, direction);
        Some((Color::new(1.0, 1.0, 1.0), scattered))
    }
    fn box_clone(&self) -> Box<dyn Material> {
        Box::new(Dielectric { ir: self.ir })
    }
}

fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

impl Clone for Box<dyn Material> {
    fn clone(&self) -> Box<dyn Material> {
        self.as_ref().box_clone()
    }
}
