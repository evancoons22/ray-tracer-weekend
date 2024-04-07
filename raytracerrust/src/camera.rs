use crate::vec3::*;
use crate::hittable::*;
use crate::helper::*;
use crate::ray::Ray;
use crate::color::*;

use std::sync::mpsc; // multiprocessor, single consumer
use rayon::prelude::*;

use rand::Rng;
use std::io::Write;
use std::f32::INFINITY;

pub struct Camera {
    // camera
    pub center: Point3,

    // viewport
    pub image_width: i32,
    pub image_height: i32,
    pub focal_length: f32,
    pub viewport_height: f32,
    pub viewport_width: f32,
    pub viewport_u: Vec3,
    pub viewport_v: Vec3,
    pub du: Vec3,
    pub dv: Vec3,
    pub aspect_ratio: f32,
    pub viewport_upper_left_corner: Vec3,
    pub pixel00_loc: Vec3,

    // params
    pub max_depth: i32,
    pub samples_per_pixel: i32,

    // world
    pub world: HittableList,
}


impl Camera {
    pub fn ray_color(ray: &Ray, world: &HittableList, max_depth: i32) -> Color<f32> { 
        // mutable hit record
        let mut rec = HitRecord {
            p: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
            material: None,
        };

        if max_depth <= 0 {
            return Color::new(1.0, 1.0, 1.0);
        }

        if world.hit(ray, Interval::new(0.001, INFINITY), &mut rec) {
           let material = rec.material.as_ref().unwrap();
           if let Some((attenuation, scattered)) = material.scatter(ray, &rec) {
               return attenuation * Camera::ray_color(&scattered, world, max_depth - 1);
           } else { 
               Color::new(0.0, 0.0, 0.0)
           } 

        } else {
            let unit_direction = ray.direction().unit_vector();
            let t = 0.5 * (unit_direction.y() + 1.0);
            return Color::new(1.0, 1.0, 1.0) * (1.0 -t)  + Color::new(0.5, 0.7, 1.0) * t;
        
        }

    }

    fn get_ray(&self, i: usize, j: usize) -> Ray { 
        let pixel_center = self.pixel00_loc + self.du * (i as f32) + self.dv * (j as f32);
        let pixel_sample = pixel_center + Camera::pixel_sample_square(&self);

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        return Ray::new(ray_origin, ray_direction);
        } 

    fn pixel_sample_square(&self) -> Vec3 { 
        let mut rng = rand::thread_rng();
        let x = self.du * rng.gen_range(-0.5..0.5);
        let y = self.dv * rng.gen_range(-0.5..0.5);
        x + y
    }

    pub fn compute_color(&self, i: usize, j: usize) -> Color<f32> { 
        let mut color = Color::new(0.0, 0.0, 0.0);
        for _ in 1..self.samples_per_pixel {
            let ray = self.get_ray(i, j);
            color = color + Camera::ray_color(&ray, &self.world, self.max_depth);
        }
        color = color.scale_color(self.samples_per_pixel as f32);
        color = color.linear_to_gamma();
        color
    }

    pub fn render_threaded(&self) { 
        let (tx, rx) = mpsc::channel();

        eprint!("\rBeginning Multithreaded render");
        std::io::stderr().flush().unwrap();
        (0..self.image_height).into_par_iter().for_each_with(tx, |tx, j| {
            for i in 0..self.image_width {
                let color = self.compute_color(i as usize, j as usize); // Assume this is a function that does the computation.
                tx.send((j, i, color)).unwrap();
            }
        });

        let mut results = Vec::new();
        for _ in 0..self.image_height*self.image_width {
            results.push(rx.recv().unwrap());
        }
        results.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));

        for (_, _, color) in results {
            print!("{}", color); // Your printing logic here.
        }
        eprintln!("\nDone.");
    }

    
    

    pub fn render(&self) { 
        for j in 0..self.image_height { 
            eprint!("\rScanlines remaining: {}", self.image_height - j);
            std::io::stderr().flush().unwrap();
            for i in 0..self.image_width {
                let mut color = Color::new(0.0, 0.0, 0.0);
                for _ in 1..self.samples_per_pixel {
                    let ray = self.get_ray(i as usize, j as usize);
                    color = color + Camera::ray_color(&ray, &self.world, self.max_depth);
                }
                //color = color + Camera::ray_color(&self.get_ray(i as usize, j as usize), &self.world, self.max_depth);
                color = color.scale_color(self.samples_per_pixel as f32);
                color = color.linear_to_gamma();
                print!("{}", color);
            }
        }
        eprintln!("\nDone.");
    }

    pub fn new(image_width: i32, aspect_ratio: f32, world: HittableList, samples_per_pixel: i32) -> Camera {

        // calculate image_height and make sure it is > 1, else, set to 1
        let image_height = if aspect_ratio > 1.0 {
            (image_width as f32 / aspect_ratio) as i32
        } else {
            (image_width as f32 / aspect_ratio).ceil() as i32
        };

        // don't use aspect ratio to calculate viewport height and width
        // this is because we may have lost some precision when calculating image_height by rounding
        // want this to be as accurate as possible
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f32 / image_height as f32); 

        let camera_center = Point3::new(0.0, 0.0, 0.0);
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        let du = viewport_u / (image_width as f32);
        let dv = viewport_v / (image_height as f32);

        let viewport_upper_left_corner = camera_center - Vec3::new(0.0, 0.0, focal_length) -  viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left_corner + du / 2.0 + dv / 2.0;



        Camera {
            center: camera_center,
            image_width,
            image_height,
            focal_length,
            viewport_height,
            viewport_width,
            viewport_u,
            viewport_v,
            du,
            dv,
            aspect_ratio,
            viewport_upper_left_corner,
            pixel00_loc,
            max_depth: 50,
            samples_per_pixel,
            world,
        }
    }

}


