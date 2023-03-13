use std::cell::RefCell;

use glam::{IVec3, Vec3};
use sdl2::{pixels::Color, render::Canvas as SDLCanvas, video::Window};

use crate::math::*;

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
        let sdl_point =
            sdl2::rect::Point::new(self.half_width + point.x(), self.half_height - point.y());

        self.canvas
            .borrow_mut()
            .draw_point(sdl_point)
            .map_err(anyhow::Error::msg)
    }

    pub fn draw_line(&self, p0: Point, p1: Point, color: Color) -> anyhow::Result<()> {
        // NOTE: Bresenham's line algorithm is an example of a more optimal way to do this
        // https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
        //
        // DDA is another option for this
        // https://en.wikipedia.org/wiki/Digital_differential_analyzer_(graphics_algorithm)

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
            let ys = interpolate(p0.x(), y0, p1.x(), y1);
            for x in p0.x()..=p1.x() {
                let idx = (x - p0.x()) as usize;
                self.put_pixel(Point::new(x, ys[idx] as i32, 1.0), color)?;
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
            let xs = interpolate(p0.y(), x0, p1.y(), x1);
            for y in p0.y()..=p1.y() {
                let idx = (y - p0.y()) as usize;
                self.put_pixel(Point::new(xs[idx] as i32, y, 1.0), color)?;
            }
        }

        Ok(())
    }

    pub fn draw_wireframe_triangle(
        &self,
        p0: Point,
        p1: Point,
        p2: Point,
        color: Color,
    ) -> anyhow::Result<()> {
        self.draw_line(p0, p1, color)?;
        self.draw_line(p1, p2, color)?;
        self.draw_line(p2, p0, color)?;

        Ok(())
    }

    pub fn draw_filled_triangle(
        &self,
        p0: Point,
        p1: Point,
        p2: Point,
        color: Color,
    ) -> anyhow::Result<()> {
        self.draw_shaded_triangle(
            Point::new(p0.x(), p0.y(), 1.0),
            Point::new(p1.x(), p1.y(), 1.0),
            Point::new(p2.x(), p2.y(), 1.0),
            color,
        )
    }

    pub fn draw_shaded_triangle(
        &self,
        mut p0: Point,
        mut p1: Point,
        mut p2: Point,
        color: Color,
    ) -> anyhow::Result<()> {
        // NOTE: DDA is another option for this
        // https://en.wikipedia.org/wiki/Digital_differential_analyzer_(graphics_algorithm)

        // sort points by increasing y (p0 is bottom, p2 is top)
        if p1.y() < p0.y() {
            swap_points(&mut p0, &mut p1)
        }
        if p2.y() < p0.y() {
            swap_points(&mut p0, &mut p2)
        }
        if p2.y() < p1.y() {
            swap_points(&mut p1, &mut p2)
        }

        let x0 = p0.x() as f32;
        let x1 = p1.x() as f32;
        let x2 = p2.x() as f32;

        let h0 = p0.h();
        let h1 = p1.h();
        let h2 = p2.h();

        // compute edge x-coordinates and h values of the triangle edges
        let mut x01 = interpolate(p0.y(), x0, p1.y(), x1);
        let mut h01 = interpolate(p0.y(), h0, p1.y(), h1);

        let mut x12 = interpolate(p1.y(), x1, p2.y(), x2);
        let mut h12 = interpolate(p1.y(), h1, p2.y(), h2);

        let x02 = interpolate(p0.y(), x0, p2.y(), x2);
        let h02 = interpolate(p0.y(), h0, p2.y(), h2);

        // concatenate the short sides (x1/h1 and x12/h12)
        x01.pop(); // remove overlapping point first
        x01.append(&mut x12);
        let x012 = x01;

        h01.pop();
        h01.append(&mut h12);
        let h012 = h01;

        // determine which is left or right by comparing the middle row
        let mut x_left = &x012;
        let mut h_left = &h012;
        let mut x_right = &x02;
        let mut h_right = &h02;
        let m = x02.len() / 2;
        if x02[m] < x012[m] {
            x_left = &x02;
            h_left = &h02;

            x_right = &x012;
            h_right = &h012;
        }

        // draw the horizontal segments
        for y in p0.y()..=p2.y() {
            let idx = (y - p0.y()) as usize;
            let x_l = x_left[idx] as i32;
            let h_l = h_left[idx];
            let x_r = x_right[idx] as i32;
            let h_r = h_right[idx];

            let h_segment = interpolate(x_l, h_l, x_r, h_r);
            for x in x_l..=x_r {
                let idx = (x - x_l) as usize;
                let c = h_segment[idx];

                let shaded_color = Color::RGB(
                    (color.r as f32 * c) as u8,
                    (color.g as f32 * c) as u8,
                    (color.b as f32 * c) as u8,
                );
                self.put_pixel(Point::new(x, y, 1.0), shaded_color)?;
            }
        }

        Ok(())
    }

    pub fn present(&self) {
        self.canvas.borrow_mut().present();
    }
}
