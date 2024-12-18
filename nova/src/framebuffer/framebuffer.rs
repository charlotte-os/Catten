use core::sync::atomic::{AtomicPtr, Ordering};

use crate::hal::environment::boot_protocol::limine::FRAMEBUFFER_REQUEST;
use crate::framebuffer::chars::{FONT, FONT_HEIGHT, FONT_WIDTH};
// External crate for bootloader-specific functions and types.
extern crate limine;
use limine::framebuffer::Framebuffer;
use spin::lazy::Lazy;
use spin::mutex::TicketMutex;

use super::console::{CONSOLE_HEIGHT, CONSOLE_WIDTH};

/// Global access to the framebuffer
pub static FRAMEBUFFER: Lazy<TicketMutex<FrameBufferInfo>> =
    Lazy::new(|| TicketMutex::new(init_framebuffer().unwrap()));

/// A struct representing the framebuffer information,
/// including its memory address, dimensions, pixel format, etc.
pub struct FrameBufferInfo {
    address: AtomicPtr<u32>,
    width: usize,
    height: usize,
    pitch: usize,
    bpp: usize,
    scale: usize,
}

/// including its memory address, dimensions, pixel format, etc.
#[derive(Copy, Clone)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

#[allow(unused)]
impl FrameBufferInfo {
    /// Constructs a new `FrameBufferInfo` instance from a limine framebuffer.
    ///
    /// # Arguments
    ///
    /// * `framebuffer` - A reference to a limine `Framebuffer` struct.
    pub fn new(framebuffer: &Framebuffer) -> Self {
        let mut framebuffer = Self {
            address: AtomicPtr::new(framebuffer.addr() as *mut u32),
            width: framebuffer.width() as usize,
            height: framebuffer.height() as usize,
            pitch: framebuffer.pitch() as usize,
            bpp: framebuffer.bpp() as usize,
            scale: 1,
        };

        /* Initialize framebuffer scale automatically */
        framebuffer.calc_scale();
        framebuffer
    }

