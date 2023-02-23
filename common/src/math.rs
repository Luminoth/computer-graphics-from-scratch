use glam::Vec3;

pub fn reflect_ray(r: Vec3, n: Vec3) -> Vec3 {
    2.0 * n * n.dot(r) - r
}
