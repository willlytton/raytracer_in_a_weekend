use ::std::f64;
use std::io::{self, Write};

pub mod color;
mod vec3;

fn write_ppm(w: i32, h: i32, max_color: i32) {
    // function that loops through P3 color values from 0px to 256px and prints the value to screen
    println!("{} {} {}", w, h, max_color);

    for j in (0..=h).rev() {
        eprintln!("\rScanline remaining: {}", j); // TODO: need to figure out a way to add steady progression to loop with flush
        for i in 0..w {
            let r: f32 = i as f32 / w as f32;
            let g: f32 = j as f32 / h as f32;
            let b: f32 = 0.25;
            color::pixel_color(i as f32 / w - 1.0 as f32, j as f32 / h - 1.0 as f32, 0.25);
            color::write_color(out, pixel_color);
        }
    }
}
fn main() {
    let image_width: i32 = 256;
    let image_height: i32 = 256;
    let max_color: i32 = 255;

    write_ppm(image_width, image_height, max_color);
}
