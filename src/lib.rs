mod color;
mod ray;
mod vec;

pub type Point3 = Vec3;

use color::Color;
use ray::Ray;
use vec::Vec3;

pub fn run(image_width: u32) {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::origin();
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..=image_height - 1).rev() {
        eprintln!("\rScanlines remaining: {}", j);
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let r = Ray {
                origin,
                direction: lower_left_corner + u * horizontal + v * vertical - origin,
            };
            print!("{}", r.ray_color());
        }
    }

    eprintln!("\nDone.");
}
