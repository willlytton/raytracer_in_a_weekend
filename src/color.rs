use std::io::Write;
use glam::Vec3;



pub type Color = Vec3::new();

pub fn write_color<W: Write>(out: &mut W, pixel_color: Color) { // Result is an error handiling type that is used to indicate success or failure
    // out.write_all(&[
    writeln!(out, "{} {} {}",
        (255.999 * pixel_color.x()) as i32, // coverts the f64 value to a u8
        (255.999 * pixel_color.y()) as i32,
        (255.999 * pixel_color.z()) as i32,
    ).unwrap();
}
