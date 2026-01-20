use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let bdf_path = Path::new(&manifest_dir).join("src/ter-u12b.bdf");
    let out_path = Path::new(&manifest_dir).join("src/kernel/font_data.rs");

    let file = match File::open(&bdf_path) {
        Ok(f) => f,
        Err(e) => {
            panic!("failed to open {}: {}", bdf_path.display(), e);
        }
    };

    let reader = io::BufReader::new(file);

    /// BDFをパースしたフォントデータ
    #[derive(Default)]
    struct Glyph {
        /// Unicodeコードポイント
        code: i32,
        /// バウンディングボックス幅
        bbx_w: usize,
        /// バウンディングボックス高さ
        bbx_h: usize,
        /// バウンディングボックスXオフセット
        bbx_x: isize,
        /// バウンディングボックスYオフセット
        bbx_y: isize,
        /// ビットマップデータ
        bitmap: Vec<u8>,
    }

    let mut glyphs: Vec<Glyph> = Vec::new();

    // すべての行を読み込み
    let all_lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();
    let mut i = 0usize;
    while i < all_lines.len() {
        let line = &all_lines[i];
        if line.starts_with("STARTCHAR") {
            // ENDCHARを探す
            let mut j = i + 1;
            while j < all_lines.len() && !all_lines[j].starts_with("ENDCHAR") {
                j += 1;
            }
            // i+1...j
            let chunk = &all_lines[i+1..j.min(all_lines.len())];
            let mut glyph = Glyph::default();

            let mut in_bitmap = false;
            for cl in chunk {
                if in_bitmap {
                    if cl.trim().is_empty() { continue; }
                    let hex = cl.trim();

                    let mut row_bytes: Vec<u8> = Vec::new();
                    let hex_clean = if hex.len() % 2 == 1 { format!("0{}", hex) } else { hex.to_string() };
                    for k in (0..hex_clean.len()).step_by(2) {
                        let byte = u8::from_str_radix(&hex_clean[k..k+2], 16).unwrap_or(0);
                        row_bytes.push(byte);
                    }
                    glyph.bitmap.extend(row_bytes);
                    continue;
                }
                if cl.starts_with("ENCODING") {
                    let parts: Vec<_> = cl.split_whitespace().collect();
                    if parts.len() >= 2 {
                        glyph.code = parts[1].parse::<i32>().unwrap_or(-1);
                    }
                } else if cl.starts_with("BBX") {
                    let parts: Vec<_> = cl.split_whitespace().collect();
                    if parts.len() >= 5 {
                        glyph.bbx_w = parts[1].parse::<usize>().unwrap_or(0);
                        glyph.bbx_h = parts[2].parse::<usize>().unwrap_or(0);
                        glyph.bbx_x = parts[3].parse::<isize>().unwrap_or(0);
                        glyph.bbx_y = parts[4].parse::<isize>().unwrap_or(0);
                    }
                } else if cl.starts_with("BITMAP") {
                    in_bitmap = true;
                }
            }
            if glyph.code >= 0 {
                glyphs.push(glyph);
            }
            i = j + 1;
            continue;
        }
        i += 1;
    }

    glyphs.retain(|g| (32..=126).contains(&g.code));
    glyphs.sort_by_key(|g| g.code);

    let glyph_height = glyphs.iter().map(|g| g.bbx_h).max().unwrap_or(8);

    let mut offsets: Vec<u32> = Vec::new();
    let mut widths: Vec<u8> = Vec::new();
    let mut bitmap_bytes: Vec<u8> = Vec::new();
    for g in &glyphs {
        offsets.push(bitmap_bytes.len() as u32);
        widths.push(g.bbx_w as u8);
        let bytes_per_row = ((g.bbx_w + 7) / 8) as usize;
        let rows_present = if bytes_per_row > 0 { g.bitmap.len() / bytes_per_row } else { 0 };
        for r in 0..glyph_height {
            if r < rows_present {
                let start = r * bytes_per_row;
                bitmap_bytes.extend_from_slice(&g.bitmap[start..start+bytes_per_row]);
            } else {
                bitmap_bytes.extend(std::iter::repeat(0u8).take(bytes_per_row));
            }
        }
    }
    offsets.push(bitmap_bytes.len() as u32);

    let mut code_to_index: Vec<i32> = vec![-1; 95];
    for (idx, g) in glyphs.iter().enumerate() {
        let cp = g.code as usize;
        if (32..=126).contains(&g.code) {
            code_to_index[cp - 32] = idx as i32;
        }
    }

    let mut out = File::create(&out_path).expect("failed to create font_data.rs");
    writeln!(out, "// Auto-generated from {}", bdf_path.display()).unwrap();
    writeln!(out, "pub const GLYPH_HEIGHT: usize = {};", glyph_height).unwrap();
    writeln!(out, "pub const GLYPH_COUNT: usize = {};", glyphs.len()).unwrap();

    writeln!(out, "pub const GLYPH_WIDTHS: [u8; {}] = [", widths.len()).unwrap();
    for w in &widths { writeln!(out, "    {},", w).unwrap(); }
    writeln!(out, "]; ").unwrap();

    writeln!(out, "pub const GLYPH_OFFSETS: [u32; {}] = [", offsets.len()).unwrap();
    for o in &offsets { writeln!(out, "    {},", o).unwrap(); }
    writeln!(out, "]; ").unwrap();

    writeln!(out, "pub const GLYPH_BITMAPS: [u8; {}] = [", bitmap_bytes.len()).unwrap();
    for b in &bitmap_bytes { writeln!(out, "    {},", b).unwrap(); }
    writeln!(out, "]; ").unwrap();

    writeln!(out, "pub const CODE_TO_INDEX: [i32; 95] = [").unwrap();
    for v in &code_to_index { writeln!(out, "    {},", v).unwrap(); }
    writeln!(out, "]; ").unwrap();

    println!("cargo:rerun-if-changed={}", bdf_path.display());
}
