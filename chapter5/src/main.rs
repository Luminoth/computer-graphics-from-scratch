use glam::{EulerRot, IVec3, Quat, Vec3};
use sdl2::pixels::Color;

use common::*;

const WINDOW_TITLE: &str = "Chapter 5";

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

const REFLECT_DEPTH: usize = 3;

fn render(canvas: &Canvas) -> anyhow::Result<()> {
    let camera = Camera::new(
        Vec3::new(5.0, 5.0, -15.0),
        Quat::from_euler(
            EulerRot::YXZ,
            -10.0_f32.to_radians(),
            10.0_f32.to_radians(),
            0.0_f32.to_radians(),
        ),
    );

    // TODO: parallelize this
    for x in -canvas.get_half_width()..=canvas.get_half_width() {
        for y in -canvas.get_half_height()..=canvas.get_half_height() {
            let direction = camera.get_rotation() * canvas.to_viewport(x, y);
            let color = trace_ray(
                camera.get_translation().as_dvec3(),
                direction.as_dvec3(),
                1.0,
                INFINITY,
                REFLECT_DEPTH,
                LIGHTS,
                SPHERES,
                Color::BLACK,
            );
            canvas.put_pixel(IVec3::new(x, y, 1), color)?;
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
