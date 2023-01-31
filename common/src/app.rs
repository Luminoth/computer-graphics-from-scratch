use sdl2::{event::Event, keyboard::Keycode, pixels::Color, EventPump, Sdl};

use crate::Canvas;

type RenderCb = fn(&Canvas) -> anyhow::Result<()>;

pub struct App {
    sdl_context: Sdl,
    canvas: Canvas,

    render: Option<RenderCb>,
}

impl App {
    pub fn builder() -> AppBuilder {
        AppBuilder::default()
    }

    fn handle_events(&self, event_pump: &mut EventPump) -> bool {
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

    pub fn run(&self) -> anyhow::Result<()> {
        let mut event_pump = self.sdl_context.event_pump().map_err(anyhow::Error::msg)?;
        'running: loop {
            // clear the canvas
            self.canvas.clear(Color::BLACK);

            // pump the event loop
            if !self.handle_events(&mut event_pump) {
                break 'running;
            }

            // render
            if let Some(render) = self.render {
                render(&self.canvas)?;
            }

            // present the frame
            self.canvas.present();

            // 60 fps-ish
            std::thread::sleep(std::time::Duration::from_millis(1_000u64 / 60));
        }

        Ok(())
    }
}

pub struct AppBuilder {
    window_title: String,
    window_width: u32,
    window_height: u32,

    render: Option<RenderCb>,
}

impl Default for AppBuilder {
    fn default() -> Self {
        Self {
            window_title: "Computer Graphics From Scratch".to_owned(),
            window_width: 800,
            window_height: 600,
            render: None,
        }
    }
}

impl AppBuilder {
    pub fn window_title(mut self, title: impl Into<String>) -> Self {
        self.window_title = title.into();

        self
    }

    pub fn window_size(mut self, width: u32, height: u32) -> Self {
        self.window_width = width;
        self.window_height = height;

        self
    }

    pub fn render(mut self, render: RenderCb) -> Self {
        self.render = Some(render);

        self
    }

    pub fn build(self) -> anyhow::Result<App> {
        // init SDL
        let sdl_context = sdl2::init().map_err(anyhow::Error::msg)?;
        let video_subsystem = sdl_context.video().map_err(anyhow::Error::msg)?;

        // create the window
        let window = video_subsystem
            .window(&self.window_title, self.window_width, self.window_height)
            .position_centered()
            .build()
            .map_err(anyhow::Error::msg)?;

        // create the canvas
        let canvas = Canvas::from_window(window)?;

        Ok(App {
            sdl_context,
            canvas,
            render: self.render,
        })
    }
}
