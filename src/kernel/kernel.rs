pub use crate::display::draw_rect;
pub use crate::BootInfo;

#[unsafe(no_mangle)]
pub extern "C" fn kernel_entry(boot_info: &'static BootInfo) -> ! {
    // 画面左上に100x100の水色の四角を描画
    draw_rect(boot_info, 0, 0, 100, 100, 0x0067A7CC);
    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}