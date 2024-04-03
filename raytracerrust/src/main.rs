use raytracerrust::camera::*;
use raytracerrust::helper::*;
use raytracerrust::hittable::*;
use raytracerrust::vec3::*;

fn main() { 
    let width = 400;
    let aspect_ratio = 16.0 / 9.0;

    let mut world = HittableList::new();

    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let cam = Camera::new(width, aspect_ratio, world, 100);

    setup_ppm(&cam );
    cam.display();
}
