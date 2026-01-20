use crate::BootInfo;

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/kernel/font_data.rs"));

/// 1文字を描画する関数
pub fn draw_char(boot_info: &BootInfo, x: usize, y: usize, ch: u8, color: u32) {
    if ch < 32 || ch > 126 {
        return;
    }
    let idx = CODE_TO_INDEX[(ch - 32) as usize];
    if idx < 0 { return; }
    let idx = idx as usize;

    let width = GLYPH_WIDTHS[idx] as usize;
    let height = GLYPH_HEIGHT;

    let bytes_per_row = ((width + 7) / 8) as usize;
    let start = GLYPH_OFFSETS[idx] as usize;
    let fb = boot_info.framebuffer_addr as *mut u32;
    let stride = boot_info.stride;
    for row in 0..height {
        let row_offset = start + row * bytes_per_row;
        for col in 0..width {
            let byte_index = row_offset + (col / 8);
            if byte_index >= GLYPH_BITMAPS.len() { continue; }
            let byte = GLYPH_BITMAPS[byte_index];
            let bit = 7 - (col % 8);
            if ((byte >> bit) & 1) != 0 {
                let px = x + col;
                let py = y + row;
                if px < boot_info.screen_width && py < boot_info.screen_height {
                    unsafe {
                        let off = py * stride + px;
                        fb.add(off).write_volatile(color);
                    }
                }
            }
        }
    }
}

/// 文字列を描画する関数
pub fn draw_text(boot_info: &BootInfo, mut x: usize, y: usize, s: &str, color: u32) {
    for &b in s.as_bytes() {
        if b < 32 || b > 126 { x += 4; continue; }
        let idx = CODE_TO_INDEX[(b - 32) as usize];
        if idx < 0 { x += 4; continue; }
        let w = GLYPH_WIDTHS[idx as usize] as usize;
        draw_char(boot_info, x, y, b, color);
        x += w + 1;
    }
}
