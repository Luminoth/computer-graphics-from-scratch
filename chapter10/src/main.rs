use glam::Vec3;
use sdl2::pixels::Color;

use common::*;

const WINDOW_TITLE: &str = "Chapter 10";

#[allow(non_snake_case)]
fn render(canvas: &Canvas) -> anyhow::Result<()> {
    let vertices = vec![
        Vec3::new(-2.0, -0.5, 5.0),
        Vec3::new(-2.0, 0.5, 5.0),
        Vec3::new(-1.0, 0.5, 5.0),
        Vec3::new(-1.0, -0.5, 5.0),
        Vec3::new(-2.0, -0.5, 6.0),
        Vec3::new(-2.0, 0.5, 6.0),
        Vec3::new(-1.0, 0.5, 6.0),
        Vec3::new(-1.0, -0.5, 6.0),
        /*Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(-1.0, 1.0, 1.0),
        Vec3::new(-1.0, -1.0, 1.0),
        Vec3::new(1.0, -1.0, 1.0),
        Vec3::new(1.0, 1.0, -1.0),
        Vec3::new(-1.0, 1.0, -1.0),
        Vec3::new(-1.0, -1.0, -1.0),
        Vec3::new(-1.0, -1.0, -1.0),*/
    ];

    let triangles = vec![
        Triangle::new(0, 1, 2, Color::RED.into()),
        Triangle::new(0, 2, 3, Color::RED.into()),
        Triangle::new(4, 0, 3, Color::GREEN.into()),
        Triangle::new(4, 3, 7, Color::GREEN.into()),
        Triangle::new(5, 4, 7, Color::BLUE.into()),
        Triangle::new(5, 7, 6, Color::BLUE.into()),
        Triangle::new(1, 5, 6, Color::YELLOW.into()),
        Triangle::new(1, 6, 2, Color::YELLOW.into()),
        Triangle::new(4, 5, 1, Color::MAGENTA.into()),
        Triangle::new(4, 1, 0, Color::MAGENTA.into()),
        Triangle::new(2, 6, 7, Color::CYAN.into()),
        Triangle::new(2, 7, 3, Color::CYAN.into()),
    ];

    canvas.render_object(vertices, triangles)?;

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
