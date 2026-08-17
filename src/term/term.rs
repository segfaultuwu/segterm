use core::fmt;

use crate::{
    ansi::*,
    fb::{Color, Framebuffer},
    graphics::font::Font,
};

/// Main struct
pub struct Terminal<'a> {
    pub fb: &'a mut Framebuffer<'a>,
    pub cfg: TerminalConfig<'a>,
}

#[derive(Clone, Copy)]
pub struct TerminalConfig<'a> {
    pub x: usize,
    pub y: usize,
    pub ansi: bool,

    pub cur: Cursor,
    pub pad: Padding,

    pub font: Font<'a>,

    pub fg: Color,
    pub bg: Color,
}

#[derive(Default, Clone, Copy)]
pub struct Cursor {
    pub x: usize,
    pub y: usize,
    pub blink: bool,
}

#[derive(Default, Clone, Copy)]
pub struct Padding {
    pub x: usize,
    pub y: usize,
}

impl Default for TerminalConfig<'_> {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            ansi: false,
            cur: Cursor::default(),
            pad: Padding::default(),
            font: Font::new(8, 14, &include_bytes!("../../assets/font.psf")[4..]),
            fg: Color::WHITE,
            bg: Color::BLACK,
        }
    }
}

impl<'a> Terminal<'a> {
    pub fn new(fb: &'a mut Framebuffer<'a>, cfg: TerminalConfig<'a>) -> Self {
        Self { fb, cfg }
    }

    pub fn write(&mut self, text: &str) {
        if !self.cfg.ansi {
            self.write_plain(text);
            return;
        }

        let mut parser = Parser::new();

        for byte in text.bytes() {
            if let Some(action) = parser.advance(byte) {
                self.handle_action(action);
            }
        }
    }

    pub fn writef(&mut self, args: core::fmt::Arguments) {
        let _ = core::fmt::Write::write_fmt(self, args);
    }

    pub fn scroll_up(&mut self) {
        self.fb.scroll_up(self.cfg.font.height as usize, self.cfg.bg);
    }

    pub fn scroll_down(&mut self) {
        self.fb.scroll_down(self.cfg.font.height as usize, self.cfg.bg);
    }

    pub fn rows(&self) -> usize {
        self.fb.height.saturating_sub(self.cfg.pad.y * 2) / self.cfg.font.height as usize
    }

    pub fn cols(&self) -> usize {
        self.fb.width.saturating_sub(self.cfg.pad.x * 2) / self.cfg.font.width as usize
    }

    fn newline(&mut self) {
        self.cfg.cur.x = 0;
        self.cfg.cur.y += 1;
        let rows = self.rows();
        if self.cfg.cur.y >= rows {
            self.cfg.cur.y = rows.saturating_sub(1);
            self.scroll_up();
        }
    }

    fn write_plain(&mut self, text: &str) {
        for byte in text.bytes() {
            if byte == b'\n' {
                self.newline();
            } else if byte == b'\r' {
                self.cfg.cur.x = 0;
            } else {
                if self.cfg.cur.x >= self.cols() {
                    self.newline();
                }
                self.draw_char(byte);
                self.cfg.cur.x += 1;
            }
        }
    }

    fn handle_action(&mut self, action: Action) {
        match action {
            Action::Print(byte) => {
                if byte == b'\n' {
                    self.newline();
                } else if byte == b'\r' {
                    self.cfg.cur.x = 0;
                } else {
                    if self.cfg.cur.x >= self.cols() {
                        self.newline();
                    }
                    self.draw_char(byte);
                    self.cfg.cur.x += 1;
                }
            }

            Action::Sgr { params, len } => {
                self.handle_sgr(&params[..len]);
            }

            Action::CursorPosition { params, len } => {
                self.handle_cursor_position(&params[..len]);
            }
        }
    }

    fn handle_cursor_position(&mut self, params: &[u16]) {
        if let Some(&row) = params.get(0) {
            self.cfg.cur.y = row as usize;
        }
        if let Some(&col) = params.get(1) {
            self.cfg.cur.x = col as usize;
        }
    }

    fn handle_sgr(&mut self, params: &[u16]) {
        for &param in params {
            match param {
                0 => {
                    self.cfg.fg = Color::WHITE;
                    self.cfg.bg = Color::BLACK;
                }

                30 => self.cfg.fg = Color::BLACK,
                31 => self.cfg.fg = Color::RED,
                32 => self.cfg.fg = Color::GREEN,
                33 => self.cfg.fg = Color::YELLOW,
                34 => self.cfg.fg = Color::BLUE,
                35 => self.cfg.fg = Color::MAGENTA,
                36 => self.cfg.fg = Color::CYAN,
                37 => self.cfg.fg = Color::WHITE,

                40 => self.cfg.bg = Color::BLACK,
                41 => self.cfg.bg = Color::RED,
                42 => self.cfg.bg = Color::GREEN,
                43 => self.cfg.bg = Color::YELLOW,
                44 => self.cfg.bg = Color::BLUE,
                45 => self.cfg.bg = Color::MAGENTA,
                46 => self.cfg.bg = Color::CYAN,
                47 => self.cfg.bg = Color::WHITE,

                _ => {}
            }
        }
    }

    fn draw_char(&mut self, byte: u8) {
        let glyph = self.cfg.font.glyph(byte);

        let x = self.cfg.pad.x + self.cfg.cur.x * self.cfg.font.width as usize;

        let y = self.cfg.pad.y + self.cfg.cur.y * self.cfg.font.height as usize;

        for (row, bits) in glyph.iter().enumerate() {
            for col in 0..self.cfg.font.width {
                if bits & (0x80 >> col) != 0 {
                    self.fb.put_pixel(x + col as usize, y + row, self.cfg.fg);
                }
            }
        }
    }
}

impl fmt::Write for Terminal<'_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write(s);
        Ok(())
    }
}
