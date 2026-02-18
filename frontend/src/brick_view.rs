use shared::brick::base::{BaseBrick, BrickSVG};
use shared::brick::h0::BrickH0;
use shared::brick::h1_base::BrickH1Base;
use shared::brick::h1_control::BrickH1Control;
use shared::brick::h2_base::BrickH2Base;
use shared::brick::h2_control::BrickH2Control;
use shared::brick::h3_base::BrickH3Base;
use shared::color::ALL_COLOR_SCHEMES;
use slint::Color;
use slint::Image;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use web_sys::{Blob, BlobPropertyBag, HtmlAnchorElement, Url};

#[cfg(not(target_arch = "wasm32"))]
use std::fs;

/// Enum representing all available brick types
#[derive(Clone)]
pub enum BrickType {
    H0(BrickH0),
    H1Base(BrickH1Base),
    H1Control(BrickH1Control),
    H2Base(BrickH2Base),
    H2Control(BrickH2Control),
    H3Base(BrickH3Base),
}

impl BrickType {
    /// Create a new brick of the specified type with the given content
    pub fn new(brick_id: i32, base: BaseBrick) -> Option<Self> {
        match brick_id {
            0 => Some(BrickType::H0(BrickH0 { base })),
            1 => Some(BrickType::H1Base(BrickH1Base { base })),
            2 => Some(BrickType::H1Control(BrickH1Control { base })),
            3 => Some(BrickType::H2Base(BrickH2Base { base })),
            4 => Some(BrickType::H2Control(BrickH2Control { base })),
            5 => Some(BrickType::H3Base(BrickH3Base { base })),
            _ => None,
        }
    }

    pub fn to_svg(&self) -> String {
        match self {
            BrickType::H0(brick) => brick.to_svg(),
            BrickType::H1Base(brick) => brick.to_svg(),
            BrickType::H1Control(brick) => brick.to_svg(),
            BrickType::H2Base(brick) => brick.to_svg(),
            BrickType::H2Control(brick) => brick.to_svg(),
            BrickType::H3Base(brick) => brick.to_svg(),
        }
    }
    pub fn to_pixmap(&self, target_dimensions: (u32, u32)) -> Result<tiny_skia::Pixmap, String> {
        match self {
            BrickType::H0(brick) => brick.to_pixmap(target_dimensions),
            BrickType::H1Base(brick) => brick.to_pixmap(target_dimensions),
            BrickType::H1Control(brick) => brick.to_pixmap(target_dimensions),
            BrickType::H2Base(brick) => brick.to_pixmap(target_dimensions),
            BrickType::H2Control(brick) => brick.to_pixmap(target_dimensions),
            BrickType::H3Base(brick) => brick.to_pixmap(target_dimensions),
        }
    }
}

pub fn brick_to_image(brick: &BrickType, target_dimensions: (u32, u32)) -> Result<Image, String> {
    let pixmap = brick.to_pixmap(target_dimensions)?;

    let buffer = slint::SharedPixelBuffer::<slint::Rgba8Pixel>::clone_from_slice(
        pixmap.data(),
        pixmap.width(),
        pixmap.height(),
    );
    Ok(Image::from_rgba8(buffer))
}

pub fn create_brick_image(
    brick_id: i32,
    base: BaseBrick,
    target_width: u32,
) -> Result<Image, String> {
    let brick = BrickType::new(brick_id, base)
        .ok_or_else(|| format!("Invalid brick type ID: {}", brick_id))?;
    brick_to_image(&brick, (target_width, target_width))
}
/// Parse a hex color string (#RRGGBB or #RRGGBBAA) into a Slint Color
pub fn parse_hex_color(hex: &str) -> Color {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
    let a = if hex.len() >= 8 {
        u8::from_str_radix(&hex[6..8], 16).unwrap_or(255)
    } else {
        255
    };
    Color::from_argb_u8(a, r, g, b)
}

/// Update the brick preview image based on current selections
pub fn update_brick_preview(
    brick_id: i32,
    content: &str,
    color_id: usize,
    target_width: u32,
    scroll_x: f32,
    scroll_y: f32,
) -> slint::Image {
    // Get the color scheme
    if color_id >= ALL_COLOR_SCHEMES.len() {
        return Image::default(); // Return an empty image if color ID is out of bounds
    }
    let color_scheme = ALL_COLOR_SCHEMES[color_id];
    let offset = (scroll_x, scroll_y);

    let base = BaseBrick {
        content: content.to_string(),
        color_scheme,
        offset,
        ..BaseBrick::default()
    };

    create_brick_image(brick_id, base, target_width).unwrap_or_else(|err| {
        eprintln!("Error generating brick image: {}", err);
        Image::default() // Return an empty image on error
    })
}

