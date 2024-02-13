#![no_std]
#![no_main]
psp::module!("amogus", 1, 0);
fn psp_main(){
    psp::enable_home_button();
    psp::dprintln!("Hello from Rust PSP!");
}