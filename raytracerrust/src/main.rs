
fn main() {
    let  image_width: i32 = 256;
    let  image_height: i32 = 256;

    print!("P3\n{:?} {:?}\n", image_width, image_height);
    print!("255\n");

    
    for i in 1..image_height { 
        for j in 1..image_width {
            let r: f32 = (i as f32) / ((image_width - 1) as f32);
            let g: f32 = (j as f32) / ((image_height - 1) as f32);
            let b: f32 = 0.0;

            let ir: i32 = r.round() as i32;
            let ig: i32 = g.round() as i32;
            let ib: i32 = b.round() as i32;

            print!("{:?} {:?} {:?}\n", ir, ig, ib);

        }
    }
}
