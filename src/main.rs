#![no_std]
#![no_main]
use core::panic::PanicInfo;



///This function is called on pani
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
