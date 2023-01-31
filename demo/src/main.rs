use sdl2::{pixels::Color, rect::Point};

use common::*;

fn render(canvas: &Canvas) -> anyhow::Result<()> {
    canvas.put_pixel(Point::new(0, 0), Color::WHITE)?;

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let app = App::builder()
        .window_title("SDL2 demo")
        .window_size(1920, 1080)
        .render(render)
        .build()?;
    app.run()?;

    Ok(())
}
