#![no_std]
#![no_main]
mod gfx;
psp::module!("Among Us PSP Version", 1, 0);
fn psp_main(){
    psp::enable_home_button();
    unsafe {
        let mut renderer = gfx::Renderer::new();
        loop {
            renderer.clear(0xFFFFCA82);
            renderer.swap_buffers();
            psp::sys::sceDisplayWaitVblankStart();
        }
    }
}