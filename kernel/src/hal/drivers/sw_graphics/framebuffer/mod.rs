use limine::framebuffer::Framebuffer as LimineFramebuffer;
use embedded_graphics::{draw_target::DrawTarget, prelude::{Dimensions, IntoStorage}, Pixel};

pub enum Error {}

pub struct Framebuffer<'a> {
    fb: &'a mut LimineFramebuffer<'a>,
}

impl Dimensions for Framebuffer<'_> {
    fn bounding_box(&self) -> embedded_graphics::primitives::Rectangle {
        embedded_graphics::primitives::Rectangle::new(
            embedded_graphics::geometry::Point::new(0, 0),
            embedded_graphics::geometry::Size::new(self.fb.width() as u32, self.fb.height() as u32),
        )
    }
}

impl DrawTarget for Framebuffer<'_> {
    type Color = embedded_graphics::pixelcolor::Rgb888;
    type Error = Error;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let fb = self.fb.addr();
        Ok(for pixel in pixels.into_iter() {
            let x = pixel.0.x as usize;
            let y = pixel.0.y as usize;
            let color = pixel.1;
            let offset = (y as u64).saturating_mul(self.fb.width())
                .saturating_add(x as u64)
                .saturating_mul(4);
            unsafe {
                core::ptr::write(fb.offset(offset as isize) as *mut u32, color.into_storage());
            }
        })
    }
}
