use crate::css::Color;
use crate::painting::Canvas;
use minifb::{Key, Window, WindowOptions};

pub struct BrowserWindow {
    window: Window,
}

impl BrowserWindow {
    pub fn new(width: usize, height: usize) -> Result<Self, minifb::Error> {
        let window = Window::new(
            "Browser Engine - Press ESC to exit",
            width,
            height,
            WindowOptions {
                resize: true,
                scale_mode: minifb::ScaleMode::AspectRatioStretch,
                ..WindowOptions::default()
            },
        )?;

        Ok(BrowserWindow { window })
    }

    /// Display the canvas in the window
    pub fn display(&mut self, canvas: &Canvas) -> Result<(), minifb::Error> {
        // Convert our RGBA colors to u32 format that minifb expects (0xRRGGBB)
        let buffer: Vec<u32> = canvas
            .pixels
            .iter()
            .map(|color| Self::color_to_u32(color))
            .collect();

        self.window
            .update_with_buffer(&buffer, canvas.width, canvas.height)?;

        Ok(())
    }

    /// Check if the window should stay open
    pub fn is_open(&self) -> bool {
        self.window.is_open() && !self.window.is_key_down(Key::Escape)
    }

    /// Convert our Color struct to minifb's u32 format
    fn color_to_u32(color: &Color) -> u32 {
        ((color.r as u32) << 16) | ((color.g as u32) << 8) | (color.b as u32)
    }
}

