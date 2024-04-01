use crate::camera::Camera;
use crate::color::*;
use std::fmt::Display;

pub fn setup_ppm(camera: &Camera) { 
    print!("P3\n{:?} {:?}\n", camera.image_width, camera.image_height);
    print!("255\n");
}

pub fn fill_background_color<T: Display>(image_width: i32, image_height: i32, color: Color<T>) { 
    for _ in 0..image_width {
        for _ in 0..image_height { 
            print!("{}", color);
        }
    }
}


pub fn fill_gradient(image_width: i32, image_height: i32) { 
    for j in 0..image_width {
        for i in 0..image_height { 
           let r: f32 = (i as f32) / ((image_width - 1) as f32) * 255.99;
           let g: f32 = (j as f32) / ((image_height - 1) as f32) * 255.99;
           let b: f32 = 0.0;

           let ir: i32 = to_int(r);
           let ig: i32 = to_int(g);
           let ib: i32 = to_int(b);

           print!("{:?} {:?} {:?}\n", ir, ig, ib);

        }
    }
}

pub fn to_int(f: f32) -> i32 { 
    f.round() as i32
}
