use core::fmt;

use spin::mutex::TicketMutex;

use crate::framebuffer::chars::{FONT_HEIGHT, FONT_WIDTH};
use crate::framebuffer::colors::Color;
use crate::framebuffer::framebuffer::FRAMEBUFFER;

pub const CONSOLE_WIDTH: usize = 80;
pub const CONSOLE_HEIGHT: usize = 50;

pub static CONSOLE: TicketMutex<Console> = TicketMutex::new(Console::new());

/// Represents a single character on the framebuffer console
#[derive(Copy, Clone)]
struct ConsoleChar {
    character: char,
    color: u32,
    background_color: u32,
}

/// Buffer of characters in the console
struct ConsoleBuffer {
    chars: [[ConsoleChar; CONSOLE_WIDTH]; CONSOLE_HEIGHT],
}

/// Framebuffer console
pub struct Console {
    buffer: ConsoleBuffer,
    cursor_x: usize,
    cursor_y: usize,
    text_color: u32,
    background_color: u32,
}

#[allow(unused)]
impl Console {
    /// Create a new console
    pub const fn new() -> Self {
        Self {
            buffer: ConsoleBuffer {
                chars: [[ConsoleChar {
                    character: '\0',
                    color: 0,
                    background_color: 0,
                }; CONSOLE_WIDTH]; CONSOLE_HEIGHT],
            },
            cursor_x: 0,
            cursor_y: 0,
            text_color: Color::WHITE,
            background_color: Color::BLACK,
        }
    }

    pub fn set_colors(&mut self, text_color: u32, background_color: u32) {
        self.text_color = text_color;
        self.background_color = background_color;
    }

    /// Write a char to the console
    pub fn write_char(
        &mut self,
        character: char,
        color: Option<u32>,
        background_color: Option<u32>,
    ) {
        // Write the character to the buffer
        match character {
            // Newline
            '\n' => {
                self.cursor_x = 0;
                self.cursor_y += 1;
            }
            // Carriage return
            '\r' => self.cursor_x = 0,
            // Tab
            '\t' => {
                for _ in 0..4 {
                    self.write_char(' ', color, background_color);
                }
            }
            // Backspace
            '\x08' => {
                if self.cursor_x > 0 {
                    self.cursor_x -= 1;
                    self.write_char(' ', color, background_color);
                    self.cursor_x -= 1;
                }
            }
            // Any other character
            _ => {
                self.buffer.chars[self.cursor_y][self.cursor_x] = ConsoleChar {
                    character,
                    color: color.unwrap_or(self.text_color),
                    background_color: background_color.unwrap_or(self.background_color),
                };
                self.cursor_x += 1;
            }
        }

        // If we've reached the end of the line, move to the next line
        if self.cursor_x >= CONSOLE_WIDTH {
            self.cursor_x = 0;
            self.cursor_y += 1;
        }

        // If we've reached the end of the console, scroll
        if self.cursor_y >= CONSOLE_HEIGHT {
            self.scroll();
        }
    }

    /// Write a string to the console
    pub fn write_str(&mut self, string: &str, color: Option<u32>, background_color: Option<u32>) {
        for character in string.chars() {
            self.write_char(character, color, background_color);
        }
        // Flush the console to the framebuffer
        self.flush();
    }

    /// Clear the console
    pub fn clear(&mut self) {
        for y in 0..CONSOLE_HEIGHT {
            for x in 0..CONSOLE_WIDTH {
                self.buffer.chars[y][x] = ConsoleChar {
                    character: '\0',
                    color: 0,
                    background_color: 0,
                };
            }
        }
        self.cursor_x = 0;
        self.cursor_y = 0;
    }

    /// Scroll the console
    fn scroll(&mut self) {
        // Move all lines up by one
        for y in 1..CONSOLE_HEIGHT {
            for x in 0..CONSOLE_WIDTH {
                self.buffer.chars[y - 1][x] = self.buffer.chars[y][x];
            }
        }

        // Clear the last line
        for x in 0..CONSOLE_WIDTH {
            self.buffer.chars[CONSOLE_HEIGHT - 1][x] = ConsoleChar {
                character: '\0',
                color: 0,
                background_color: 0,
            };
        }
        self.cursor_y = CONSOLE_HEIGHT - 1;
    }

    /// Flush the console to the framebuffer
    fn flush(&self) {
        let scale: usize = 1;
        for y in 0..CONSOLE_HEIGHT {
            for x in 0..CONSOLE_WIDTH {
                // Draw the character to the framebuffer
                FRAMEBUFFER.lock().draw_char(
                    /* Add a 1 pixel margin between characters */
                    x * FONT_WIDTH * scale + 1,
                    /* Add a 1 pixel margin between lines */
                    y * FONT_HEIGHT * scale + 1,
                    self.buffer.chars[y][x].character,
                    self.buffer.chars[y][x].color,
                    self.buffer.chars[y][x].background_color,
                );
            }
        }
    }

    pub fn clear_inner_styling(&self) {
        INNER_STYLE_SETTINGS.lock().clear();
    }
}

static INNER_STYLE_SETTINGS: TicketMutex<InnerPrintStyle> =
    TicketMutex::new(InnerPrintStyle::new());

/// Inner style settings for print macros
struct InnerPrintStyle {
    text_color: Option<u32>,
    background_color: Option<u32>,
    setting_text_color: bool,
    setting_background_color: bool,
}

impl InnerPrintStyle {
    /// Create a
    const fn new() -> Self {
        Self {
            text_color: None,
            background_color: None,
            setting_text_color: false,
            setting_background_color: false,
        }
    }

    fn clear(&mut self) {
        self.text_color = None;
        self.background_color = None;
        self.setting_text_color = false;
        self.setting_background_color = false;
    }
}

impl fmt::Write for Console {
    fn write_str(&mut self, string: &str) -> fmt::Result {
        let mut reading_color_type = false;
        let mut styling = INNER_STYLE_SETTINGS.lock();

        if styling.setting_text_color {
            styling.text_color = Some(u32::from_str_radix(string, 16).unwrap_or(Color::WHITE));
            styling.setting_text_color = false;
            return Ok(());
        }
        if styling.setting_background_color {
            styling.background_color =
                Some(u32::from_str_radix(string, 16).unwrap_or(Color::BLACK));
            styling.setting_background_color = false;
            return Ok(());
        }

        for character in string.chars() {
            if character == '[' {
                reading_color_type = true;
                continue;
            }
            if reading_color_type {
                if character == 'b' || character == 'B' {
                    styling.setting_text_color = false;
                    styling.setting_background_color = true;
                } else if character == 'f' || character == 'F' {
                    styling.setting_text_color = true;
                    styling.setting_background_color = false;
                }
                reading_color_type = false;
                continue;
            }
            self.write_char(character, styling.text_color, styling.background_color);
        }
        // Flush the console to the framebuffer
        self.flush();
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        use core::fmt::Write;
        use crate::framebuffer::console::CONSOLE;
        let con = CONSOLE.lock();
        con.write_fmt(format_args!($($arg)*)).unwrap();
        con.clear_inner_styling();
    }
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {
        use core::fmt::Write;
        use crate::framebuffer::console::CONSOLE;
        let mut con = CONSOLE.lock();
        con.write_fmt(format_args!($($arg)*)).unwrap();
        con.write_char('\n', None, None);
        con.clear_inner_styling();
    }
}
