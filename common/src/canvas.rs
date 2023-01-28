use std::cell::RefCell;

use sdl2::{pixels::Color, rect::Point, render::Canvas as SDLCanvas, video::Window};

pub struct Canvas {
    canvas: RefCell<SDLCanvas<Window>>,
}

impl Canvas {
    pub fn from_window(window: Window) -> anyhow::Result<Self> {
        let canvas = window.into_canvas().build().map_err(anyhow::Error::msg)?;
        Ok(Self {
            canvas: RefCell::new(canvas),
        })
    }

    pub fn clear(&self, color: Color) {
        self.canvas.borrow_mut().set_draw_color(color);
        self.canvas.borrow_mut().clear();
    }

    pub fn put_pixel(&self, point: Point, color: Color) -> anyhow::Result<()> {
        self.canvas.borrow_mut().set_draw_color(color);

        self.canvas
            .borrow_mut()
            .draw_point(point)
            .map_err(anyhow::Error::msg)
    }

    pub fn present(&self) {
        self.canvas.borrow_mut().present();
    }
}
