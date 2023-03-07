use sdl2::{pixels::Color, rect::Point};

use common::*;

const WINDOW_TITLE: &str = "Chapter 6";

fn render(canvas: &Canvas) -> anyhow::Result<()> {
    canvas.draw_line(Point::new(-200, -100), Point::new(240, 120), Color::WHITE)?;
    canvas.draw_line(Point::new(-50, -200), Point::new(60, 240), Color::WHITE)?;

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
