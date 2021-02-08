#[panic_handler]
fn panic(_: &::core::panic::PanicInfo) -> ! {
    use core::arch::nvptx::*;

    unsafe { trap() }
}
