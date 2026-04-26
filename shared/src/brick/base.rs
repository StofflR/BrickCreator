use crate::color::ColorScheme;
use crate::common::*;
use rusttype::{Font, Scale, point};

pub const VARIABLE_MARKER: &str = "*";
pub const DROP_MARKER: &str = "_";
const DROP_SCALE: f32 = 0.8;
pub const DEFAULT_X_OFFSET: f32 = 0.11;
pub const EMPTY_BRICK_HINT: &str = "Use * for variables and _ for dropdowns";

// escaping: ensure that the symbol in the brick is read as text
fn escape_xml_text(text: &str) -> String {
    let mut escaped = String::with_capacity(text.len());
    for ch in text.chars() {
        match ch {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&apos;"),
            ' ' => escaped.push_str("&#160;"),
            c if c < '\u{20}' && c != '\n' && c != '\r' && c != '\t' => {}
            _ => escaped.push(ch),
        }
    }
    escaped
}

fn font_size_from_scale(scale: &Scale) -> f32 {
    (scale.y + scale.x) / 2.0
}

fn svg_text_scale(font_size: f32) -> Scale {
    Scale {
        x: font_size,
        y: font_size,
    }
}

fn advance(text: &str, scale: &Scale) -> f32 {
    let (advance, _) = get_font_metrics(text, scale);
    advance
}

pub fn get_cap_height(scale: &Scale) -> f32 {
    let Some(font) = Font::try_from_bytes(FONT_DATA) else {
        return 0.0;
    };
    let v_metrics = font.v_metrics(rusttype::Scale {
        x: scale.x,
        y: scale.y,
    });

    v_metrics.ascent - v_metrics.descent + v_metrics.line_gap
}

fn get_font_metrics(text: &str, scale: &Scale) -> (f32, f32) {
    let Some(font) = Font::try_from_bytes(FONT_DATA) else {
        return (0.0, 0.0);
    };

    let Some(width) = font
        .layout(
            text,
            rusttype::Scale {
                x: scale.x,
                y: scale.y,
            },
            point(0.0, 0.0),
        )
        .map(|c| c.position().x + c.unpositioned().h_metrics().advance_width)
        .last()
    else {
        return (0.0, get_cap_height(scale));
    };

    (width, get_cap_height(scale))
}

fn handle_text(content: &str, color_scheme: &ColorScheme, font_size: f32, text_width: f32) -> String {
    let content = escape_xml_text(content);
    format!(
        "<text xml:space=\"preserve\" textLength=\"{}\" lengthAdjust=\"spacingAndGlyphs\" style=\"fill:{};font-size:{}px;font-family:'Roboto',sans-serif;font-weight:bold;\">{}</text>",
        text_width.max(0.0), color_scheme.text, font_size, content
    )
}

pub fn render_plain_text(content: &str, color_scheme: &ColorScheme, scale: &Scale) -> String {
    let font_size = font_size_from_scale(scale);
    let text_scale = svg_text_scale(font_size);
    handle_text(
        content,
        color_scheme,
        font_size,
        advance(content, &text_scale),
    )
}

pub fn render_empty_hint(
    content: &str,
    color_scheme: &ColorScheme,
    scale: &Scale,
    max_width: f32,
) -> String {
    let content = escape_xml_text(content);
    format!(
        "<text xml:space=\"preserve\" textLength=\"{}\" lengthAdjust=\"spacingAndGlyphs\" style=\"fill:{};font-size:{}px;font-family:'Roboto',sans-serif;font-weight:bold;opacity:0.55;\">{}</text>",
        max_width.max(0.0),
        color_scheme.text,
        font_size_from_scale(scale),
        content
    )
}

pub fn content_width(content: &str, brick: &BaseBrick) -> f32 {
    let segmetns = content.splitn(3, VARIABLE_MARKER);
    let font_size = font_size_from_scale(&brick.scale);
    let text_scale = svg_text_scale(font_size);
    let drop_font_size = font_size * DROP_SCALE;
    let drop_scale = svg_text_scale(drop_font_size);

    segmetns
        .enumerate()
        .map(|(index, element)| match index {
            0 => line_segment_width(element, &text_scale, &drop_scale),
            1 => advance(element, &text_scale),
            _ => content_width(element, brick),
        })
        .sum()
}

fn line_segment_width(content: &str, text_scale: &Scale, drop_scale: &Scale) -> f32 {
    content
        .splitn(3, DROP_MARKER)
        .enumerate()
        .map(|(index, element)| match index {
            0 => advance(element, text_scale),
            1 => advance(element, drop_scale),
            _ => line_segment_width(element, text_scale, drop_scale),
        })
        .sum()
}
fn handle_drop(content: &str, color_scheme: &ColorScheme, font_size: f32) -> String {
    let text_width = advance(content, &svg_text_scale(font_size));
    let content = escape_xml_text(content);
    format!(
        "<text xml:space=\"preserve\" textLength=\"{}\" lengthAdjust=\"spacingAndGlyphs\" style=\"fill:{};font-size:{}px;font-family:'Roboto',sans-serif;font-weight:bold;\">{}</text>",
        text_width.max(0.0), color_scheme.text, font_size, content
    )
}

fn handle_line_segment(content: &str, brick: &BaseBrick) -> String {
    let segments = content.splitn(3, DROP_MARKER);
    let font_size = font_size_from_scale(&brick.scale);
    let text_scale = svg_text_scale(font_size);
    let drop_font_size = font_size * DROP_SCALE;
    let drop_scale = svg_text_scale(drop_font_size);
    segments
        .enumerate()
        .map(|(index, element)| match index {
            0 => {
                let width = advance(element, &text_scale);
                (
                    handle_text(element, &brick.color_scheme, font_size, width),
                    width,
                )
            }
            1 => {
                let width = advance(element, &drop_scale);
                (
                    handle_drop(element, &brick.color_scheme, drop_font_size),
                    width,
                )
            }
            _ => (parse_line(element, brick), 0.0),
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
fn handle_variable(content: &str, brick: &BaseBrick) -> String {
    let color_scheme = &brick.color_scheme;
    let font_size = font_size_from_scale(&brick.scale);
    let text_scale = svg_text_scale(font_size);
    let text_width = advance(content, &text_scale);
    let underline_y = font_size * 0.12;
    let content = escape_xml_text(content);
    format!(
        "<g><text xml:space=\"preserve\" textLength=\"{}\" lengthAdjust=\"spacingAndGlyphs\" style=\"fill:{};font-size:{}px;font-family:'Roboto',sans-serif;font-weight:bold;\">{}</text><line stroke=\"{}\" x1=\"0\" y1=\"{}\" x2=\"{}\" y2=\"{}\"/></g>",
        text_width.max(0.0), color_scheme.text, font_size, content, color_scheme.text, underline_y, text_width, underline_y
    )
}

pub fn parse_line(content: &str, brick: &BaseBrick) -> String {
    let segmetns = content.splitn(3, VARIABLE_MARKER);
    let font_size = font_size_from_scale(&brick.scale);
    let text_scale = svg_text_scale(font_size);

    segmetns
        .enumerate()
        .map(|(index, element)| {
            match index {
                0 => (handle_line_segment(element, brick), advance(element, &text_scale)),
                1 => (handle_variable(element, brick), advance(element, &text_scale)),
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

#[derive(Clone, PartialEq)]
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
            offset: (DEFAULT_X_OFFSET, 0.0),
            scale: Scale { x: 18.75, y: 13.0 },
        }
    }
}
