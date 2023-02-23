use glam::Vec3;
use sdl2::{pixels::Color, rect::Point};

use common::*;

const WINDOW_TITLE: &str = "Chapter 3";

const SPHERES: &[Sphere] = &[
    Sphere::new(Vec3::new(0.0, -1.0, 3.0), 1.0, Color::RED, Some(500.0)),
    Sphere::new(Vec3::new(2.0, 0.0, 4.0), 1.0, Color::BLUE, Some(500.0)),
    Sphere::new(Vec3::new(-2.0, 0.0, 4.0), 1.0, Color::GREEN, Some(10.0)),
    Sphere::new(
        Vec3::new(0.0, -5001.0, 0.0),
        5000.0,
        Color::YELLOW,
        Some(1000.0),
    ),
];

const LIGHTS: &[Light] = &[
    Light::new_ambient(0.2),
    Light::new_point(0.6, Vec3::new(2.0, 1.0, 0.0)),
    Light::new_directional(0.2, Vec3::new(1.0, 4.0, 4.0)),
];

fn compute_lighting(point: Vec3, normal: Vec3, v: Vec3, shininess: Option<f32>) -> f32 {
    assert!(normal.is_normalized());

    LIGHTS
        .iter()
        .map(|light| light.get_contribution(point, normal, v, shininess))
        .sum()
}

fn trace_ray(origin: Vec3, direction: Vec3, t_min: f32, t_max: f32) -> Color {
    let mut closest_t = f32::MAX;
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

    if let Some(closest_sphere) = closest_sphere {
        let closest_sphere = &SPHERES[closest_sphere];

        let p = origin + closest_t * direction;
        let n = p - closest_sphere.get_center();
        let n = n.normalize_or_zero();

        let l = compute_lighting(p, n, -direction, closest_sphere.get_shininess());
        Color::RGB(
            (closest_sphere.get_color().r as f32 * l) as u8,
            (closest_sphere.get_color().g as f32 * l) as u8,
            (closest_sphere.get_color().b as f32 * l) as u8,
        )
    } else {
        Color::WHITE
    }
}

fn render(canvas: &Canvas) -> anyhow::Result<()> {
    let camera_pos = Vec3::default();
    for x in -canvas.get_half_width()..=canvas.get_half_width() {
        for y in -canvas.get_half_height()..=canvas.get_half_height() {
            let direction = canvas.to_viewport(x, y);
            let color = trace_ray(camera_pos, direction, 1.0, f32::MAX);
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