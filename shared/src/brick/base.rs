use crate::color::ColorScheme;
use once_cell::sync::Lazy;
use rusttype::{Font, Scale, point};
use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};

const VARIABLE_MARKER: &str = "$";
const DROP_MARKER: &str = "*";
const DROP_SCALE: f32 = 0.8;

const FONT_DATA: &[u8] = include_bytes!("../../../shared/res/Roboto/static/Roboto-Bold.ttf");

static OPTIONS: Lazy<Arc<usvg::Options>> = Lazy::new(|| {
    let mut fontdb = usvg::fontdb::Database::new();
    fontdb.load_font_data(FONT_DATA.to_vec());

    let options = usvg::Options {
        fontdb: std::sync::Arc::new(fontdb),
        ..Default::default()
    };
    Arc::new(options)
});

fn font_size_from_scale(scale: &Scale) -> f32 {
    (scale.y + scale.x) / 2.0
}

fn advance(text: &str, scale: &Scale) -> f32 {
    let (advance, _) = get_font_metrics(text, scale);
    advance
}

fn get_cap_height(scale: &Scale) -> f32 {
    let Some(font) = Font::try_from_bytes(FONT_DATA) else {
        return 0.0;
    };
    let v_metrics = font.v_metrics(*scale);

    v_metrics.ascent - v_metrics.descent + v_metrics.line_gap
}

fn get_font_metrics(text: &str, scale: &Scale) -> (f32, f32) {
    let Some(font) = Font::try_from_bytes(FONT_DATA) else {
        return (0.0, 0.0);
    };

    let Some(width) = font
        .layout(text, *scale, point(0.0, 0.0))
        .map(|c| c.position().x + c.unpositioned().h_metrics().advance_width)
        .last()
    else {
        return (0.0, get_cap_height(scale));
    };

    (width, get_cap_height(scale))
}

fn handle_line_segment(content: &str, brick: &impl BrickSVG) -> String {
    let segments = content.splitn(3, DROP_MARKER);
    let font_size = font_size_from_scale(&brick.scale);
    segments
        .enumerate()
        .map(|(index, element)| {
            let advance = advance(element, &brick.scale);
            match index {
                0 => (
                    handle_text(element, &brick.color_scheme, font_size),
                    advance,
                ),
                1 => (
                    handle_drop(element, &brick.color_scheme, font_size),
                    DROP_SCALE * advance,
                ),
                _ => (parse_line(element, brick), 0.0),
            }
        })
        .fold(
            (0.0, String::new()),
            |(acc, content), (current, advance)| {
                let current = format!(
                    "<g transform=\"translate({} {})\">{}</g>",
                    acc, 0.0, current
                );
                (acc + advance, format!("{}{}", content, current))
            },
        )
        .1
}

fn handle_text(content: &str, color_scheme: &ColorScheme, font_size: f32) -> String {
    format!(
        "<text xml:space=\"preserve\" style=\"fill:{};font-size:{}px;font-family:'Roboto',sans-serif;font-weight:bold;\">{}</text>",
        color_scheme.text, font_size, content
    )
}
fn handle_drop(content: &str, color_scheme: &ColorScheme, font_size: f32) -> String {
    format!(
        "<text xml:space=\"preserve\" style=\"fill:{};font-size:{}px;font-family:'Roboto',sans-serif;font-weight:bold;\" transform=\"scale({})\">{}</text>",
        color_scheme.text, font_size, DROP_SCALE, content
    )
}
fn handle_variable(content: &str, brick: &impl BrickSVG) -> String {
    let y = -0.1;
    let color_scheme = &brick.color_scheme;
    let font_size = font_size_from_scale(&brick.scale);
    let advance = advance(content, &brick.scale);
    format!(
        "<g><text xml:space=\"preserve\" style=\"fill:{};font-size:{}px;font-family:'Roboto',sans-serif;font-weight:bold;\">{}</text><line stroke=\"{}\" x1=\"0\" y1=\"{}\" x2=\"{}\" y2=\"{}\"/></g>",
        color_scheme.text, font_size, content, color_scheme.text, y, advance, y
    )
}

fn parse_line(content: &str, brick: &impl BrickSVG) -> String {
    let segmetns = content.splitn(3, VARIABLE_MARKER);

    segmetns
        .enumerate()
        .map(|(index, element)| {
            let advance = advance(element, &brick.scale);
            match index {
                0 => (handle_line_segment(element, brick), advance),
                1 => (handle_variable(element, brick), advance),
                _ => (parse_line(element, brick), 0.0),
            }
        })
        .fold(
            (0.0, String::new()),
            |(acc, content), (current, advance)| {
                let current = format!(
                    "<g transform=\"translate({} {})\">{}</g>",
                    acc, 0.0, current
                );
                (acc + advance, format!("{}{}", content, current))
            },
        )
        .1
}

pub fn parse_content(brick: &impl BrickSVG) -> String {
    let offset = &brick.offset;
    let content = &brick.content;
    let scale = &brick.scale;
    let cap_height = get_cap_height(scale);
    let lines = content.split('\n');

    // Get brick dimensions for offset calculation
    let (brick_width, brick_height) = brick.get_dimensions();
    let offset_x = offset.0 * brick_width as f32;
    let offset_y = offset.1 * brick_height as f32;

    let svg_lines = lines.enumerate().map(|(index, line)| {
        format!(
            "<g transform=\"translate({} {})\">{}</g>",
            offset_x,
            index as f32 * cap_height * 1.1 + 20.0 + offset_y,
            parse_line(line, brick)
        )
    });
    svg_lines.collect()
}
#[derive(Clone)]
pub struct BaseBrick {
    pub content: String,
    pub color_scheme: ColorScheme,
    pub offset: (f32, f32),
    pub scale: Scale,
}

impl Default for BaseBrick {
    fn default() -> Self {
        Self {
            content: String::new(),
            color_scheme: ColorScheme::default(),
            offset: (0.0, 0.0),
            scale: Scale { x: 18.75, y: 13.0 },
        }
    }
}

pub trait BrickSVG: Deref<Target = BaseBrick> + DerefMut {
    fn get_dimensions(&self) -> (u32, u32);

    fn to_pixmap(&self, target_dimensions: (u32, u32)) -> Result<tiny_skia::Pixmap, String> {
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
        let target_width = target_dimensions.0;
        let target_height = (target_width as f32 / aspect_ratio) as u32;

        let mut pixmap = tiny_skia::Pixmap::new(target_width, target_height)
            .ok_or_else(|| format!("Failed to create pixmap with {target_width} and {target_height}"))?;

        let scale_x = target_width as f32 / svg_width;
        let scale_y = target_height as f32 / svg_height;
        let transform = tiny_skia::Transform::from_scale(scale_x, scale_y);

        resvg::render(&tree, transform, &mut pixmap.as_mut());
        Ok(pixmap)
    }

    fn to_svg(&self) -> String;
}
