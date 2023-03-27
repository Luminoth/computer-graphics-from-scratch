use glam::Vec3;
use sdl2::pixels::Color;

use common::*;

const WINDOW_TITLE: &str = "Chapter 8";

fn render(canvas: &Canvas) -> anyhow::Result<()> {
    canvas.draw_shaded_triangle(
        Vec3::new(-200.0, -250.0, 0.3),
        Vec3::new(200.0, 50.0, 0.1),
        Vec3::new(20.0, 250.0, 1.0),
        Color::GREEN,
    )?;

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
