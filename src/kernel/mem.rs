use crate::BootInfo;
use crate::font::draw_text;
use crate::{MemoryRegion, MemoryType};

/// メモリサイズをMiB単位でフォーマットする関数
fn format_mem_mib(mut mib: u64, out: &mut heapless::String<32>) {
	if mib == 0 {
		let _ = out.push_str("0");
		return;
	}
	let mut buf = [0u8; 20];
	let mut i = 0usize;
	while mib > 0 {
		buf[i] = b'0' + (mib % 10) as u8;
		mib /= 10;
		i += 1;
	}
	for j in (0..i).rev() {
		let _ = out.push((buf[j] as char));
	}
}

/// メモリ情報を画面に表示する関数
pub fn show_memory_info(boot_info: &BootInfo, x: usize, y: usize, color: u32) {
	let mut total: u64 = 0;
	let mut usable: u64 = 0;

    // メモリマップを走査して合計と使用可能メモリを計算
	unsafe {
		let entry_size = boot_info.memory_map_entry_size as usize;
		let entries = boot_info.memory_map_len as usize;
		let base = boot_info.memory_map_addr as *const u8;
		for i in 0..entries {
			let entry_ptr = base.add(i * entry_size) as *const MemoryRegion;
			let region = &*entry_ptr;
			total = total.saturating_add(region.len);
			if region.region_type == MemoryType::Usable {
				usable = usable.saturating_add(region.len);
			}
		}
	}

	let total_mib = total / 1024 / 1024;
	let usable_mib = usable / 1024 / 1024;

    // フォーマットして描画
	let mut s1: heapless::String<32> = heapless::String::new();
	let mut s2: heapless::String<32> = heapless::String::new();
	format_mem_mib(total_mib, &mut s1);
	format_mem_mib(usable_mib, &mut s2);

	let mut line1: heapless::String<64> = heapless::String::new();
	let _ = line1.push_str("Total: ");
	let _ = line1.push_str(&s1);
	let _ = line1.push_str(" MiB");

	let mut line2: heapless::String<64> = heapless::String::new();
	let _ = line2.push_str("Usable: ");
	let _ = line2.push_str(&s2);
	let _ = line2.push_str(" MiB");

	draw_text(boot_info, x, y, &line1, color);
	draw_text(boot_info, x, y + 16, &line2, color);
}

