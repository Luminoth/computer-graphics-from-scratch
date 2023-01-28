use std::cell::RefCell;

use sdl2::{pixels::Color, rect::Point, render::Canvas as SDLCanvas, video::Window};

pub struct Canvas {
    half_width: i32,
    half_height: i32,

    canvas: RefCell<SDLCanvas<Window>>,
}

impl Canvas {
    pub fn from_window(window: Window) -> anyhow::Result<Self> {
        let size = window.size();
        let canvas = window.into_canvas().build().map_err(anyhow::Error::msg)?;
        Ok(Self {
            half_width: size.0 as i32 / 2,
            half_height: size.1 as i32 / 2,
            canvas: RefCell::new(canvas),
        })
    }

    pub fn clear(&self, color: Color) {
        self.canvas.borrow_mut().set_draw_color(color);
        self.canvas.borrow_mut().clear();
    }

    pub fn put_pixel(&self, point: Point, color: Color) -> anyhow::Result<()> {
        self.canvas.borrow_mut().set_draw_color(color);

        // SDL coordinate system is top-left center so we need to convert
        // from the book's centered coordinate system first
        let sdl_point = Point::new(self.half_width + point.x, self.half_height - point.y);

        self.canvas
            .borrow_mut()
            .draw_point(sdl_point)
            .map_err(anyhow::Error::msg)
    }

    pub fn present(&self) {
        self.canvas.borrow_mut().present();
    }
}
