use glam::IVec3;
use sdl2::{pixels::Color, rect::Point};

use common::*;

const SPHERES: &'static [Sphere] = &[
    Sphere::new(IVec3::new(0, -1, 3), 1, Color::RED),
    Sphere::new(IVec3::new(2, 0, 4), 1, Color::BLUE),
    Sphere::new(IVec3::new(-2, 0, 4), 1, Color::GREEN),
];

const MAX_RAY_LEN: i32 = 10;

fn trace_ray(origin: IVec3, direction: IVec3, t_min: i32, t_max: i32) -> Color {
    let mut closest_t = i32::MAX;
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
        Color::WHITE
    }
}

fn render(canvas: &Canvas) -> anyhow::Result<()> {
    let camera_pos = IVec3::default();
    for x in -canvas.get_half_width()..=canvas.get_half_width() {
        for y in -canvas.get_half_height()..=canvas.get_half_height() {
            let direction = canvas.to_viewport(x, y);
            let color = trace_ray(camera_pos, direction, 1, MAX_RAY_LEN);
            canvas.put_pixel(Point::new(x, y), color)?;
        }
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let app = App::builder()
        .window_title("Chapter 1")
        .window_size(1920, 1080)
        .render(render)
        .build()?;

    app.run()?;

    Ok(())
}
