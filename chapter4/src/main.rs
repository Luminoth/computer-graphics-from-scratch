use glam::Vec3;
use sdl2::{pixels::Color, rect::Point};

use common::*;

const WINDOW_TITLE: &str = "Chapter 4";

const BACKGROUND_COLOR: Color = Color::BLACK;

const INFINITY: f32 = f32::MAX;

const SPHERES: &[Shape] = &[
    Shape::new_sphere(
        Vec3::new(0.0, -1.0, 3.0),
        1.0,
        Material::new(Color::RED, Some(500.0), Some(0.2)),
    ),
    Shape::new_sphere(
        Vec3::new(2.0, 0.0, 4.0),
        1.0,
        Material::new(Color::BLUE, Some(500.0), Some(0.3)),
    ),
    Shape::new_sphere(
        Vec3::new(-2.0, 0.0, 4.0),
        1.0,
        Material::new(Color::GREEN, Some(10.0), Some(0.4)),
    ),
    Shape::new_sphere(
        Vec3::new(0.0, -5001.0, 0.0),
        5000.0,
        Material::new(Color::YELLOW, Some(1000.0), Some(0.5)),
    ),
];

const LIGHTS: &[Light] = &[
    Light::new_ambient(0.2),
    Light::new_point(0.6, Vec3::new(2.0, 1.0, 0.0)),
    Light::new_directional(0.2, Vec3::new(1.0, 4.0, 4.0)),
];

const SURFACE_EPSILON: f32 = 0.001;

const REFLECT_DEPTH: usize = 3;

fn closest_intersection(
    origin: Vec3,
    direction: Vec3,
    t_min: f32,
    t_max: f32,
) -> (Option<usize>, f32) {
    let mut closest_t = INFINITY;
    let mut closest_sphere = None;

    for (idx, sphere) in SPHERES.iter().enumerate() {
        if let Some((t1, t2)) = sphere.intersect_ray(origin, direction) {
            if (t_min..=t_max).contains(&t1) && t1 < closest_t {
                closest_t = t1;
                closest_sphere = Some(idx);
            }

            if (t_min..=t_max).contains(&t2) && t2 < closest_t {
                closest_t = t2;
                closest_sphere = Some(idx);
            }
        }
    }

    (closest_sphere, closest_t)
}

fn compute_lighting(point: Vec3, normal: Vec3, v: Vec3, shininess: Option<f32>) -> f32 {
    assert!(normal.is_normalized());

    LIGHTS
        .iter()
        .map(|light| match light {
            Light::Ambient(light) => light.get_contribution(),
            Light::Point(light) => {
                let l = light.get_position() - point;
                let t_max = 1.0;
                let (shadow_sphere, _) = closest_intersection(point, l, SURFACE_EPSILON, t_max);
                if shadow_sphere.is_some() {
                    0.0
                } else {
                    light.get_contribution(point, normal, v, shininess)
                }
            }
            Light::Directional(light) => {
                let l = light.get_direction();
                let t_max = INFINITY;
                let (shadow_sphere, _) = closest_intersection(point, l, SURFACE_EPSILON, t_max);
                if shadow_sphere.is_some() {
                    0.0
                } else {
                    light.get_contribution(normal, v, shininess)
                }
            }
        })
        .sum()
}

fn trace_ray(origin: Vec3, direction: Vec3, t_min: f32, t_max: f32, depth: usize) -> Color {
    let (closest_sphere, closest_t) = closest_intersection(origin, direction, t_min, t_max);
    if let Some(closest_sphere) = closest_sphere {
        let closest_sphere = &SPHERES[closest_sphere];

        let p = origin + closest_t * direction;
        let n = p - closest_sphere.get_center();
        let n = n.normalize_or_zero();

        let material = closest_sphere.get_material();
        let l = compute_lighting(p, n, -direction, material.get_shininess());

        let color = material.get_color();
        let local_color = Color::RGB(
            (color.r as f32 * l) as u8,
            (color.g as f32 * l) as u8,
            (color.b as f32 * l) as u8,
        );

        if depth == 0 {
            return local_color;
        }

        let r = material.get_reflectiveness();
        if let Some(r) = r {
            let reflected = reflect_ray(-direction, n);
            let reflected_color = trace_ray(p, reflected, SURFACE_EPSILON, INFINITY, depth - 1);

            Color::RGB(
                (local_color.r as f32 * (1.0 - r) + reflected_color.r as f32 * r) as u8,
                (local_color.g as f32 * (1.0 - r) + reflected_color.g as f32 * r) as u8,
                (local_color.b as f32 * (1.0 - r) + reflected_color.b as f32 * r) as u8,
            )
        } else {
            local_color
        }
    } else {
        BACKGROUND_COLOR
    }
}

fn render(canvas: &Canvas) -> anyhow::Result<()> {
    let camera_pos = Vec3::default();
    for x in -canvas.get_half_width()..=canvas.get_half_width() {
        for y in -canvas.get_half_height()..=canvas.get_half_height() {
            let direction = canvas.to_viewport(x, y);
            let color = trace_ray(camera_pos, direction, 1.0, INFINITY, REFLECT_DEPTH);
            canvas.put_pixel(Point::new(x, y), color)?;
        }
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let app = App::builder()
        .window_title(WINDOW_TITLE)
        .window_size(1080, 1080)
        .render(render)
        .build()?;

    app.run()?;

    Ok(())
}
