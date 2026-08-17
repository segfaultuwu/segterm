use super::color::Color;
use alloc::vec;
use alloc::vec::Vec;
use limine::framebuffer::Framebuffer as LimineFb;

pub struct Framebuffer<'a> {
    pub inner: &'a LimineFb,

    pub width: usize,
    pub height: usize,
    pub pitch: usize,
    pub bpp: usize,

    back_buf: Vec<u8>,
    back_buf_enabled: bool,
}

impl<'a> Framebuffer<'a> {
    pub fn new(inner: &'a LimineFb) -> Self {
        let width = inner.width as usize;
        let height = inner.height as usize;
        let pitch = inner.pitch as usize;

        let size = pitch * height;

        Self {
            inner,
            width,
            height,
            pitch,
            bpp: inner.bpp as usize,

            back_buf: vec![0; size],
            back_buf_enabled: false,
        }
    }

    pub fn backbuffer_enabled(&self) -> bool {
        self.back_buf_enabled
    }

    pub fn enable_backbuffer(&mut self) {
        self.back_buf_enabled = true;
    }

    pub fn disable_backbuffer(&mut self) {
        self.back_buf_enabled = false;
    }

    pub fn switch_backbuffer(&mut self) {
        self.back_buf_enabled = !self.back_buf_enabled;
    }

    pub fn put_pixel(&mut self, x: usize, y: usize, color: Color) {
        if x >= self.width || y >= self.height {
            return;
        }

        if self.bpp != 32 {
            return;
        }

        let offset = y * self.pitch + x * 4;

        let red_shift = self.inner.red_mask_shift;
        let green_shift = self.inner.green_mask_shift;
        let blue_shift = self.inner.blue_mask_shift;

        let pixel = ((color.r as u32) << red_shift)
            | ((color.g as u32) << green_shift)
            | ((color.b as u32) << blue_shift);

        let bytes = pixel.to_ne_bytes();

        if self.back_buf_enabled {
            self.back_buf[offset..offset + 4].copy_from_slice(&bytes);
        } else {
            unsafe {
                let ptr = self.inner.address() as *mut u8;

                ptr.add(offset).write_volatile(bytes[0]);
                ptr.add(offset + 1).write_volatile(bytes[1]);
                ptr.add(offset + 2).write_volatile(bytes[2]);
                ptr.add(offset + 3).write_volatile(bytes[3]);
            }
        }
    }

    pub fn present(&mut self) {
        if !self.back_buf_enabled {
            return;
        }

        unsafe {
            let framebuffer = self.inner.address() as *mut u8;

            for i in 0..self.back_buf.len() {
                framebuffer.add(i).write_volatile(self.back_buf[i]);
            }
        }
    }

    pub fn clear_backbuffer(&mut self, color: Color) {
        if self.bpp != 32 {
            return;
        }

        let red_shift = self.inner.red_mask_shift;
        let green_shift = self.inner.green_mask_shift;
        let blue_shift = self.inner.blue_mask_shift;

        let pixel = ((color.r as u32) << red_shift)
            | ((color.g as u32) << green_shift)
            | ((color.b as u32) << blue_shift);

        let bytes = pixel.to_ne_bytes();

        for chunk in self.back_buf.chunks_exact_mut(4) {
            chunk.copy_from_slice(&bytes);
        }
    }

    pub fn scroll_up(&mut self, pixels: usize, background: Color) {
        let offset = pixels * self.pitch;
        let size = self.height * self.pitch;

        if offset >= size {
            return;
        }

        if self.back_buf_enabled {
            self.back_buf.copy_within(offset..size, 0);
            
            let clear_start = size - offset;
            let red_shift = self.inner.red_mask_shift;
            let green_shift = self.inner.green_mask_shift;
            let blue_shift = self.inner.blue_mask_shift;

            let pixel = ((background.r as u32) << red_shift)
                | ((background.g as u32) << green_shift)
                | ((background.b as u32) << blue_shift);
            let bytes = pixel.to_ne_bytes();

            for chunk in self.back_buf[clear_start..].chunks_exact_mut(4) {
                chunk.copy_from_slice(&bytes);
            }
        } else {
            unsafe {
                let ptr = self.inner.address() as *mut u8;
                core::ptr::copy(ptr.add(offset), ptr, size - offset);
                
                let clear_start = size - offset;
                let red_shift = self.inner.red_mask_shift;
                let green_shift = self.inner.green_mask_shift;
                let blue_shift = self.inner.blue_mask_shift;

                let pixel = ((background.r as u32) << red_shift)
                    | ((background.g as u32) << green_shift)
                    | ((background.b as u32) << blue_shift);
                let bytes = pixel.to_ne_bytes();

                let mut current = clear_start;
                while current < size {
                    ptr.add(current).write_volatile(bytes[0]);
                    ptr.add(current + 1).write_volatile(bytes[1]);
                    ptr.add(current + 2).write_volatile(bytes[2]);
                    ptr.add(current + 3).write_volatile(bytes[3]);
                    current += 4;
                }
            }
        }
    }

    pub fn scroll_down(&mut self, pixels: usize, background: Color) {
        let offset = pixels * self.pitch;
        let size = self.height * self.pitch;

        if offset >= size {
            return;
        }

        if self.back_buf_enabled {
            self.back_buf.copy_within(0..size - offset, offset);
            
            let clear_end = offset;
            let red_shift = self.inner.red_mask_shift;
            let green_shift = self.inner.green_mask_shift;
            let blue_shift = self.inner.blue_mask_shift;

            let pixel = ((background.r as u32) << red_shift)
                | ((background.g as u32) << green_shift)
                | ((background.b as u32) << blue_shift);
            let bytes = pixel.to_ne_bytes();

            for chunk in self.back_buf[0..clear_end].chunks_exact_mut(4) {
                chunk.copy_from_slice(&bytes);
            }
        } else {
            unsafe {
                let ptr = self.inner.address() as *mut u8;
                core::ptr::copy(ptr, ptr.add(offset), size - offset);
                
                let clear_end = offset;
                let red_shift = self.inner.red_mask_shift;
                let green_shift = self.inner.green_mask_shift;
                let blue_shift = self.inner.blue_mask_shift;

                let pixel = ((background.r as u32) << red_shift)
                    | ((background.g as u32) << green_shift)
                    | ((background.b as u32) << blue_shift);
                let bytes = pixel.to_ne_bytes();

                let mut current = 0;
                while current < clear_end {
                    ptr.add(current).write_volatile(bytes[0]);
                    ptr.add(current + 1).write_volatile(bytes[1]);
                    ptr.add(current + 2).write_volatile(bytes[2]);
                    ptr.add(current + 3).write_volatile(bytes[3]);
                    current += 4;
                }
            }
        }
    }
}
