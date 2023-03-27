use glam::Vec3;
use sdl2::pixels::Color;

use common::*;

const WINDOW_TITLE: &str = "Chapter 6";

fn render(canvas: &Canvas) -> anyhow::Result<()> {
    canvas.draw_line(
        Vec3::new(-200.0, -100.0, 1.0),
        Vec3::new(240.0, 120.0, 1.0),
        Color::WHITE,
    )?;
    canvas.draw_line(
        Vec3::new(-50.0, -200.0, 1.0),
        Vec3::new(60.0, 240.0, 1.0),
        Color::WHITE,
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
