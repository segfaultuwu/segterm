#[derive(Debug, Clone, Copy)]
pub struct Font<'a> {
    pub width: u8,
    pub height: u8,
    pub glyphs: &'a [u8],
}

impl<'a> Font<'a> {
    pub const fn new(width: u8, height: u8, glyphs: &'a [u8]) -> Self {
        Self {
            width,
            height,
            glyphs,
        }
    }

    /// Get glyph
    pub fn glyph(&self, character: u8) -> &[u8] {
        let glyph_size = self.height as usize;
        let offset = character as usize * glyph_size;

        &self.glyphs[offset..offset + glyph_size]
    }
}