    /// Draws a line between two points using Bresenham's line algorithm.
    ///
    /// # Arguments
    ///
    /// * `p0` - The start point of the line.
    /// * `p1` - The end point of the line.
    /// * `color` - The color of the line in ARGB format.
    pub fn draw_line(&self, p0: Point, p1: Point, color: u32) {
        let mut x0 = p0.x;
        let mut y0 = p0.y;
        let x1 = p1.x;
        let y1 = p1.y;

        let dx = isize::abs(x1 - x0);
        let dy = -isize::abs(y1 - y0);
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy; // error value e_xy

        loop {
            self.draw_pixel(x0 as usize, y0 as usize, color);
            // Draw the current pixel
            if x0 == x1 && y0 == y1 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                // e_xy+e_x > 0
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                // e_xy+e_y < 0
                err += dx;
                y0 += sy;
            }
        }
    }

    /// Clears the entire screen to a single color.
    ///
    /// # Arguments
    ///
    /// * `color` - The color to fill the screen with, in ARGB format.
    pub fn clear_screen(&self, color: u32) {
        for y in 0..self.height {
            for x in 0..self.width {
                let pixel_offset = y * self.pitch / (self.bpp / 8) + x;
                unsafe {
                    *self.address.load(Ordering::Relaxed).add(pixel_offset) = color;
                }
            }
        }
    }

    /// Draws a single pixel at the specified location.
    ///
    /// # Arguments
    ///
    /// * `x` - The x coordinate of the pixel.
    /// * `y` - The y coordinate of the pixel.
    /// * `color` - The color of the pixel in ARGB format.

    pub fn draw_pixel(&self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            let pixel_offset = y * self.pitch / (self.bpp / 8) + x;
            unsafe {
                *self.address.load(Ordering::Relaxed).add(pixel_offset) = color;
            }
        }
    }

    /// Draws text starting from a specified location.
    ///
    /// # Arguments
    ///
    /// * `x` - The x coordinate of the starting point of the text.
    /// * `y` - The y coordinate of the starting point of the text.
    /// * `text` - The text to draw.
    /// * `color` - The color of the text in ARGB format.

    pub fn draw_text(&self, mut x: usize, mut y: usize, text: &str, color: u32, background_color: u32) {
        let start_x = x; // Remember the starting x position to reset to it on new lines
        for c in text.chars() {
            match c {
                '\n' => {
                    y += FONT_HEIGHT + 1;
                    x = start_x;
                }
                _ => {
                    self.draw_char(x, y, c, color, background_color);
                    x += FONT_WIDTH;
                }
            }
        }
    }

    /// Helper method to draw a single character from its bitmap.
    ///
    /// # Arguments
    ///
    /// * `x` - The x coordinate where the character should be drawn.
    /// * `y` - The y coordinate where the character should be drawn.
    /// * `bitmap` - A reference to the bitmap array representing the character.
    /// * `color` - The color of the character in ARGB format.
    pub fn draw_char(&self, x: usize, y: usize, chracter: char, color: u32, background_color: u32) {
        let char_int: usize = chracter as usize;
        let first_byte_index = char_int * 16;
        let mut do_draw: bool;
        let mut colour_buffer: u32;
        for by in 0..16 {
            for bi in 0..8 {
                do_draw = ((FONT[first_byte_index + by] >> (7 - bi)) & 1) != 0;
                if do_draw {
                    colour_buffer = color;
                } else {
                    colour_buffer = background_color;
                }
                self.draw_pixel(x + bi, y + by, colour_buffer);
            }
        }
    }

    /// Draws a rectangle at the specified location and dimensions.
    ///
    /// # Arguments
    ///
    /// * `x` - The x coordinate of the rectangle's top-left corner.
    /// * `y` - The y coordinate of the rectangle's top-left corner.
    /// * `width` - The width of the rectangle.
    /// * `height` - The height of the rectangle.
    /// * `color` - The color of the rectangle in ARGB format.
    pub fn draw_rect(&self, x: usize, y: usize, width: usize, height: usize, color: u32) {
        for row in y..y + height {
            for col in x..x + width {
                self.draw_pixel(col, row, color);
            }
        }
    }

    /// Draws a filled triangle between three points.
    ///
    /// # Arguments
    ///
    /// * `p1`, `p2`, `p3` - The vertices of the triangle.
    /// * `color` - The color to fill the triangle with, in ARGB format.
    pub fn draw_triangle(&self, p1: Point, p2: Point, p3: Point, color: u32) {
        // First, sort vertices by y-coordinate
        let mut vertices = [p1, p2, p3];
        vertices.sort_unstable_by_key(|v| v.y);

        // Define a closure to interpolate x values for a given y
        let interpolate_x = |p1: Point, p2: Point, current_y: isize| -> isize {
            if p1.y == p2.y {
                return p1.x;
            }
            p1.x + (p2.x - p1.x) * (current_y - p1.y) / (p2.y - p1.y)
        };

        // Function to fill between two edges from startY to endY
        let fill_between_edges =
            |self_ref: &Self, start_y: isize, end_y: isize, p_left: Point, p_right_start: Point, p_right_end: Point| {
                for y in start_y..=end_y {
                    let x_start = interpolate_x(p_left, p_right_start, y);
                    let x_end = interpolate_x(p_left, p_right_end, y);
                    for x in x_start.min(x_end)..=x_start.max(x_end) {
                        self_ref.draw_pixel(x as usize, y as usize, color);
                    }
                }
            };

        // Fill from top vertex to middle vertex
        fill_between_edges(
            self,
            vertices[0].y,
            vertices[1].y,
            vertices[0],
            vertices[1],
            vertices[2],
        );

        // Fill from middle vertex to bottom vertex
        fill_between_edges(
            self,
            vertices[1].y,
            vertices[2].y,
            vertices[2],
            vertices[0],
            vertices[1],
        );
    }

    /// Return the framebuffer scaling multiplier
    pub fn get_scale(&self) -> usize {
        self.scale
    }

    /// Automatically select scaling based on resolution
    /// Call this whenever resolution of the monitor changes!
    pub fn calc_scale(&mut self) {
        let scale_width = self.width / (CONSOLE_WIDTH * FONT_WIDTH);
        let scale_height = self.height / (CONSOLE_HEIGHT * FONT_HEIGHT);
        self.scale = if (scale_height > scale_width) {
            scale_width
        } else {
            scale_height
        };
    }
}

/// Initializes the framebuffer and returns a `FrameBufferInfo` instance if successful.
pub fn init_framebuffer() -> Option<FrameBufferInfo> {
    if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response() {
        if framebuffer_response.framebuffers().count() < 1 {
            panic!("No framebuffer returned from bootloader!");
        }

        let framebuffer = &framebuffer_response.framebuffers().next().unwrap();
        Some(FrameBufferInfo::new(framebuffer))
    } else {
        panic!("No framebuffer returned from bootlaoder!");
    }
}
