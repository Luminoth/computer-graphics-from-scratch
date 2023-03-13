use sdl2::pixels::Color;

use common::*;

const WINDOW_TITLE: &str = "Chapter 8";

fn render(canvas: &Canvas) -> anyhow::Result<()> {
    canvas.draw_shaded_triangle(
        Point::new(-200, -250, 0.3),
        Point::new(200, 50, 0.1),
        Point::new(20, 250, 1.0),
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
