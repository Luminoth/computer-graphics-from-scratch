use glam::Vec3;

use common::*;

const WINDOW_TITLE: &str = "Chapter 10";

#[allow(non_snake_case)]
fn render(canvas: &Canvas) -> anyhow::Result<()> {
    let mut scene = Scene::default();
    scene.add_instance(Instance::new_cube(Transform::from_translation(Vec3::new(
        0.0, 0.0, 5.0,
    ))));
    scene.add_instance(Instance::new_cube(Transform::from_translation(Vec3::new(
        1.0, 2.0, 3.0,
    ))));

    canvas.render_scene(&scene)?;

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
