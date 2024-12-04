use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::prelude::{Dimensions, IntoStorage};
use embedded_graphics::Pixel;
use lazy_static::lazy_static;
use limine::framebuffer::Framebuffer as LimineFramebuffer;
use spin::Mutex;

use crate::logln;

lazy_static! {
    pub static ref FRAMEBUFFER: Mutex<Framebuffer> = Mutex::new(Framebuffer::from(
        &crate::environment::boot_protocol::limine::FRAMEBUFFER_REQUEST
            .get_response()
            .expect("Limine did not provide a response to the framebuffer request.")
            .framebuffers()
            .next()
            .expect("The framebuffer request contains no usable framebuffers.")
    ));
}

pub struct Framebuffer {
    base: *mut u8,
    width: u64,
    height: u64,
}

impl From<&LimineFramebuffer<'_>> for Framebuffer {
    fn from(fb: &LimineFramebuffer) -> Self {
        Framebuffer {
            base: fb.addr(),
            width: fb.width(),
            height: fb.height(),
        }
    }
}

impl Dimensions for Framebuffer {
    fn bounding_box(&self) -> embedded_graphics::primitives::Rectangle {
        embedded_graphics::primitives::Rectangle::new(
            embedded_graphics::geometry::Point::new(0, 0),
            embedded_graphics::geometry::Size::new(self.width as u32, self.height as u32),
        )
    }
}

impl DrawTarget for Framebuffer {
    type Color = embedded_graphics::pixelcolor::Rgb888;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let fb = self.base;
        for pixel in pixels.into_iter() {
            let x = pixel.0.x as usize;
            let y = pixel.0.y as usize;
            let color = pixel.1;
            if x < self.width as usize && y < self.height as usize {
                let offset = (y as u64 * self.width as u64 + x as u64) * 4u64;
                unsafe {
                    #[cfg(debug_assertions)]
                    logln!(
                        "Drawing pixel at ({:?}, {:?}) address {:?} with color {:?}",
                        x,
                        y,
                        (fb.offset(offset as isize)),
                        color
                    );

                    core::ptr::write_volatile(fb.offset(offset as isize) as *mut u32, color.into_storage());
                }
            }
        }
        Ok(())
    }
}

unsafe impl Send for Framebuffer {}
