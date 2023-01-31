use std::cell::RefCell;

use glam::{IVec3, Vec3};
use sdl2::{pixels::Color, rect::Point, render::Canvas as SDLCanvas, video::Window};

pub struct Canvas {
    half_width: i32,
    width_ratio: f32,
    half_height: i32,
    height_ratio: f32,

    viewport_distance: i32,

    canvas: RefCell<SDLCanvas<Window>>,
}

impl Canvas {
    pub fn from_window(window: Window) -> anyhow::Result<Self> {
        let viewport = IVec3::new(1, 1, 1);

        let size = window.size();
        let canvas = window.into_canvas().build().map_err(anyhow::Error::msg)?;

        Ok(Self {
            half_width: size.0 as i32 / 2,
            width_ratio: viewport.x as f32 / size.0 as f32,
            half_height: size.1 as i32 / 2,
            height_ratio: viewport.y as f32 / size.1 as f32,
            viewport_distance: viewport.z,
            canvas: RefCell::new(canvas),
        })
    }

    pub fn get_half_width(&self) -> i32 {
        self.half_width
    }

    pub fn get_half_height(&self) -> i32 {
        self.half_height
    }

    pub fn to_viewport(&self, x: i32, y: i32) -> Vec3 {
        Vec3::new(
            x as f32 * self.width_ratio,
            y as f32 * self.height_ratio,
            self.viewport_distance as f32,
        )
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
