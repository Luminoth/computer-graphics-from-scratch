use sdl2::{event::Event, keyboard::Keycode, pixels::Color};

fn main() -> anyhow::Result<()> {
    let sdl_context = sdl2::init().unwrap();

    // create the window
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("SDL2 demo", 1024, 768)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    // start with yellow
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        // render the next color
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();

        // pump the event loop
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        // present the frame
        canvas.present();

        // 60 fps-ish
        std::thread::sleep(std::time::Duration::from_millis(1_000u64 / 60));
    }

    Ok(())
}
