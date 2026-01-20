use crate::font::draw_text;

/// フレームバッファに矩形を描画
pub fn draw_rect(boot_info: &crate::BootInfo, x: usize, y: usize, w: usize, h: usize, color: u32) {
    let fb = boot_info.framebuffer_addr as *mut u32;
    let width = boot_info.screen_width;
    let height = boot_info.screen_height;
    let stride = boot_info.stride;

    for dy in 0..h {
        for dx in 0..w {
            let px = x + dx;
            let py = y + dy;
            if px < width && py < height {
                unsafe {
                    // strideはピクセル数
                    let offset = py * stride + px;
                    fb.add(offset).write_volatile(color);
                }
            }
        }
    }
}

/// 幅
pub fn screen_width(boot_info: &crate::BootInfo) -> usize {
    boot_info.screen_width
}

/// 高さ
pub fn screen_height(boot_info: &crate::BootInfo) -> usize {
    boot_info.screen_height
}

/// ウィンドウを描画する関数
pub fn draw_window(boot_info: &crate::BootInfo, x: usize, y: usize, w: usize, h: usize, title: &str,) {
    // ウィンドウ本体
    draw_rect(boot_info, x, y, w, h, 0x00FFFFFF);
    // タイトルバー
    let titlebar_height = 24;
    draw_rect(boot_info, x, y, w, titlebar_height, 0x00405060);
    // タイトルテキスト
    draw_text(boot_info, x + 4, y + 4, title, 0x00FFFFFF);
    // 枠線
    let border_color = 0x00000000;
    // 上
    draw_rect(boot_info, x, y, w, 1, border_color);
    // 下
    draw_rect(boot_info, x, y + h - 1, w, 1, border_color);
    // 左
    draw_rect(boot_info, x, y, 1, h, border_color);
    // 右
    draw_rect(boot_info, x + w - 1, y, 1, h, border_color);
}