/// Save brick as PNG
pub fn save_as_png(
    brick_id: i32,
    content: &str,
    color_id: usize,
    scroll_x: f32,
    scroll_y: f32,
    width: u32,
) -> bool {
    if color_id >= ALL_COLOR_SCHEMES.len() {
        eprintln!("Invalid color scheme ID: {}", color_id);
        return false;
    }

    let color_scheme = ALL_COLOR_SCHEMES[color_id];
    let offset = (scroll_x, scroll_y);

    let base = BaseBrick {
        content: content.to_string(),
        color_scheme,
        offset,
        ..BaseBrick::default()
    };

    let brick = match BrickType::new(brick_id, base) {
        Some(b) => b,
        None => {
            eprintln!("Invalid brick type ID: {}", brick_id);
            return false;
        }
    };

    let pixmap = match brick.to_pixmap((width, width)) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error generating pixmap: {}", e);
            return false;
        }
    };

    let png_data = match pixmap.encode_png() {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error encoding PNG: {}", e);
            return false;
        }
    };

    save_file("brick.png", &png_data, "image/png")
}

/// Save brick as SVG
pub fn save_as_svg(
    brick_id: i32,
    content: &str,
    color_id: usize,
    scroll_x: f32,
    scroll_y: f32,
) -> bool {
    if color_id >= ALL_COLOR_SCHEMES.len() {
        eprintln!("Invalid color scheme ID: {}", color_id);
        return false;
    }

    let color_scheme = ALL_COLOR_SCHEMES[color_id];
    let offset = (scroll_x, scroll_y);

    let base = BaseBrick {
        content: content.to_string(),
        color_scheme,
        offset,
        ..BaseBrick::default()
    };

    let brick = match BrickType::new(brick_id, base) {
        Some(b) => b,
        None => {
            eprintln!("Invalid brick type ID: {}", brick_id);
            return false;
        }
    };

    let svg_data = brick.to_svg();
    save_file("brick.svg", svg_data.as_bytes(), "image/svg+xml")
}

/// Platform-specific file saving
#[cfg(target_arch = "wasm32")]
fn save_file(filename: &str, data: &[u8], mime_type: &str) -> bool {
    use wasm_bindgen::JsCast;

    let window = match web_sys::window() {
        Some(w) => w,
        None => {
            eprintln!("No window object available");
            return false;
        }
    };

    let document = match window.document() {
        Some(d) => d,
        None => {
            eprintln!("No document object available");
            return false;
        }
    };

    // Create a Blob from the data
    let mut blob_opts = BlobPropertyBag::new();
    blob_opts.type_(mime_type);

    let array = js_sys::Uint8Array::from(data);
    let array_val: &JsValue = array.as_ref();
    let parts = js_sys::Array::new();
    parts.push(array_val);

    let blob = match Blob::new_with_u8_array_sequence_and_options(&parts, &blob_opts) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("Error creating blob: {:?}", e);
            return false;
        }
    };

    // Create a download URL
    let url = match Url::create_object_url_with_blob(&blob) {
        Ok(u) => u,
        Err(e) => {
            eprintln!("Error creating object URL: {:?}", e);
            return false;
        }
    };

    // Create a temporary anchor element and trigger download
    let anchor = match document.create_element("a") {
        Ok(a) => a,
        Err(e) => {
            eprintln!("Error creating anchor element: {:?}", e);
            return false;
        }
    };

    let anchor: HtmlAnchorElement = match anchor.dyn_into() {
        Ok(a) => a,
        Err(_) => {
            eprintln!("Failed to cast to HtmlAnchorElement");
            return false;
        }
    };

    anchor.set_href(&url);
    anchor.set_download(filename);
    anchor.style().set_property("display", "none").ok();

    if let Err(e) = document.body().unwrap().append_child(&anchor) {
        eprintln!("Error appending anchor to body: {:?}", e);
        return false;
    }

    anchor.click();

    if let Err(e) = document.body().unwrap().remove_child(&anchor) {
        eprintln!("Error removing anchor from body: {:?}", e);
    }

    let _ = Url::revoke_object_url(&url);

    true
}

/// Platform-specific file saving (native)
#[cfg(not(target_arch = "wasm32"))]
fn save_file(filename: &str, data: &[u8], _mime_type: &str) -> bool {
    match fs::write(filename, data) {
        Ok(_) => {
            println!("File saved as: {}", filename);
            true
        }
        Err(e) => {
            eprintln!("Error saving file: {}", e);
            false
        }
    }
}
