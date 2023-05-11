use glam::Vec3;

pub const INFINITY: f64 = f64::MAX;

#[inline]
pub(crate) fn swap_vertices(v0: &mut Vec3, v1: &mut Vec3) {
    std::mem::swap(v0, v1);
}

/// Linear interpolation of d wrt i
pub fn interpolate(i0: i32, d0: f32, i1: i32, d1: f32) -> Vec<f32> {
    // TODO: um... but allocating for this is really bad

    if i0 == i1 {
        return vec![d0];
    }

    let mut values = Vec::with_capacity(((i1 - i0).abs() + 1) as usize);

    let a = (d1 - d0) / (i1 - i0) as f32;

    let mut d = d0;
    for _ in i0..=i1 {
        values.push(d);
        d += a;
    }

    values
}

#[derive(Debug, Copy, Clone)]
pub struct Plane {
    normal: Vec3,
    distance: f32,
}

impl Plane {
    #[inline]
    pub fn get_normal(&self) -> Vec3 {
        self.normal
    }

    #[inline]
    pub fn get_distance(&self) -> f32 {
        self.distance
    }

    #[inline]
    pub fn signed_distance(&self, v: &Vec3) -> f32 {
        (v.x * self.normal.x) + (v.y * self.normal.y) + (v.z * self.normal.z) + self.distance
    }
}
