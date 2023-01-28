use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Point, EventPump, Sdl};

use common::Canvas;

fn handle_events(event_pump: &mut EventPump) -> bool {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => return false,
            _ => {}
        }
    }

    true
}

fn run(sdl_context: Sdl, canvas: Canvas) -> anyhow::Result<()> {
    let mut event_pump = sdl_context.event_pump().map_err(anyhow::Error::msg)?;
    'running: loop {
        canvas.clear(Color::BLACK);

        // pump the event loop
        if !handle_events(&mut event_pump) {
            break 'running;
        }

        canvas.put_pixel(Point::new(0, 0), Color::WHITE)?;

        // present the frame
        canvas.present();

        // 60 fps-ish
        std::thread::sleep(std::time::Duration::from_millis(1_000u64 / 60));
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    // init SDL
    let sdl_context = sdl2::init().map_err(anyhow::Error::msg)?;
    let video_subsystem = sdl_context.video().map_err(anyhow::Error::msg)?;

    // create the window (1080p)
    let window = video_subsystem
        .window("SDL2 demo", 1920, 1080)
        .position_centered()
        .build()
        .map_err(anyhow::Error::msg)?;

    // create the canvas
    let canvas = Canvas::from_window(window)?;

    // run
    run(sdl_context, canvas)?;

    Ok(())
}
