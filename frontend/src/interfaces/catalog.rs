use shared::common::BrickRenderable;

pub fn render_all_bricks_zip_bytes(tile_width: u32) -> Result<Vec<u8>, String> {
    use std::io::{Cursor, Write};

    use zip::CompressionMethod;
    use zip::write::{FileOptions, ZipWriter};

    let tile_width = tile_width.max(1);
    let mut zip = ZipWriter::new(Cursor::new(Vec::<u8>::new()));
    let opts = FileOptions::<()>::default().compression_method(CompressionMethod::Stored);

    for (path, json) in crate::generated::brick_catalog::BRICKS {
        let brick = serde_json::from_str::<Box<dyn BrickRenderable>>(json)
            .map_err(|e| format!("{path}: {e}"))?;
        let png = brick
            .to_pixmap(tile_width)
            .map_err(|e| format!("{path}: {e}"))?
            .encode_png()
            .map_err(|e| e.to_string())?;

        let mut name = path.to_string();
        if let Some(stripped) = name.strip_suffix(".json") {
            name = stripped.to_string();
        }
        name.push_str(".png");

        zip.start_file(name, opts).map_err(|e| e.to_string())?;
        zip.write_all(&png).map_err(|e| e.to_string())?;
    }

    zip.finish()
        .map_err(|e| e.to_string())
        .map(|c| c.into_inner())
}
