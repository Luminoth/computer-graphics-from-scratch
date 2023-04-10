use glam::{Quat, Vec3};

use common::*;

const WINDOW_TITLE: &str = "Chapter 11";

fn render(canvas: &Canvas) -> anyhow::Result<()> {
    let scene = Scene::default();

    let camera = Camera::new(
        Vec3::new(-3.0, 1.0, -2.0),
        Quat::from_rotation_y(-30.0_f32.to_radians()),
    );

    scene.render(canvas, &camera)?;

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
