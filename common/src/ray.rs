use glam::DVec3;
use sdl2::pixels::Color;

use crate::lights::*;
use crate::math::*;
use crate::shapes::*;

/// Reflects a ray around a normal
pub fn reflect_ray(r: DVec3, n: DVec3) -> DVec3 {
    2.0 * n * n.dot(r) - r
}

/// Checks to see if the given ray intersects a shape
pub fn does_intersect(
    origin: DVec3,
    direction: DVec3,
    t_min: f64,
    t_max: f64,
    shapes: impl AsRef<[Shape]>,
) -> bool {
    let shapes = shapes.as_ref();

    shapes.iter().any(|shape| {
        if let Some((t1, t2)) = shape.intersect_ray(origin, direction) {
            if (t_min..=t_max).contains(&t1) {
                return true;
            }

            if (t_min..=t_max).contains(&t2) {
                return true;
            }
        }

        false
    })
}

/// Finds the shape closest to the origin that intersects the ray between t_min / t_max
pub fn closest_intersection(
    origin: DVec3,
    direction: DVec3,
    t_min: f64,
    t_max: f64,
    shapes: impl AsRef<[Shape]>,
) -> (Option<usize>, f64) {
    let shapes = shapes.as_ref();

    let mut closest_t = INFINITY;
    let mut closest_shape_idx = None;

    // TODO: there's probably a better method for this
    shapes.iter().enumerate().for_each(|(idx, shape)| {
        if let Some((t1, t2)) = shape.intersect_ray(origin, direction) {
            if (t_min..=t_max).contains(&t1) && t1 < closest_t {
                closest_t = t1;
                closest_shape_idx = Some(idx);
            }

            if (t_min..=t_max).contains(&t2) && t2 < closest_t {
                closest_t = t2;
                closest_shape_idx = Some(idx);
            }
        }
    });

    (closest_shape_idx, closest_t)
}

/// Trace the given ray and return the intersection color
#[allow(clippy::too_many_arguments)]
pub fn trace_ray_no_lights(
    origin: DVec3,
    direction: DVec3,
    t_min: f64,
    t_max: f64,
    shapes: impl AsRef<[Shape]>,
    background: Color,
) -> Color {
    let shapes = shapes.as_ref();

    let mut closest_t = INFINITY;
    let mut closest_shape_idx = None;

    // TODO: there's probably a better method for this
    shapes.iter().enumerate().for_each(|(idx, shape)| {
        if let Some((t1, t2)) = shape.intersect_ray(origin, direction) {
            if (t_min..=t_max).contains(&t1) && t1 < closest_t {
                closest_t = t1;
                closest_shape_idx = Some(idx);
            }

            if (t_min..=t_max).contains(&t2) && t2 < closest_t {
                closest_t = t2;
                closest_shape_idx = Some(idx);
            }
        }
    });

    if let Some(closest_shape_idx) = closest_shape_idx {
        shapes[closest_shape_idx].get_material().get_color()
    } else {
        background
    }
}

/// Trace the given ray and return the intersection color
#[allow(clippy::too_many_arguments)]
pub fn trace_ray(
    origin: DVec3,
    direction: DVec3,
    t_min: f64,
    t_max: f64,
    reflection_depth: usize,
    lights: impl AsRef<[Light]>,
    shapes: impl AsRef<[Shape]>,
    background: Color,
) -> Color {
    let lights = lights.as_ref();
    let shapes = shapes.as_ref();

    let (closest_shape_idx, closest_t) =
        closest_intersection(origin, direction, t_min, t_max, shapes);
    if let Some(closest_shape_idx) = closest_shape_idx {
        let closest_shape = &shapes[closest_shape_idx];

        let p = origin + closest_t * direction;
        let n = p - closest_shape.get_center().as_dvec3();
        let n = n.normalize_or_zero();

        let material = closest_shape.get_material();
        let l = compute_lighting(p, n, -direction, material.get_shininess(), lights, shapes);

        let color = material.get_color();
        let local_color = Color::RGB(
            (color.r as f32 * l) as u8,
            (color.g as f32 * l) as u8,
            (color.b as f32 * l) as u8,
        );

        if reflection_depth == 0 {
            return local_color;
        }

        let r = material.get_reflectiveness();
        if let Some(r) = r {
            let reflected = reflect_ray(-direction, n);
            let reflected_color = trace_ray(
                p,
                reflected,
                SURFACE_EPSILON,
                INFINITY,
                reflection_depth - 1,
                lights,
                shapes,
                background,
            );

            Color::RGB(
                (local_color.r as f32 * (1.0 - r) + reflected_color.r as f32 * r) as u8,
                (local_color.g as f32 * (1.0 - r) + reflected_color.g as f32 * r) as u8,
                (local_color.b as f32 * (1.0 - r) + reflected_color.b as f32 * r) as u8,
            )
        } else {
            local_color
        }
    } else {
        background
    }
}
