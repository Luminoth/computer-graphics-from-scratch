use std::cell::RefCell;

use glam::{IVec3, Vec3};
use sdl2::{pixels::Color, render::Canvas as SDLCanvas, video::Window};

use crate::math::*;
use crate::Triangle;

pub struct Canvas {
    half_width: i32,
    width_ratio: f32,
    inv_width_ratio: f32,

    half_height: i32,
    height_ratio: f32,
    inv_height_ratio: f32,

    viewport_distance: f32,

    canvas: RefCell<SDLCanvas<Window>>,
}

impl Canvas {
    pub fn from_window(window: Window) -> anyhow::Result<Self> {
        let viewport = IVec3::new(1, 1, 1);

        let size = window.size();
        let canvas = window.into_canvas().build().map_err(anyhow::Error::msg)?;

        let width_ratio = viewport.x as f32 / size.0 as f32;
        let height_ratio = viewport.y as f32 / size.1 as f32;

        Ok(Self {
            half_width: size.0 as i32 / 2,
            width_ratio,
            inv_width_ratio: 1.0 / width_ratio,
            half_height: size.1 as i32 / 2,
            height_ratio,
            inv_height_ratio: 1.0 / height_ratio,
            viewport_distance: viewport.z as f32,
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

    #[inline]
    pub fn to_viewport(&self, x: i32, y: i32) -> Vec3 {
        Vec3::new(
            x as f32 * self.width_ratio,
            y as f32 * self.height_ratio,
            self.viewport_distance,
        )
    }

    #[inline]
    pub fn from_viewport(&self, x: f32, y: f32) -> Vec3 {
        Vec3::new(
            x * self.inv_width_ratio,
            y * self.inv_height_ratio,
            self.viewport_distance,
        )
    }

    #[inline]
    pub fn project(&self, v: &Vec3) -> Vec3 {
        self.from_viewport(
            v.x * self.viewport_distance / v.z,
            v.y * self.viewport_distance / v.z,
        )
    }

    pub fn clear(&self, color: Color) {
        self.canvas.borrow_mut().set_draw_color(color);
        self.canvas.borrow_mut().clear();
    }

    pub fn put_pixel(&self, point: IVec3, color: Color) -> anyhow::Result<()> {
        self.canvas.borrow_mut().set_draw_color(color);

        // SDL coordinate system is top-left center so we need to convert
        // from the book's centered coordinate system first
        let sdl_point =
            sdl2::rect::Point::new(self.half_width + point.x, self.half_height - point.y);

        self.canvas
            .borrow_mut()
            .draw_point(sdl_point)
            .map_err(anyhow::Error::msg)
    }

    pub fn draw_line(&self, v0: Vec3, v1: Vec3, color: Color) -> anyhow::Result<()> {
        // NOTE: Bresenham's line algorithm is an example of a more optimal way to do this
        // https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
        //
        // DDA is another option for this
        // https://en.wikipedia.org/wiki/Digital_differential_analyzer_(graphics_algorithm)

        let x0 = v0.x as i32;
        let x1 = v1.x as i32;
        let dx = v1.x - v0.x;

        let y0 = v0.y as i32;
        let y1 = v1.y as i32;
        let dy = v1.y - v0.y;

        if dx.abs() > dy.abs() {
            // horizontal-ish line

            // always draw left-to-right
            if v0.x > v1.x {
                return self.draw_line(v1, v0, color);
            }

            /*let a = dy / dx;

            let mut y = v0.y;
            for x in x0..=x1 {
                self.put_pixel(Point::new(x, y as i32), color)?;
                y += a;
            }*/
            let ys = interpolate(x0, v0.y, x1, v1.y);
            for x in x0..=x1 {
                let idx = (x - x0) as usize;
                self.put_pixel(IVec3::new(x, ys[idx] as i32, 1), color)?;
            }
        } else {
            // vertical-ish line

            // always draw bottom-to-top
            if v0.y > v1.y {
                return self.draw_line(v1, v0, color);
            }

            /*let a = dx / dy;

            let mut x = v0.x;
            for y in y0..=y1 {
                self.put_pixel(Point::new(x as i32, y), color)?;
                x += a;
            }*/
            let xs = interpolate(y0, v0.x, y1, v1.x);
            for y in y0..=y1 {
                let idx = (y - y0) as usize;
                self.put_pixel(IVec3::new(xs[idx] as i32, y, 1), color)?;
            }
        }

        Ok(())
    }

    pub fn draw_wireframe_triangle(
        &self,
        v0: Vec3,
        v1: Vec3,
        v2: Vec3,
        color: Color,
    ) -> anyhow::Result<()> {
        self.draw_line(v0, v1, color)?;
        self.draw_line(v1, v2, color)?;
        self.draw_line(v2, v0, color)?;

        Ok(())
    }

    pub fn draw_filled_triangle(
        &self,
        v0: Vec3,
        v1: Vec3,
        v2: Vec3,
        color: Color,
    ) -> anyhow::Result<()> {
        // TODO: we probably shouldn't be overwriting the z here
        self.draw_shaded_triangle(
            Vec3::new(v0.x, v0.y, 1.0),
            Vec3::new(v1.x, v1.y, 1.0),
            Vec3::new(v2.x, v2.y, 1.0),
            color,
        )
    }

    pub fn draw_shaded_triangle(
        &self,
        mut v0: Vec3,
        mut v1: Vec3,
        mut v2: Vec3,
        color: Color,
    ) -> anyhow::Result<()> {
        // NOTE: DDA is another option for this
        // https://en.wikipedia.org/wiki/Digital_differential_analyzer_(graphics_algorithm)

        // sort points by increasing y (p0 is bottom, p2 is top)
        if v1.y < v0.y {
            swap_vertices(&mut v0, &mut v1)
        }
        if v2.y < v0.y {
            swap_vertices(&mut v0, &mut v2)
        }
        if v2.y < v1.y {
            swap_vertices(&mut v1, &mut v2)
        }

        let y0 = v0.y as i32;
        let y1 = v1.y as i32;
        let y2 = v2.y as i32;

        // compute edge x-coordinates and h values of the triangle edges
        let mut x01 = interpolate(y0, v0.x, y1, v1.x);
        let mut h01 = interpolate(y0, v0.z, y1, v1.z);

        let mut x12 = interpolate(y1, v1.x, y2, v2.x);
        let mut h12 = interpolate(y1, v1.z, y2, v2.z);

        let x02 = interpolate(y0, v0.x, y2, v2.x);
        let h02 = interpolate(y0, v0.z, y2, v2.z);

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
        for y in y0..=y2 {
            let idx = (y - y0) as usize;
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
                self.put_pixel(IVec3::new(x, y, 1), shaded_color)?;
            }
        }

        Ok(())
    }

    pub fn render_object(
        &self,
        vertices: impl AsRef<[Vec3]>,
        triangles: impl AsRef<[Triangle]>,
    ) -> anyhow::Result<()> {
        let mut projected = Vec::with_capacity(vertices.as_ref().len());
        for v in vertices.as_ref() {
            // world space to viewport space
            projected.push(self.project(v));
        }

        for t in triangles.as_ref() {
            t.render(self, &projected)?;
        }

        Ok(())
    }

    pub fn present(&self) {
        self.canvas.borrow_mut().present();
    }
}
