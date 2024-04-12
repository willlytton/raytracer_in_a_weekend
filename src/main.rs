use std::io;
use glam::Vec3;
use camera::Camera;

mod sphere;
mod camera;




fn main() -> io::Result<()> {
    

    let camera = Camera::new(
        Vec3::new(0.0,0.0,0.0),
        400,
        16.0 / 9.0,
        1.0,
        2.0,
    );
    camera.render()?;
    Ok(())
}
