pub use crate::display::{draw_rect, draw_window, screen_height, screen_width};
pub use crate::BootInfo;
pub use crate::font::draw_text;
pub use crate::mem::show_memory_info;

#[unsafe(no_mangle)]
pub extern "C" fn kernel_entry(boot_info: &'static BootInfo) -> ! {
    let dw = screen_width(boot_info);
    let dh = screen_height(boot_info);

    // 画面いっぱいに青い背景を描画
    draw_rect(boot_info, 0, 0, dw, dh, 0x0067A7CC);

    // タスクバーっぽいのを描画
    let taskbar_height = 40;
    draw_rect(boot_info, 0, dh - taskbar_height, dw, taskbar_height, 0x00405060);

    draw_window(boot_info, 50, 50, 300, 200, "Memory Info");
    {
        show_memory_info(boot_info, 60, 80, 0x00000000);
    }
    
    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}