use sdl2::pixels::Color;

use common::*;

const WINDOW_TITLE: &str = "Chapter 7";

fn render(canvas: &Canvas) -> anyhow::Result<()> {
    canvas.draw_filled_triangle(
        Point::new(-200, -250, 1.0),
        Point::new(200, 50, 1.0),
        Point::new(20, 250, 1.0),
        Color::GREEN,
    )?;

    canvas.draw_wireframe_triangle(
        Point::new(-200, -250, 1.0),
        Point::new(200, 50, 1.0),
        Point::new(20, 250, 1.0),
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
