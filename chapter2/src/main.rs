use glam::Vec3;
use sdl2::{pixels::Color, rect::Point};

use common::*;

const WINDOW_TITLE: &str = "Chapter 2";

const BACKGROUND_COLOR: Color = Color::WHITE;

const SPHERES: &[Sphere] = &[
    Sphere::new(Vec3::new(0.0, -1.0, 3.0), 1.0, Color::RED, None, None),
    Sphere::new(Vec3::new(2.0, 0.0, 4.0), 1.0, Color::BLUE, None, None),
    Sphere::new(Vec3::new(-2.0, 0.0, 4.0), 1.0, Color::GREEN, None, None),
];

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
        SPHERES[closest_sphere].get_color()
    } else {
        BACKGROUND_COLOR
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
