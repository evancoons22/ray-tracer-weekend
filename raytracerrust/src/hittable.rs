use crate::vec3::*;
use crate::ray::*;
use crate::helper::*;
use crate::material::*;

pub struct HitRecord { 
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub material: Option<Box<dyn Material>>,
} 

pub trait Hittable { 
    fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
    fn set_face_normal(&self, ray: &Ray, outward_normal: Vec3, rec: &mut HitRecord) { 
        rec.front_face = ray.direction().dot(outward_normal) < 0.0;
        rec.normal = if rec.front_face { outward_normal } else { -outward_normal };
    }
}

pub struct HittableList { 
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList { 
    pub fn new() -> HittableList { 
        HittableList { objects: Vec::new() }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) { 
        self.objects.push(object);
    }
}

impl Hittable for HittableList { 
    fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool { 
        let mut temp_rec = HitRecord { 
            p: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
            material: None,
        };
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.t_max;
        for object in self.objects.iter() { 
            if object.hit(ray, Interval::new(ray_t.t_min, closest_so_far), &mut temp_rec) { 
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec.p = temp_rec.p;
                rec.normal = temp_rec.normal;
                rec.t = temp_rec.t;
                rec.front_face = temp_rec.front_face;
            }
        }
        hit_anything
    }
}

pub struct Sphere { 
    pub center: Point3,
    pub radius: f32,
    material: Option<Box<dyn Material>>,
}

impl Sphere { 
    pub fn new<T: Material>(center: Point3, radius: f32, material: T) -> Sphere { 
        Sphere { center, radius, material: Some(Box::new(material))}
    }
}

impl Hittable for Sphere { 
    fn hit(&self, ray: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool { 
        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = oc.dot(ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();
        // check the smaller root first
        let mut root = (-half_b - sqrtd) / a;
        if !ray_t.surrounds(root) { 
            root = (-half_b + sqrtd) / a;
            if !ray_t.surrounds(root) { 
                return false;
            }
        }
        rec.t = root;
        rec.p = ray.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        self.set_face_normal(ray, outward_normal, rec);
        true
    }
}
