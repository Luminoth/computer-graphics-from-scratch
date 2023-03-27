use glam::Vec3;
use sdl2::pixels::Color;

use common::*;

const WINDOW_TITLE: &str = "Chapter 9";

#[allow(non_snake_case)]
fn render(canvas: &Canvas) -> anyhow::Result<()> {
    // "front" vertices
    let vAf = Vec3::new(-2.0, -0.5, 5.0);
    let vBf = Vec3::new(-2.0, 0.5, 5.0);
    let vCf = Vec3::new(-1.0, 0.5, 5.0);
    let vDf = Vec3::new(-1.0, -0.5, 5.0);

    // "back" vertices
    let vAb = Vec3::new(-2.0, -0.5, 6.0);
    let vBb = Vec3::new(-2.0, 0.5, 6.0);
    let vCb = Vec3::new(-1.0, 0.5, 6.0);
    let vDb = Vec3::new(-1.0, -0.5, 6.0);

    // front face
    canvas.draw_line(canvas.project(vAf), canvas.project(vBf), Color::BLUE)?;
    canvas.draw_line(canvas.project(vBf), canvas.project(vCf), Color::BLUE)?;
    canvas.draw_line(canvas.project(vCf), canvas.project(vDf), Color::BLUE)?;
    canvas.draw_line(canvas.project(vDf), canvas.project(vAf), Color::BLUE)?;

    // back face
    canvas.draw_line(canvas.project(vAb), canvas.project(vBb), Color::RED)?;
    canvas.draw_line(canvas.project(vBb), canvas.project(vCb), Color::RED)?;
    canvas.draw_line(canvas.project(vCb), canvas.project(vDb), Color::RED)?;
    canvas.draw_line(canvas.project(vDb), canvas.project(vAb), Color::RED)?;

    // edges
    canvas.draw_line(canvas.project(vAf), canvas.project(vAb), Color::GREEN)?;
    canvas.draw_line(canvas.project(vBf), canvas.project(vBb), Color::GREEN)?;
    canvas.draw_line(canvas.project(vCf), canvas.project(vCb), Color::GREEN)?;
    canvas.draw_line(canvas.project(vDf), canvas.project(vDb), Color::GREEN)?;

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
