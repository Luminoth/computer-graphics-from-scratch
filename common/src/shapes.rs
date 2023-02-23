use glam::Vec3;
use sdl2::pixels::Color;

#[derive(Debug)]
pub struct Sphere {
    center: Vec3,
    radius: f32,

    color: Color,
    shininess: Option<f32>,
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            center: Vec3::default(),
            radius: 1.0,
            color: Color::WHITE,
            shininess: None,
        }
    }
}

impl Sphere {
    pub const fn new(center: Vec3, radius: f32, color: Color, shininess: Option<f32>) -> Self {
        Self {
            center,
            radius,
            color,
            shininess,
        }
    }

    #[inline]
    pub fn get_center(&self) -> Vec3 {
        self.center
    }

    #[inline]
    pub fn get_radius(&self) -> f32 {
        self.radius
    }

    #[inline]
    pub fn get_color(&self) -> Color {
        self.color
    }

    #[inline]
    pub fn get_shininess(&self) -> Option<f32> {
        self.shininess
    }

    // page 20 - 22
    pub fn intersect_ray(&self, origin: Vec3, direction: Vec3) -> Option<(f32, f32)> {
        let r = self.radius;
        let co = origin - self.center;

        let a = direction.dot(direction);
        let b = 2.0 * co.dot(direction);
        let c = co.dot(co) - r * r;

        // solve (t1, t2) = (-b +- sqrt(b^2 - 4ac)) / 2a

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            // no solution
            return None;
        }

        let ds = discriminant.sqrt();

        let t1 = (-b + ds) / (2.0 * a);
        let t2 = (-b - ds) / (2.0 * a);

        Some((t1, t2))
    }
}
