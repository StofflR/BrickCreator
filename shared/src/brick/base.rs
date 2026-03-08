use crate::color::ColorScheme;
use crate::common::*;
use rusttype::{Font, Scale, point};

const VARIABLE_MARKER: &str = "$";
const DROP_MARKER: &str = "*";
const DROP_SCALE: f32 = 0.8;
pub const DEFAULT_X_OFFSET: f32 = 0.11;

fn font_size_from_scale(scale: &Scale) -> f32 {
    (scale.y + scale.x) / 2.0
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

fn handle_line_segment(content: &str, brick: &BaseBrick) -> String {
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
fn handle_variable(content: &str, brick: &BaseBrick) -> String {
    let y = -0.1;
    let color_scheme = &brick.color_scheme;
    let font_size = font_size_from_scale(&brick.scale);
    let advance = advance(content, &brick.scale);
    format!(
        "<g><text xml:space=\"preserve\" style=\"fill:{};font-size:{}px;font-family:'Roboto',sans-serif;font-weight:bold;\">{}</text><line stroke=\"{}\" x1=\"0\" y1=\"{}\" x2=\"{}\" y2=\"{}\"/></g>",
        color_scheme.text, font_size, content, color_scheme.text, y, advance, y
    )
}

pub fn parse_line(content: &str, brick: &BaseBrick) -> String {
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
