use glam::IVec3;
use sdl2::pixels::Color;

#[derive(Debug)]
pub struct Sphere {
    center: IVec3,
    radius: u32,

    color: Color,
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            center: IVec3::default(),
            radius: 1,
            color: Color::WHITE,
        }
    }
}

impl Sphere {
    pub const fn new(center: IVec3, radius: u32, color: Color) -> Self {
        Self {
            center,
            radius,
            color,
        }
    }

    pub fn get_center(&self) -> IVec3 {
        self.center
    }

    pub fn get_radius(&self) -> u32 {
        self.radius
    }

    pub fn get_color(&self) -> Color {
        self.color
    }

    // page 20 - 22
    pub fn intersect_ray(&self, origin: IVec3, direction: IVec3) -> Option<(i32, i32)> {
        let r = self.radius as i32;
        let co = origin - self.center;

        let a = direction.dot(direction);
        let b = 2 * co.dot(direction);
        let c = co.dot(co) - r * r;

        // solve (t1, t2) = (-b +- sqrt(b^2 - 4ac)) / 2a

        let discriminant = b * b - 4 * a * c;
        if discriminant < 0 {
            // no solution
            return None;
        }

        let ds = (discriminant as f32).sqrt() as i32;

        let t1 = (-b + ds) / (2 * a);
        let t2 = (-b - ds) / (2 * a);

        Some((t1, t2))
    }
}
