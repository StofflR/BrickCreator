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
        let is_empty = self.content.trim().is_empty();
        let content = if is_empty {
            EMPTY_BRICK_HINT
        } else {
            self.content.as_str()
        };
        let scale = self.scale;
        let cap_height = get_cap_height(&scale);

        // Get brick dimensions for offset calculation
        let (brick_width, brick_height) = self.get_dimensions();
        let offset_x = offset.0 * brick_width as f32;
        let offset_y = offset.1 * brick_height as f32;
        let available_width = (brick_width as f32 - offset_x * 2.0).max(0.0);
        let lines: Vec<String> = content
            .split('\n')
            .flat_map(|line| wrap_content_line(line, self, available_width))
            .collect();

        let svg_lines = lines.iter().enumerate().map(|(index, line)| {
            let line_content = if is_empty {
                render_empty_hint(line, &self.color_scheme, &scale, available_width)
            } else {
                parse_line(line, self)
            };
            format!(
                "<g transform=\"translate({} {})\">{}</g>",
                offset_x,
                index as f32 * cap_height * 1.1 + 20.0 + offset_y,
                line_content
            )
        });
        svg_lines.collect()
    }
}

fn wrap_content_line(content: &str, brick: &BaseBrick, max_width: f32) -> Vec<String> {
    if content.is_empty() || max_width <= 0.0 {
        return vec![content.to_string()];
    }

    if content_width(content, brick) <= max_width {
        return vec![content.to_string()];
    }

    let mut lines = Vec::new();
    let mut start = 0;

    while start < content.len() {
        let remaining = &content[start..];
        if content_width(remaining, brick) <= max_width {
            lines.push(remaining.to_string());
            break;
        }

        let mut last_whitespace_break = None;
        let mut previous_end = start;
        let mut chosen_end = None;

        for (index, ch) in remaining.char_indices() {
            let end = start + index + ch.len_utf8();
            let candidate = &content[start..end];
            if ch.is_whitespace() {
                last_whitespace_break = Some(end);
            }
            if content_width(candidate, brick) > max_width {
                chosen_end = last_whitespace_break.or(Some(previous_end.max(start + ch.len_utf8())));
                break;
            }
            previous_end = end;
        }

        let mut end = chosen_end.unwrap_or(content.len());
        if end <= start {
            end = content[start..]
                .char_indices()
                .nth(1)
                .map(|(index, _)| start + index)
                .unwrap_or(content.len());
        }

        let line = content[start..end].trim_end();
        if !line.is_empty() {
            lines.push(line.to_string());
        }

        start = end;
        while start < content.len() {
            let next = &content[start..];
            let Some(ch) = next.chars().next() else {
                break;
            };
            if !ch.is_whitespace() {
                break;
            }
            start += ch.len_utf8();
        }
    }

    if lines.is_empty() {
        vec![content.to_string()]
    } else {
        lines
    }
}
