use crate::{brick::base::*, types};
use std::ops::{Deref, DerefMut};

use once_cell::sync::Lazy;
use std::sync::Arc;

pub const FONT_DATA: &[u8] = include_bytes!("../../shared/res/Roboto/static/Roboto-Bold.ttf");

pub static OPTIONS: Lazy<Arc<usvg::Options>> = Lazy::new(|| {
    let mut fontdb = usvg::fontdb::Database::new();
    fontdb.load_font_data(FONT_DATA.to_vec());

    let options = usvg::Options {
        fontdb: std::sync::Arc::new(fontdb),
        ..Default::default()
    };
    Arc::new(options)
});

pub trait Pixmap {
    fn to_pixmap(&self, target_width: u32) -> Result<tiny_skia::Pixmap, String>;
}

pub trait SVGRenderable {
    fn to_svg(&self) -> String;
}

pub trait BrickRenderable: Brick + Pixmap {}

// Blanket impl for anything that implements both
impl<T: Brick + Pixmap> BrickRenderable for T {}

impl<T: SVGRenderable> Pixmap for T {
    fn to_pixmap(&self, target_width: u32) -> Result<tiny_skia::Pixmap, String> {
        let svg_string = self.to_svg();

        let tree = usvg::Tree::from_str(&svg_string, &OPTIONS)
            .map_err(|e| format!("Failed to parse SVG: {:?}", e))?;

        let svg_size = tree.size();
        let svg_width = svg_size.width();
        let svg_height = svg_size.height();

        if svg_width == 0.0 || svg_height == 0.0 {
            return Err("SVG has zero width or height".to_string());
        }

        let aspect_ratio = svg_width / svg_height;
        let target_height = (target_width as f32 / aspect_ratio) as u32;

        let mut pixmap = tiny_skia::Pixmap::new(target_width, target_height).ok_or_else(|| {
            format!("Failed to create pixmap with {target_width} and {target_height}")
        })?;

        let scale_x = target_width as f32 / svg_width;
        let scale_y = target_height as f32 / svg_height;
        let transform = tiny_skia::Transform::from_scale(scale_x, scale_y);

        resvg::render(&tree, transform, &mut pixmap.as_mut());
        Ok(pixmap)
    }
}

pub trait Brick: Deref<Target = BaseBrick> + DerefMut + SVGRenderable + Pixmap {
    fn get_type(&self) -> types::BrickType;
    fn get_dimensions(&self) -> (u32, u32);
    fn parse_content(&self) -> String {
        let offset = &self.offset;
        let content = &self.content;
        let scale = &self.scale;
        let cap_height = get_cap_height(scale);
        let lines = content.split('\n');

        // Get brick dimensions for offset calculation
        let (brick_width, brick_height) = self.get_dimensions();
        let offset_x = offset.0 * brick_width as f32;
        let offset_y = offset.1 * brick_height as f32;

        let svg_lines = lines.enumerate().map(|(index, line)| {
            format!(
                "<g transform=\"translate({} {})\">{}</g>",
                offset_x,
                index as f32 * cap_height * 1.1 + 20.0 + offset_y,
                parse_line(line, self)
            )
        });
        svg_lines.collect()
    }
}
