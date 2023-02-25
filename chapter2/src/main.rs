use glam::Vec3;
use sdl2::{pixels::Color, rect::Point};

use common::*;

const WINDOW_TITLE: &str = "Chapter 2";

const SPHERES: &[Shape] = &[
    Shape::new_sphere(
        Vec3::new(0.0, -1.0, 3.0),
        1.0,
        Material::new(Color::RED, None, None),
    ),
    Shape::new_sphere(
        Vec3::new(2.0, 0.0, 4.0),
        1.0,
        Material::new(Color::BLUE, None, None),
    ),
    Shape::new_sphere(
        Vec3::new(-2.0, 0.0, 4.0),
        1.0,
        Material::new(Color::GREEN, None, None),
    ),
];

fn render(canvas: &Canvas) -> anyhow::Result<()> {
    let camera = Camera::default();
    for x in -canvas.get_half_width()..=canvas.get_half_width() {
        for y in -canvas.get_half_height()..=canvas.get_half_height() {
            let direction = camera.get_rotation() * canvas.to_viewport(x, y);
            let color = trace_ray_no_lights(
                camera.get_position(),
                direction,
                1.0,
                INFINITY,
                SPHERES,
                Color::WHITE,
            );
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
