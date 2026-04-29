use crate::color::ColorScheme;
use crate::common::*;
use rusttype::{Font, Scale, point};

pub const VARIABLE_MARKER: &str = "*";
pub const DROP_MARKER: &str = "_";
const DROP_SCALE: f32 = 0.8;
const DROPDOWN_TRIANGLE_SCALE: f32 = 0.65;
const DROPDOWN_TRIANGLE_RIGHT_MARGIN: f32 = 0.2;
pub const DEFAULT_X_OFFSET: f32 = 0.11;
pub const EMPTY_BRICK_HINT: &str =
    "Enter content here! Use *word* for variables and _word_ for dropdowns";

#[derive(Clone, Debug, PartialEq)]
pub enum ContentLine {
    Plain(String),
}

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

fn split_marker_pair<'a>(content: &'a str, marker: &str) -> Option<(&'a str, &'a str, &'a str)> {
    let mut start = 0;

    while let Some(open_offset) = content[start..].find(marker) {
        let open = start + open_offset;
        let inner_start = open + marker.len();

        if let Some(close_offset) = content[inner_start..].find(marker) {
            let close = inner_start + close_offset;
            let inner = &content[inner_start..close];
            if inner.chars().any(|ch| !ch.is_whitespace()) {
                return Some((&content[..open], inner, &content[close + marker.len()..]));
            }
            start = close + marker.len();
        } else {
            return None;
        }
    }

    None
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

fn handle_text(
    content: &str,
    color_scheme: &ColorScheme,
    font_size: f32,
    text_width: f32,
) -> String {
    let content = escape_xml_text(content);
    format!(
        "<text xml:space=\"preserve\" textLength=\"{}\" lengthAdjust=\"spacingAndGlyphs\" style=\"fill:{};font-size:{}px;font-family:'Roboto',sans-serif;font-weight:bold;\">{}</text>",
        text_width.max(0.0),
        color_scheme.text,
        font_size,
        content
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
    let font_size = font_size_from_scale(&brick.scale);
    let text_scale = svg_text_scale(font_size);
    let drop_font_size = font_size * DROP_SCALE;
    let drop_scale = svg_text_scale(drop_font_size);

    match split_marker_pair(content, VARIABLE_MARKER) {
        Some((prefix, inner, suffix)) => {
            line_segment_width(prefix, &text_scale, &drop_scale)
                + advance(inner, &text_scale)
                + content_width(suffix, brick)
        }
        None => line_segment_width(content, &text_scale, &drop_scale),
    }
}

fn line_segment_width(content: &str, text_scale: &Scale, drop_scale: &Scale) -> f32 {
    match split_marker_pair(content, DROP_MARKER) {
        Some((prefix, inner, suffix)) => {
            advance(prefix, text_scale)
                + advance(inner, drop_scale)
                + line_segment_width(suffix, text_scale, drop_scale)
        }
        None => advance(content, text_scale),
    }
}

fn handle_drop(content: &str, color_scheme: &ColorScheme, font_size: f32) -> String {
    let text_width = advance(content, &svg_text_scale(font_size));
    let content = escape_xml_text(content);
    format!(
        "<text xml:space=\"preserve\" textLength=\"{}\" lengthAdjust=\"spacingAndGlyphs\" style=\"fill:{};font-size:{}px;font-family:'Roboto',sans-serif;font-weight:bold;\">{}</text>",
        text_width.max(0.0),
        color_scheme.text,
        font_size,
        content
    )
}

pub fn dropdown_triangle_reserved_width(scale: &Scale) -> f32 {
    let font_size = font_size_from_scale(scale) * DROP_SCALE;
    let triangle_width = font_size * DROPDOWN_TRIANGLE_SCALE;
    triangle_width + font_size * DROPDOWN_TRIANGLE_RIGHT_MARGIN
}

pub fn line_has_dropdown(content: &str) -> bool {
    split_marker_pair(content, DROP_MARKER).is_some()
}

pub fn render_line_dropdown_triangle(
    color_scheme: &ColorScheme,
    scale: &Scale,
    text_width: f32,
) -> String {
    let font_size = font_size_from_scale(scale) * DROP_SCALE;
    let triangle_width = font_size * DROPDOWN_TRIANGLE_SCALE;
    let triangle_height = triangle_width * 0.7;
    let triangle_x = text_width.max(0.0);
    let triangle_top = -font_size * 0.45;
    let triangle_bottom = triangle_top + triangle_height;

    format!(
        "<polygon fill=\"{}\" points=\"{},{} {},{} {},{}\" />",
        color_scheme.text,
        triangle_x,
        triangle_top,
        triangle_x + triangle_width,
        triangle_top,
        triangle_x + triangle_width / 2.0,
        triangle_bottom,
    )
}

fn handle_line_segment(content: &str, brick: &BaseBrick) -> String {
    let font_size = font_size_from_scale(&brick.scale);
    let text_scale = svg_text_scale(font_size);
    let drop_font_size = font_size * DROP_SCALE;
    let drop_scale = svg_text_scale(drop_font_size);

    match split_marker_pair(content, DROP_MARKER) {
        Some((prefix, inner, suffix)) => {
            let prefix_width = advance(prefix, &text_scale);
            let drop_width = advance(inner, &drop_scale);
            let suffix = parse_line(suffix, brick);

            format!(
                "<g transform=\"translate(0 0)\">{}</g><g transform=\"translate({} 0)\">{}</g><g transform=\"translate({} 0)\">{}</g>",
                handle_text(prefix, &brick.color_scheme, font_size, prefix_width),
                prefix_width,
                handle_drop(inner, &brick.color_scheme, drop_font_size),
                prefix_width + drop_width,
                suffix
            )
        }
        None => {
            let width = advance(content, &text_scale);
            handle_text(content, &brick.color_scheme, font_size, width)
        }
    }
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
        text_width.max(0.0),
        color_scheme.text,
        font_size,
        content,
        color_scheme.text,
        underline_y,
        text_width,
        underline_y
    )
}

pub fn parse_line(content: &str, brick: &BaseBrick) -> String {
    let font_size = font_size_from_scale(&brick.scale);
    let text_scale = svg_text_scale(font_size);

    match split_marker_pair(content, VARIABLE_MARKER) {
        Some((prefix, inner, suffix)) => {
            let prefix_width =
                line_segment_width(prefix, &text_scale, &svg_text_scale(font_size * DROP_SCALE));
            let variable_width = advance(inner, &text_scale);

            format!(
                "<g transform=\"translate(0 0)\">{}</g><g transform=\"translate({} 0)\">{}</g><g transform=\"translate({} 0)\">{}</g>",
                handle_line_segment(prefix, brick),
                prefix_width,
                handle_variable(inner, brick),
                prefix_width + variable_width,
                parse_line(suffix, brick)
            )
        }
        None => handle_line_segment(content, brick),
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lone_markers_do_not_create_dropdowns() {
        assert!(!line_has_dropdown("before _ after"));
        assert!(!line_has_dropdown("_"));
    }

    #[test]
    fn paired_dropdown_markers_still_work() {
        assert!(line_has_dropdown("before _choice_ after"));
    }

    #[test]
    fn parser_requires_paired_variable_markers() {
        let brick = BaseBrick::default();

        let plain = parse_line("keep * this plain", &brick);
        let variable = parse_line("make *this* variable", &brick);

        assert!(plain.contains("keep&#160;*&#160;this&#160;plain"));
        assert!(variable.contains("<line stroke="));
    }
}
