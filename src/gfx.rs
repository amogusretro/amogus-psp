pub struct Renderer{
    draw_buffer: *mut u32,
    disp_buffer: *mut u32
}

use psp::sys::{
    DisplayMode,
    DisplayPixelFormat,
    DisplaySetBufSync

};

impl Renderer{
    pub unsafe fn new() -> Self {
        let draw_buffer = psp::sys::sceGeEdramGetAddr() as *mut u32;
        let disp_buffer = psp::sys::sceGeEdramGetAddr().add(512 * 272 * 4) as *mut u32;
        psp::sys::sceDisplaySetMode(DisplayMode::Lcd, 480, 272);
        psp::sys::sceDisplaySetFrameBuf(disp_buffer as *const u8, 512, DisplayPixelFormat::Psm8888, DisplaySetBufSync::NextFrame);
        Self {
            draw_buffer,
            disp_buffer
        }

    }
    pub fn clear(&self, color: u32){
        unsafe {
            for i in 0..512*272{
                *self.draw_buffer.add(i as usize) = color;
            }
        }
    }
    pub fn swap_buffers(&mut self){
        core::mem::swap(&mut self.draw_buffer, &mut self.disp_buffer);
        unsafe {
            psp::sys::sceKernelDcacheWritebackInvalidateAll();
            psp::sys::sceDisplaySetFrameBuf(
                self.disp_buffer as *const u8,
                512,
                DisplayPixelFormat::Psm8888,
                DisplaySetBufSync::NextFrame
            );
        }
    }
}