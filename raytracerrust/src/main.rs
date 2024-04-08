use raytracerrust::camera::*;
use raytracerrust::helper::*;
use raytracerrust::hittable::*;
use raytracerrust::vec3::*;
use raytracerrust::color::*;
use raytracerrust::material::*;

fn main() { 
    let width = 400;
    let aspect_ratio = 16.0 / 9.0;

    let mut world = HittableList::new();

    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let material_left = Metal::new(Color::new(0.8, 0.8, 0.8), 0.15);
    // let material_left = Dielectric::new(1.0);
    // let material_bubble = Dielectric::new(1.0 / 1.5);
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 1.0);

    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, material_center)));
    world.add(Box::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left)));
    //world.add(Box::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.4, material_bubble)));
    world.add(Box::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_right)));

    //world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, Lambertian::new(Color::new(0.8, 0.3, 0.3)))));
    //world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, Lambertian::new(Color::new(0.8, 0.8, 0.0)))));

    let cam = Camera::new(width, aspect_ratio, world, 100);

    setup_ppm(&cam );
    cam.render_threaded();
}
