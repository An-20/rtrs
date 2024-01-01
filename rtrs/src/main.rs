mod ray;
mod types;
mod hittable;

use ray::Ray;
use types::{Vec3, Point3, Colour};


fn unit_vector(vec: &Vec3) -> Vec3 {
    vec.clone() / vec.norm()
}


fn hit_sphere(center: Point3, radius: f64, ray: &Ray) -> f64 {
    let oc: Vec3 = ray.origin() - center;
    let a = ray.direction().norm_squared();
    let half_b = oc.dot(ray.direction());
    let c = oc.norm_squared() - radius * radius;
    let discriminant: f64 = half_b * half_b - a*c;
    
    if discriminant < 0.0 {
        -1.0
    }
    else {
        (-half_b - discriminant.sqrt()) / a
    }
}


fn ray_colour(ray: &Ray) -> Colour {
    let t = hit_sphere(Point3::new(0.0,0.0,-1.0), 0.5, ray);
    if t > 0.0 {
        let n: Vec3 = unit_vector(&(ray.at(t) - Vec3::new(0.0, 0.0, -1.0)));
        return 0.5 * Colour::new(n.x + 1.0, n.y + 1.0, n.z + 1.0);
    }

    let unit_direction: Vec3 = ray.direction().clone() / ray.direction().norm();
    let a = 0.5 * (unit_direction.y + 1.0);
    return (1.0-a)*Colour::new(1.0, 1.0, 1.0) + a*Colour::new(0.5, 0.7, 1.0)
}


fn colour_to_ppm_string(colour: &Colour) -> String {
    format!("{} {} {}\n", (255.999 * colour.x) as u32, (255.999 * colour.y) as u32, (255.999 * colour.z) as u32)
}


fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 2000;
    const IMAGE_HEIGHT: u32 = if IMAGE_WIDTH as f64 / ASPECT_RATIO > 1.0 {
        (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32
    } else { 1 };

    const FOCAL_LENGTH: f64 = 1.0;
    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * (IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64);
    const CAMERA_CENTRE: Point3 = Point3::new(0.0, 0.0, 0.0);

    let viewport_u: Vec3 = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let viewport_v: Vec3 = Vec3::new(0.0, -VIEWPORT_HEIGHT, 0.0);

    let pixel_delta_u: Vec3 = viewport_u / IMAGE_WIDTH as f64;
    let pixel_delta_v: Vec3 = viewport_v / IMAGE_HEIGHT as f64;

    let viewport_upper_left: Vec3 = CAMERA_CENTRE - Vec3::new(0.0, 0.0, FOCAL_LENGTH) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc: Vec3 = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in 0..IMAGE_HEIGHT {
        for i in 0..IMAGE_WIDTH {
            let pixel_centre: Vec3 = pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_centre - CAMERA_CENTRE;

            let ray: Ray = Ray::new(CAMERA_CENTRE.clone(), ray_direction.clone());

            let pixel_colour: Colour = ray_colour(&ray);
            print!("{}", colour_to_ppm_string(&pixel_colour));
        }
    }
}
