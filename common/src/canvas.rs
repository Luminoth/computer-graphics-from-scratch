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

    #[inline]
    pub fn get_half_width(&self) -> i32 {
        self.half_width
    }

    #[inline]
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

    pub fn draw_line(&self, p0: Point, p1: Point, color: Color) -> anyhow::Result<()> {
        let x0 = p0.x() as f32;
        let x1 = p1.x() as f32;
        let dx = x1 - x0;

        let y0 = p0.y() as f32;
        let y1 = p1.y() as f32;
        let dy = y1 - y0;

        if dx.abs() > dy.abs() {
            // horizontal-ish line

            // always draw left-to-right
            if x0 > x1 {
                return self.draw_line(p1, p0, color);
            }

            /*let a = dy / dx;

            let mut y = y0;
            for x in p0.x()..=p1.x() {
                self.put_pixel(Point::new(x, y as i32), color)?;
                y += a;
            }*/
            let ys = crate::math::interpolate(p0.x(), y0, p1.x(), y1);
            for x in p0.x()..=p1.x() {
                self.put_pixel(Point::new(x, ys[(x - p0.x()) as usize] as i32), color)?;
            }
        } else {
            // vertical-ish line

            // always draw bottom-to-top
            if y0 > y1 {
                return self.draw_line(p1, p0, color);
            }

            /*let a = dx / dy;

            let mut x = x0;
            for y in p0.y()..=p1.y() {
                self.put_pixel(Point::new(x as i32, y), color)?;
                x += a;
            }*/
            let xs = crate::math::interpolate(p0.y(), x0, p1.y(), x1);
            for y in p0.y()..=p1.y() {
                self.put_pixel(Point::new(xs[(y - p0.y()) as usize] as i32, y), color)?;
            }
        }

        Ok(())
    }

    pub fn present(&self) {
        self.canvas.borrow_mut().present();
    }
}
