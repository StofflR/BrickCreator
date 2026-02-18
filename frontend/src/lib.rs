slint::include_modules!();

pub mod brick_view;

use shared::color::*;
use slint::{VecModel};
use std::rc::Rc;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

pub fn setup_ui() -> Result<MainWindow, slint::PlatformError> {
    let ui = MainWindow::new()?;

    // Initialize brick types
    let brick_types = Rc::new(VecModel::from(vec![
        BrickTypeInfo {
            id: 0,
            name: "H0 Collapsed".into(),
            description: "Thin horizontal brick".into(),
        },
        BrickTypeInfo {
            id: 1,
            name: "H1 Base".into(),
            description: "Standard base brick - small".into(),
        },
        BrickTypeInfo {
            id: 2,
            name: "H1 Control".into(),
            description: "Control brick - small".into(),
        },
        BrickTypeInfo {
            id: 3,
            name: "H2 Base".into(),
            description: "Standard base brick - medium".into(),
        },
        BrickTypeInfo {
            id: 4,
            name: "H2 Control".into(),
            description: "Control brick - medium".into(),
        },
        BrickTypeInfo {
            id: 5,
            name: "H3 Base".into(),
            description: "Standard base brick - large".into(),
        },
    ]));

    // Initialize color schemes
    let color_schemes = Rc::new(VecModel::from(
        ALL_COLOR_SCHEMES
            .iter()
            .enumerate()
            .map(|(idx, scheme)| ColorSchemeInfo {
                id: idx as i32,
                name: scheme.name.into(),
                color: brick_view::parse_hex_color(scheme.color),
                shade: brick_view::parse_hex_color(scheme.shade),
                border: brick_view::parse_hex_color(scheme.border),
                text: brick_view::parse_hex_color(scheme.text),
            })
            .collect::<Vec<_>>(),
    ));

    ui.set_brick_types(brick_types.into());
    ui.set_color_schemes(color_schemes.into());
    ui.set_selected_brick_type(0);
    ui.set_selected_color_scheme(0);

    // Connect brick preview update callback via global
    ui.global::<BrickGlobals>().on_update_brick_preview(|brick_id, content, color_id, target_width, scroll_x, scroll_y| {
        brick_view::update_brick_preview(brick_id, content.as_str(), color_id as usize, target_width as u32, scroll_x, scroll_y)
    });

    // Connect save as PNG callback
    ui.global::<BrickGlobals>().on_save_as_png(|brick_id, content, color_id, scroll_x, scroll_y, width| {
        brick_view::save_as_png(brick_id, content.as_str(), color_id as usize, scroll_x, scroll_y, width as u32)
    });

    // Connect save as SVG callback
    ui.global::<BrickGlobals>().on_save_as_svg(|brick_id, content, color_id, scroll_x, scroll_y| {
        brick_view::save_as_svg(brick_id, content.as_str(), color_id as usize, scroll_x, scroll_y)
    });

    // Handle reset
    let ui_weak = ui.as_weak();
    ui.on_reset(move || {
        let ui = ui_weak.unwrap();
        ui.set_brick_content("".into());
        ui.set_selected_brick_type(0);
        ui.set_selected_color_scheme(0);
    });

    Ok(ui)
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn wasm_main() {
    // Set up panic hook for better error messages in the browser console
    console_error_panic_hook::set_once();
    
    let ui = setup_ui().expect("Failed to initialize UI");
    ui.run().expect("Failed to run UI");
}
