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
    canvas.draw_line(
        canvas.project(vAf).as_ivec3().into(),
        canvas.project(vBf).as_ivec3().into(),
        Color::BLUE,
    )?;
    canvas.draw_line(
        canvas.project(vBf).as_ivec3().into(),
        canvas.project(vCf).as_ivec3().into(),
        Color::BLUE,
    )?;
    canvas.draw_line(
        canvas.project(vCf).as_ivec3().into(),
        canvas.project(vDf).as_ivec3().into(),
        Color::BLUE,
    )?;
    canvas.draw_line(
        canvas.project(vDf).as_ivec3().into(),
        canvas.project(vAf).as_ivec3().into(),
        Color::BLUE,
    )?;

    // back face
    canvas.draw_line(
        canvas.project(vAb).as_ivec3().into(),
        canvas.project(vBb).as_ivec3().into(),
        Color::RED,
    )?;
    canvas.draw_line(
        canvas.project(vBb).as_ivec3().into(),
        canvas.project(vCb).as_ivec3().into(),
        Color::RED,
    )?;
    canvas.draw_line(
        canvas.project(vCb).as_ivec3().into(),
        canvas.project(vDb).as_ivec3().into(),
        Color::RED,
    )?;
    canvas.draw_line(
        canvas.project(vDb).as_ivec3().into(),
        canvas.project(vAb).as_ivec3().into(),
        Color::RED,
    )?;

    // edges
    canvas.draw_line(
        canvas.project(vAf).as_ivec3().into(),
        canvas.project(vAb).as_ivec3().into(),
        Color::GREEN,
    )?;
    canvas.draw_line(
        canvas.project(vBf).as_ivec3().into(),
        canvas.project(vBb).as_ivec3().into(),
        Color::GREEN,
    )?;
    canvas.draw_line(
        canvas.project(vCf).as_ivec3().into(),
        canvas.project(vCb).as_ivec3().into(),
        Color::GREEN,
    )?;
    canvas.draw_line(
        canvas.project(vDf).as_ivec3().into(),
        canvas.project(vDb).as_ivec3().into(),
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
