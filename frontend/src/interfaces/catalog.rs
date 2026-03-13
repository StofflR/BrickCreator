use shared::common::BrickRenderable;

pub fn render_all_bricks_png_bytes(tile_width: u32) -> Result<Vec<u8>, String> {
    let tile_width = tile_width.max(1);

    let pixmaps = crate::generated::brick_catalog::BRICKS
        .iter()
        .map(|(path, json)| {
            let brick = serde_json::from_str::<Box<dyn BrickRenderable>>(json)
                .map_err(|e| format!("{path}: {e}"))?;
            brick
                .to_pixmap(tile_width)
                .map_err(|e| format!("{path}: {e}"))
        })
        .collect::<Result<Vec<_>, _>>()?;

    if pixmaps.is_empty() {
        return tiny_skia::Pixmap::new(tile_width, 1)
            .ok_or_else(|| "Failed to create empty pixmap".to_string())?
            .encode_png()
            .map_err(|e| e.to_string());
    }

    let count = pixmaps.len() as u32;
    let cols = ((count as f32).sqrt().ceil() as u32).max(1);
    let rows = count.div_ceil(cols).max(1);

    let max_h = pixmaps
        .iter()
        .map(tiny_skia::Pixmap::height)
        .max()
        .unwrap_or(1)
        .max(1);

    let canvas_w = cols.saturating_mul(tile_width).max(1);
    let canvas_h = rows.saturating_mul(max_h).max(1);

    // Keep within typical max texture sizes.
    if canvas_w > 16_384 || canvas_h > 16_384 {
        return Err(format!(
            "Resulting image too large: {}x{}. Try a smaller tile width.",
            canvas_w, canvas_h
        ));
    }

    let mut canvas = tiny_skia::Pixmap::new(canvas_w, canvas_h)
        .ok_or_else(|| format!("Failed to create canvas {}x{}", canvas_w, canvas_h))?;

    for (i, pm) in pixmaps.iter().enumerate() {
        let i = i as u32;
        let col = i % cols;
        let row = i / cols;
        let x = col.saturating_mul(tile_width);
        let y = row.saturating_mul(max_h);
        blit_over(&mut canvas, pm, x, y);
    }

    canvas.encode_png().map_err(|e| e.to_string())
}

fn blit_over(canvas: &mut tiny_skia::Pixmap, src: &tiny_skia::Pixmap, x: u32, y: u32) {
    let canvas_w = canvas.width();
    let canvas_h = canvas.height();

    if x >= canvas_w || y >= canvas_h {
        return;
    }

    let copy_w = (canvas_w - x).min(src.width()) as usize;
    let copy_h = (canvas_h - y).min(src.height()) as usize;

    let src_data = src.data();
    let canvas_w_usize = canvas_w as usize;
    let src_w_usize = src.width() as usize;

    for row in 0..copy_h {
        let dst_row = (y as usize) + row;
        let src_row_start = row * src_w_usize * 4;
        let dst_row_start = dst_row * canvas_w_usize * 4 + (x as usize) * 4;

        let canvas_data = canvas.data_mut();

        for col in 0..copy_w {
            let s = src_row_start + col * 4;
            let d = dst_row_start + col * 4;

            let src_a = src_data[s + 3] as u32;
            if src_a == 0 {
                continue;
            }
            let inv_a = 255 - src_a;

            canvas_data[d] = (src_data[s] as u32 + canvas_data[d] as u32 * inv_a / 255) as u8;
            canvas_data[d + 1] =
                (src_data[s + 1] as u32 + canvas_data[d + 1] as u32 * inv_a / 255) as u8;
            canvas_data[d + 2] =
                (src_data[s + 2] as u32 + canvas_data[d + 2] as u32 * inv_a / 255) as u8;
            canvas_data[d + 3] = (src_a + canvas_data[d + 3] as u32 * inv_a / 255) as u8;
        }
    }
}

