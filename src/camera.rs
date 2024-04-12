use std::io;
use glam::Vec3;
use std::{ fs::File, io::BufWriter, io::Write };

use super::sphere::hit_sphere;





pub struct Ray {
    pub orig: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(orig: Vec3, dir: Vec3) -> Self {
        Ray { orig: orig, dir: dir }
    }

    #[allow(dead_code)]
    pub fn at(&self, t: f32) -> Vec3 {
        self.orig + t * self.dir
    }
    #[allow(dead_code)]
    pub fn origin(&self) -> Vec3 {
        return self.orig;
    }

    pub fn direction(&self) -> Vec3 {
        return self.dir;
    }

}

pub struct Camera {
    image_width: u32,
    image_height: u32,
    // pixel_samples: f32, // color scale factor for a sum of pixel samples
    camera_center: Vec3, // camera center
    pixel100_loc: Vec3, // location for pixel 0, 0
    pixel_delta_u: Vec3, //offset to pixel below,
    pixel_delta_v: Vec3,
    // defocus_disk_u: Vec3, // Defocus disk horizontal radius
    // defocus_disk_v: Vec3, // Defocus disk vertial radius
}

impl Camera {
    pub fn new(
        camera_center: Vec3,
        image_width: u32,
        aspect_ratio: f32,
        focal_length: f32,
        viewport_height: f32,
    ) -> Self {

        let image_height: u32 = 1.0_f32.max(image_width as f32 / aspect_ratio) as u32;
        if image_height < 1 { 1 } else { image_height };
        // Determine viewport dimensions
        // let focal_length: f32 = 1.0;
        // let viewport_height: f32 = 2.0;
        let viewport_width: f32 = viewport_height * (image_width as f32 / image_height as f32);
        // let camera_center = Vec3::new(0.0, 0.0, 0.0);


        // Calculate the u,v,w unit basis vectors for the camera coordinate frame

        // Calculate the vectors across the horizontal and down the vertical viewport edges
        let viewport_u: Vec3 = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel
        let pixel_delta_u = viewport_u / image_width as f32;
        let pixel_delta_v = viewport_v / image_height as f32;

        // Calculate the location of upper left pixel
        let viewport_upper_left = camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u/2 as f32 - viewport_v/2 as f32;
        println!("{}", viewport_upper_left);
        let pixel100_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        // Calculate the camera defocus dish basis vectors
        // let defocus_radius = focus_distance * defocus_angle / 2;
        // let defocus_distance_u = u * defocus_radius;
        // let defocus_disk_v = v * defocus_radius;

        Camera {
            image_height,
            image_width,
            // pixel_samples,
            camera_center,
            pixel100_loc,
            pixel_delta_u,
            pixel_delta_v,
            // defocus_disk_u,
            // defocus_disk_v,
        
        }
    }

    // parameter is hitable world
    pub fn render(&self) -> io::Result<()> {
        let f = File::create("image.ppm")?;
        let mut writer = BufWriter::new(f);
        write!(
            &mut writer,
            "P3\n{} {}\n255\n",
            self.image_width, self.image_height
        )?;
        for j in 0..self.image_height {
            println!("Scanlines remaining: {:?}", (self.image_height - j));
            for i in 0..self.image_width {
                let pixel_center: Vec3 = self.pixel100_loc + (i as f32 * self.pixel_delta_u) + (j as f32 * self.pixel_delta_v);
                let ray_direction = pixel_center - self.camera_center;
                let r = Ray::new(self.camera_center, ray_direction);

                let pixel_color: Vec3 = self.ray_color(&r);
                Camera::write_color(&mut writer, &pixel_color)?;  
            }
        }
        Ok(())
    }
    // temporary to spawn color
    fn ray_color(&self, r: &Ray) -> Vec3 {
        if hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, r) {
            return Vec3::new(1.0, 0.0, 0.0 );
        }
        let unit_direction: Vec3 = r.direction();
        let a: f32 = 0.5 * (unit_direction.y + 1.0);
        return (1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0);

        // Vec3::ZERO // Sets all Vector coordinates set to zero
    }

    fn write_color(out: &mut BufWriter<File>, pixel_color: &Vec3) -> io::Result<()> { // Result is an error handiling type that is used to indicate success or failure
        let rbyte = (255.999 * pixel_color.x) as u32; // coverts the f64 value to a u8
        let gbyte = (255.999 * pixel_color.y) as u32;
        let bbyte = (255.999 * pixel_color.z) as u32;
        write!(out, "{} {} {}", rbyte, gbyte, bbyte)?;
        Ok(())
    }
}