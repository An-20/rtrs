extern crate nalgebra;


type Vec3 = nalgebra::Vector3<f64>;
type Point3 = Vec3;
type Colour = Vec3;


fn colour_to_ppm_string(colour: &Colour) -> String {
    format!("{} {} {}\n", (255.999 * colour.x) as u32, (255.999 * colour.y) as u32, (255.999 * colour.z) as u32)
}


fn main() {
    let image_width: u32 = 256;
    let image_height: u32 = 256;

    print!("P3\n{} {}\n255\n", image_width, image_height);

    for i in 0..image_height {
        for j in 0..image_width {
            let pixel_colour: Colour = Colour::new(i as f64 / (image_width as f64 - 1.0), j as f64 / (image_height as f64 - 1.0), 0.0);  
            print!("{}", colour_to_ppm_string(&pixel_colour));
        }
    }
}
