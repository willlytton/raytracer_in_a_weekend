use std::io::{self, Write};



// questionable: correlates to the vec3 struct float elements 
// struct Color {
//     pixel_color: Vec3,
// }


pub fn write_color<W: Write>(out: &mut W, pixel_color: Color) -> () { // Result is an error handiling type that is used to indicate success or failure
    out.write_all(&[
        (255.999 * pixel_color.x()) as u8, // coverts the f64 value to a u8
        (255.999 * pixel_color.y()) as u8,
        (255.999 * pixel_color.z()) as u8,
    ]);
    // Ok(()) // Ok is a variant of Result that indicates success
    

}