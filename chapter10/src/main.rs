use glam::{Quat, Vec3};

use common::*;

const WINDOW_TITLE: &str = "Chapter 10";

fn render(canvas: &Canvas) -> anyhow::Result<()> {
    let mut scene = Scene::default();
    scene.add_instance(Instance::new_cube(Transform::new(
        Vec3::new(-1.5, 0.0, 7.0),
        Quat::default(),
        0.75,
    )));
    scene.add_instance(Instance::new_cube(Transform::new(
        Vec3::new(1.25, 2.5, 7.5),
        Quat::from_rotation_y(195.0_f32.to_radians()),
        1.0,
    )));

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
