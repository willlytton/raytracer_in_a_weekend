use glam::Vec3;
use super::camera::Ray;



pub fn hit_sphere(center: Vec3, radius: f32, r: &Ray) -> bool {
    let oc: Vec3 = center - r.origin();
    let a = Vec3::dot(r.direction(), r.direction());
    let b = -2.0 * Vec3::dot(r.direction(), oc);
    let c = Vec3::dot(oc, oc) - radius * radius;
    let discriminate = b * b - 4 as f32 * a * c;
    return discriminate >= 0 as f32;
}
