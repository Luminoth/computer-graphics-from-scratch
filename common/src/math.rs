use glam::IVec3;

pub const INFINITY: f64 = f64::MAX;

#[derive(Debug, Default, Copy, Clone)]
pub struct Point {
    x: i32,
    y: i32,
    h: f32,
}

impl From<sdl2::rect::Point> for Point {
    fn from(point: sdl2::rect::Point) -> Self {
        Point::new(point.x(), point.y(), 1.0)
    }
}

impl From<IVec3> for Point {
    fn from(v: IVec3) -> Self {
        Point::new(v.x, v.y, v.z as f32)
    }
}

impl Point {
    #[inline]
    pub fn new(x: i32, y: i32, h: f32) -> Self {
        Self { x, y, h }
    }

    #[inline]
    pub fn x(&self) -> i32 {
        self.x
    }

    #[inline]
    pub fn y(&self) -> i32 {
        self.y
    }

    #[inline]
    pub fn h(&self) -> f32 {
        self.h
    }
}

#[inline]
pub(crate) fn swap_points(p0: &mut Point, p1: &mut Point) {
    std::mem::swap(p0, p1);
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
