#[cfg(target_arch = "wasm32")]
use crate::app::editors::brick::BrickEditor;
#[cfg(target_arch = "wasm32")]
use crate::app::editors::tutorial::TutorialEditor;
#[cfg(target_arch = "wasm32")]
use crate::components::sidebar::Sidebar;
#[cfg(target_arch = "wasm32")]
use crate::components::icon_button::IconButton;
#[cfg(target_arch = "wasm32")]
use crate::style;
#[cfg(target_arch = "wasm32")]
use crate::interfaces::brick::BrickState;
#[cfg(target_arch = "wasm32")]
use crate::interfaces::catalog;
#[cfg(target_arch = "wasm32")]
use crate::interfaces::ninepatch;
#[cfg(target_arch = "wasm32")]
use crate::interfaces::tutorial::{tutorial_from_states, tutorial_png_bytes, TutorialAction, TutorialViewState};
#[cfg(target_arch = "wasm32")]
use crate::interfaces::utility;
#[cfg(target_arch = "wasm32")]
use shared::tutorial::Tutorial;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::closure::Closure;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use web_sys::{Blob, BlobPropertyBag, HtmlAnchorElement, Url};
#[cfg(target_arch = "wasm32")]
use yew::prelude::*;

#[cfg(target_arch = "wasm32")]
const ICON_UPLOAD: &str = include_str!("../res/upload.svg");
#[cfg(target_arch = "wasm32")]
const ICON_DOWNLOAD: &str = include_str!("../res/download.svg");
#[cfg(target_arch = "wasm32")]
const ICON_EXPORT_ALL_BRICKS: &str = include_str!("../res/zipfolder.svg");
#[cfg(target_arch = "wasm32")]
const ICON_EXPORT_NINEPATCH: &str = "<span class='icon-text'>9</span>";
#[cfg(target_arch = "wasm32")]
const ICON_LOADING: &str = "<svg class=\"icon-spinner\" xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 24 24\" width=\"24px\" height=\"24px\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\" stroke-linecap=\"round\" stroke-linejoin=\"round\" aria-hidden=\"true\"><path d=\"M12 3a9 9 0 1 0 9 9\"/></svg>";
#[cfg(target_arch = "wasm32")]
const ICON_MENU: &str = "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 24 24\" width=\"24px\" height=\"24px\" fill=\"currentColor\"><rect x=\"4\" y=\"5\" width=\"16\" height=\"2\"/><rect x=\"4\" y=\"11\" width=\"16\" height=\"2\"/><rect x=\"4\" y=\"17\" width=\"16\" height=\"2\"/></svg>";
#[cfg(target_arch = "wasm32")]
const ICON_UNDO: &str = include_str!("../res/undo.svg");
#[cfg(target_arch = "wasm32")]
const ICON_REDO: &str = include_str!("../res/redo.svg");
#[cfg(target_arch = "wasm32")]
const ICON_HELP: &str = "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 24 24\" width=\"18px\" height=\"18px\" fill=\"currentColor\"><path d=\"M9 21h6v-1H9zm3-20a7 7 0 0 0-4 12.75V17a1 1 0 0 0 1 1h6a1 1 0 0 0 1-1v-3.25A7 7 0 0 0 12 1zm2.4 11.55-.4.3V15h-4v-1.5c0-1.2.58-2.32 1.55-3l.55-.4a1.97 1.97 0 0 0 .9-1.67A2.05 2.05 0 0 0 10.5 6.5 2.07 2.07 0 0 0 8.5 8H7a3.5 3.5 0 0 1 7 0c0 1.17-.57 2.28-1.6 2.98z\"/></svg>";
#[cfg(target_arch = "wasm32")]
const ICON_ABOUT: &str = include_str!("../res/about.svg");
#[cfg(target_arch = "wasm32")]
const ICON_ADD_BRICK: &str = include_str!("../res/addBrick.svg");
#[cfg(target_arch = "wasm32")]
const ICON_DELETE: &str = include_str!("../res/delete.svg");
#[cfg(target_arch = "wasm32")]
const ICON_EDIT: &str = include_str!("../res/edit.svg");
#[cfg(target_arch = "wasm32")]
const ICON_PREVIEW: &str = include_str!("../res/preview.svg");
#[cfg(target_arch = "wasm32")]
const ICON_EDIT_SQUARE: &str = include_str!("../res/editsquare.svg");
#[cfg(target_arch = "wasm32")]
const ICON_BRICK_CATALOG: &str = include_str!("../res/brickcatalog.svg");
#[cfg(target_arch = "wasm32")]
const HELP_DOCS_URL: &str = "https://catrobat.org/docs/brickdocumentation/";
#[cfg(target_arch = "wasm32")]
const HELP_CONTACT_URL: &str = "https://developer.catrobat.org/pages/legal/imprint/";

#[cfg(target_arch = "wasm32")]
const SUN_ICON: &str = include_str!("../res/sun.svg");
#[cfg(target_arch = "wasm32")]
const MOON_ICON: &str = include_str!("../res/moon.svg");

#[cfg(target_arch = "wasm32")]
fn get_document() -> web_sys::Document {
    gloo::utils::document()
}

#[cfg(target_arch = "wasm32")]
fn themed_menu_icon(svg: &str) -> AttrValue {
    AttrValue::from(svg.replace("fill=\"#1f1f1f\"", "fill=\"currentColor\""))
}

#[cfg(target_arch = "wasm32")]
fn trigger_download(url: &str, filename: &str) {
    if let Some(window) = web_sys::window() {
        if let Some(document) = window.document() {
            if let Some(anchor) = document
                .create_element("a")
                .ok()
                .and_then(|e| e.dyn_into::<HtmlAnchorElement>().ok())
            {
                anchor.set_href(url);
                anchor.set_download(filename);
                let _ = anchor.click();
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
fn load_saved_theme() -> bool {
    gloo::utils::window()
        .local_storage()
        .ok()
        .flatten()
        .and_then(|s| s.get_item("theme").ok().flatten())
        .map(|v| v == "light")
        .unwrap_or(false)
}

#[cfg(target_arch = "wasm32")]
fn apply_theme(light: bool) {
    if let Some(el) = get_document().document_element() {
        if light {
            let _ = el.set_attribute("data-theme", "light");
        } else {
            let _ = el.remove_attribute("data-theme");
        }
    }
    if let Ok(Some(storage)) = gloo::utils::window().local_storage() {
        let _ = storage.set_item("theme", if light { "light" } else { "dark" });
    }
}

#[cfg(target_arch = "wasm32")]
fn parse_svg_viewbox_dimensions(svg: &str) -> Option<(u32, u32)> {
    let marker = "viewBox=\"0 0 ";
    let start = svg.find(marker)? + marker.len();
    let end = svg[start..].find('"')?;
    let mut parts = svg[start..start + end].split_whitespace();
    let width = parts.next()?.parse::<f32>().ok()?;
    let height = parts.next()?.parse::<f32>().ok()?;
    Some((width.ceil() as u32, height.ceil() as u32))
}

#[cfg(target_arch = "wasm32")]
fn position_svg_root(mut svg: String, y_offset: u32) -> String {
    if let Some(open_end) = svg.find('>') {
        svg.insert_str(open_end, &format!(" x=\"0\" y=\"{}\"", y_offset));
    }
    svg
}

#[cfg(target_arch = "wasm32")]
fn export_tutorial_svg(states: &[BrickState]) -> Result<String, String> {
    let mut content = String::new();
    let mut y_offset = 0u32;
    let mut max_width = 0u32;

    for state in states {
        let svg = state.clone().get_svg();
        let (width, height) = parse_svg_viewbox_dimensions(&svg).unwrap_or((0, 0));
        max_width = max_width.max(width);
        let positioned = position_svg_root(svg, y_offset);
        content.push_str(&positioned);
        y_offset += height;
    }

    let width = max_width.max(1);
    let height = y_offset.max(1);
    Ok(format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {width} {height}\" width=\"{width}\" height=\"{height}\">{content}</svg>",
        width = width,
        height = height,
        content = content
    ))
}

#[cfg(target_arch = "wasm32")]
#[derive(Clone, Default, PartialEq)]
struct AppSnapshot {
    brick: BrickState,
    tutorial: TutorialViewState,
}

#[cfg(target_arch = "wasm32")]
#[function_component(App)]
fn app() -> Html {
    let brick = use_reducer(BrickState::default);
    let tutorial = use_reducer(TutorialViewState::default);

    let export_mode = use_state(|| false);
    let all_bricks_url = use_state(|| Option::<String>::None);
    let all_bricks_rendering = use_state(|| false);
    let ninepatch_url = use_state(|| Option::<String>::None);
    let ninepatch_rendering = use_state(|| false);
    let menu_open = use_state(|| false);
    let help_submenu_open = use_state(|| false);
    let explanation_modal_open = use_state(|| false);
    let history = use_state(|| vec![AppSnapshot::default()]);
    let history_index = use_state(|| 0usize);
    let restoring_history = use_mut_ref(|| 0usize);
    let light = use_state(|| {
        let saved = load_saved_theme();
        apply_theme(saved);
        saved
    });

    {
        let history = history.clone();
        let history_index = history_index.clone();
        let restoring_history = restoring_history.clone();
        let snapshot = AppSnapshot {
            brick: (*brick).clone(),
            tutorial: (*tutorial).clone(),
        };
        use_effect_with(snapshot, move |snapshot| {
            let pending_restores = *restoring_history.borrow();
            if pending_restores > 0 {
                *restoring_history.borrow_mut() = pending_restores - 1;
            } else {
                let current_index = *history_index;
                let mut next = (*history).clone();
                if !next
                    .get(current_index)
                    .map(|entry| entry == snapshot)
                    .unwrap_or(false)
                {
                    next.truncate(current_index + 1);
                    next.push(snapshot.clone());
                    history.set(next);
                    history_index.set(current_index + 1);
                }
            }
            || ()
        });
    }

    let tutorial_len = (*tutorial).tutorial.content.len();
    let export_selection = use_state(|| vec![false; tutorial_len]);
    {
        let export_selection = export_selection.clone();
        use_effect_with(tutorial_len, move |len| {
            export_selection.set(vec![false; *len]);
            || ()
        });
    }

    let on_toggle_export = {
        let export_selection = export_selection.clone();
        Callback::from(move |index: usize| {
            let mut next = (*export_selection).clone();
            if index < next.len() {
                next[index] = !next[index];
                export_selection.set(next);
            }
        })
    };

    let on_export_select_all = {
        let export_selection = export_selection.clone();
        Callback::from(move |_: MouseEvent| {
            let next = vec![true; (*export_selection).len()];
            export_selection.set(next);
        })
    };

    let on_export_clear = {
        let export_selection = export_selection.clone();
        Callback::from(move |_: MouseEvent| {
            let next = vec![false; (*export_selection).len()];
            export_selection.set(next);
        })
    };

    let brick_dispatcher = brick.dispatcher();
    let tutorial_dispatcher = tutorial.dispatcher();

    let import_json = {
        let tutorial_dispatcher = tutorial_dispatcher.clone();
        Callback::from(move |_: ()| {
            let tutorial_dispatcher = tutorial_dispatcher.clone();
            utility::upload_json(Callback::from(move |text: String| {
                if Tutorial::from_json(&text).is_ok() {
                    tutorial_dispatcher.dispatch(TutorialAction::LoadJson(text));
                    return;
                }
                if let Ok(brick) = BrickState::from_json(&text) {
                    tutorial_dispatcher.dispatch(TutorialAction::InsertAfterSelected(vec![brick]));
                    return;
                }
                web_sys::console::error_1(
                    &"Import error: unsupported JSON (not a brick or tutorial)".into(),
                );
            }));
        })
    };

    let on_export_selected_json = {
        let tutorial = tutorial.clone();
        let export_selection = export_selection.clone();
        let export_mode = export_mode.clone();
        Callback::from(move |_: MouseEvent| {
            let bricks = (*tutorial).get_brick_state_list();
            let selection = (*export_selection).clone();
            let selected: Vec<BrickState> = bricks
                .into_iter()
                .enumerate()
                .filter_map(|(index, brick)| {
                    if selection.get(index).copied().unwrap_or(false) {
                        Some(brick)
                    } else {
                        None
                    }
                })
                .collect();
            if selected.is_empty() {
                web_sys::console::error_1(&"Export error: no bricks selected".into());
                return;
            }
            let tutorial = tutorial_from_states(&selected, "Exported tutorial");
            let json = tutorial.to_json();
            if let Err(e) = utility::download_json(&json, "tutorial.json") {
                web_sys::console::error_1(&format!("Export error: {e}").into());
                return;
            }
            export_mode.set(false);
        })
    };

    let on_export_selected_png = {
        let tutorial = tutorial.clone();
        let export_selection = export_selection.clone();
        let export_mode = export_mode.clone();
        Callback::from(move |_: MouseEvent| {
            let bricks = (*tutorial).get_brick_state_list();
            let selection = (*export_selection).clone();
            let selected: Vec<BrickState> = bricks
                .into_iter()
                .enumerate()
                .filter_map(|(index, brick)| {
                    if selection.get(index).copied().unwrap_or(false) {
                        Some(brick)
                    } else {
                        None
                    }
                })
                .collect();
            if selected.is_empty() {
                web_sys::console::error_1(&"Export error: no bricks selected".into());
                return;
            }
            match tutorial_png_bytes(&selected, 1920) {
                Ok(data) => {
                    if let Err(e) = utility::download_png(&data, "tutorial.png") {
                        web_sys::console::error_1(&format!("PNG error: {e}").into());
                        return;
                    }
                }
                Err(e) => {
                    web_sys::console::error_1(&format!("PNG render error: {e}").into());
                    return;
                }
            }
            export_mode.set(false);
        })
    };

    let on_export_selected_svg = {
        let tutorial = tutorial.clone();
        let export_selection = export_selection.clone();
        let export_mode = export_mode.clone();
        Callback::from(move |_: MouseEvent| {
            let bricks = (*tutorial).get_brick_state_list();
            let selection = (*export_selection).clone();
            let selected: Vec<BrickState> = bricks
                .into_iter()
                .enumerate()
                .filter_map(|(index, brick)| {
                    if selection.get(index).copied().unwrap_or(false) {
                        Some(brick)
                    } else {
                        None
                    }
                })
                .collect();
            if selected.is_empty() {
                web_sys::console::error_1(&"Export error: no bricks selected".into());
                return;
            }
            match export_tutorial_svg(&selected) {
                Ok(svg) => {
                    if let Err(e) = utility::download_svg(&svg, "tutorial.svg") {
                        web_sys::console::error_1(&format!("SVG error: {e}").into());
                        return;
                    }
                }
                Err(e) => {
                    web_sys::console::error_1(&format!("SVG render error: {e}").into());
                    return;
                }
            }
            export_mode.set(false);
        })
    };

    let export_all_bricks_zip = {
        let all_bricks_url = all_bricks_url.clone();
        let all_bricks_rendering = all_bricks_rendering.clone();
        Callback::from(move |_: MouseEvent| {
            if *all_bricks_rendering {
                return;
            }
            if let Some(url) = (*all_bricks_url).clone() {
                trigger_download(&url, "all_bricks.zip");
                return;
            }
            all_bricks_rendering.set(true);
            let all_bricks_url = all_bricks_url.clone();
            let all_bricks_rendering_done = all_bricks_rendering.clone();
            let callback = Closure::once(move || {
                match catalog::render_all_bricks_zip_bytes(192) {
                    Ok(data) => {
                        let uint8 = js_sys::Uint8Array::from(&data[..]);
                        let parts = js_sys::Array::new();
                        parts.push(&uint8.buffer());
                        let mut opts = BlobPropertyBag::new();
                        #[allow(deprecated)]
                        opts.type_("application/zip");
                        match Blob::new_with_buffer_source_sequence_and_options(&parts, &opts) {
                            Ok(blob) => match Url::create_object_url_with_blob(&blob) {
                                Ok(url) => {
                                    all_bricks_url.set(Some(url.clone()));
                                    trigger_download(&url, "all_bricks.zip");
                                }
                                Err(e) => web_sys::console::error_1(
                                    &format!("All bricks URL error: {e:?}").into(),
                                ),
                            },
                            Err(e) => web_sys::console::error_1(
                                &format!("All bricks blob error: {e:?}").into(),
                            ),
                        }
                    }
                    Err(e) => {
                        web_sys::console::error_1(&format!("All bricks render error: {e}").into())
                    }
                }
                all_bricks_rendering_done.set(false);
            });
            if let Some(window) = web_sys::window() {
                let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                    callback.as_ref().unchecked_ref(),
                    0,
                );
                callback.forget();
            } else {
                all_bricks_rendering.set(false);
            }
        })
    };

    let export_ninepatch_zip = {
        let ninepatch_url = ninepatch_url.clone();
        let ninepatch_rendering = ninepatch_rendering.clone();
        Callback::from(move |_: MouseEvent| {
            if *ninepatch_rendering {
                return;
            }
            if let Some(url) = (*ninepatch_url).clone() {
                trigger_download(&url, "ninepatch_bricks.zip");
                return;
            }
            ninepatch_rendering.set(true);
            let ninepatch_url = ninepatch_url.clone();
            let ninepatch_rendering_done = ninepatch_rendering.clone();
            let callback = Closure::once(move || {
                match ninepatch::render_ninepatch_zip_bytes() {
                    Ok(data) => {
                        let uint8 = js_sys::Uint8Array::from(&data[..]);
                        let parts = js_sys::Array::new();
                        parts.push(&uint8.buffer());
                        let mut opts = BlobPropertyBag::new();
                        #[allow(deprecated)]
                        opts.type_("application/zip");
                        match Blob::new_with_buffer_source_sequence_and_options(&parts, &opts) {
                            Ok(blob) => match Url::create_object_url_with_blob(&blob) {
                                Ok(url) => {
                                    ninepatch_url.set(Some(url.clone()));
                                    trigger_download(&url, "ninepatch_bricks.zip");
                                }
                                Err(e) => web_sys::console::error_1(
                                    &format!("Ninepatch URL error: {e:?}").into(),
                                ),
                            },
                            Err(e) => web_sys::console::error_1(
                                &format!("Ninepatch blob error: {e:?}").into(),
                            ),
                        }
                    }
                    Err(e) => {
                        web_sys::console::error_1(&format!("Ninepatch render error: {e}").into())
                    }
                }
                ninepatch_rendering_done.set(false);
            });
            if let Some(window) = web_sys::window() {
                let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                    callback.as_ref().unchecked_ref(),
                    0,
                );
                callback.forget();
            } else {
                ninepatch_rendering.set(false);
            }
        })
    };

    let on_enter_export_mode = {
        let export_mode = export_mode.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            e.stop_propagation();
            export_mode.set(true);
        })
    };

    let on_exit_export_mode = {
        let export_mode = export_mode.clone();
        Callback::from(move |_: MouseEvent| export_mode.set(false))
    };

    let on_import = {
        let import_json = import_json.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            e.stop_propagation();
            import_json.emit(());
        })
    };

    let toggle_theme = {
        let light = light.clone();
        Callback::from(move |_: MouseEvent| {
            let new_val = !*light;
            apply_theme(new_val);
            light.set(new_val);
        })
    };

    let on_menu = {
        let menu_open = menu_open.clone();
        let help_submenu_open = help_submenu_open.clone();
        Callback::from(move |_: MouseEvent| {
            let next_open = !*menu_open;
            menu_open.set(next_open);
            if !next_open {
                help_submenu_open.set(false);
            }
        })
    };

    let on_menu_mouse_leave = {
        let menu_open = menu_open.clone();
        let help_submenu_open = help_submenu_open.clone();
        Callback::from(move |_: MouseEvent| {
            menu_open.set(false);
            help_submenu_open.set(false);
        })
    };
    };

    let on_menu_mouse_leave = {
        let menu_open = menu_open.clone();
        Callback::from(move |_: MouseEvent| menu_open.set(false))
    };

    let on_undo = {
        let history = history.clone();
        let history_index = history_index.clone();
        let restoring_history = restoring_history.clone();
        let brick = brick.clone();
        let tutorial = tutorial.clone();
        let brick = brick.clone();
        let tutorial = tutorial.clone();
        let brick_dispatcher = brick_dispatcher.clone();
        let tutorial_dispatcher = tutorial_dispatcher.clone();
        Callback::from(move |_: MouseEvent| {
            let current_index = *history_index;
            if current_index == 0 {
                return;
            }
            let target_index = current_index - 1;
            if let Some(snapshot) = (*history).get(target_index).cloned() {
                let pending_restores = usize::from(*brick != snapshot.brick)
                    + usize::from(*tutorial != snapshot.tutorial);
                *restoring_history.borrow_mut() = pending_restores;
                let pending_restores = usize::from(*brick != snapshot.brick)
                    + usize::from(*tutorial != snapshot.tutorial);
                *restoring_history.borrow_mut() = pending_restores;
                brick_dispatcher.dispatch(crate::interfaces::brick::StateAction::Set(snapshot.brick));
                tutorial_dispatcher.dispatch(TutorialAction::Restore(snapshot.tutorial));
                history_index.set(target_index);
            }
        })
    };

    let on_redo = {
        let history = history.clone();
        let history_index = history_index.clone();
        let restoring_history = restoring_history.clone();
        let brick = brick.clone();
        let tutorial = tutorial.clone();
        let brick = brick.clone();
        let tutorial = tutorial.clone();
        let brick_dispatcher = brick_dispatcher.clone();
        let tutorial_dispatcher = tutorial_dispatcher.clone();
        Callback::from(move |_: MouseEvent| {
            let current_index = *history_index;
            let target_index = current_index + 1;
            if let Some(snapshot) = (*history).get(target_index).cloned() {
                let pending_restores = usize::from(*brick != snapshot.brick)
                    + usize::from(*tutorial != snapshot.tutorial);
                *restoring_history.borrow_mut() = pending_restores;
                let pending_restores = usize::from(*brick != snapshot.brick)
                    + usize::from(*tutorial != snapshot.tutorial);
                *restoring_history.borrow_mut() = pending_restores;
                brick_dispatcher.dispatch(crate::interfaces::brick::StateAction::Set(snapshot.brick));
                tutorial_dispatcher.dispatch(TutorialAction::Restore(snapshot.tutorial));
                history_index.set(target_index);
            }
        })
    };

    let on_help = {
        let help_submenu_open = help_submenu_open.clone();
        Callback::from(move |_: MouseEvent| {
            help_submenu_open.set(!*help_submenu_open);
        })
    };

    let on_help_link_click = {
        let menu_open = menu_open.clone();
        let help_submenu_open = help_submenu_open.clone();
        Callback::from(move |_: MouseEvent| {
            menu_open.set(false);
            help_submenu_open.set(false);
        })
    };

    let on_open_explanation = {
        let menu_open = menu_open.clone();
        let help_submenu_open = help_submenu_open.clone();
        let explanation_modal_open = explanation_modal_open.clone();
        Callback::from(move |_: MouseEvent| {
            menu_open.set(false);
            help_submenu_open.set(false);
            explanation_modal_open.set(true);
        })
    };

    let on_close_explanation = {
        let explanation_modal_open = explanation_modal_open.clone();
        Callback::from(move |_: MouseEvent| explanation_modal_open.set(false))
    };

    let on_about = {
        let menu_open = menu_open.clone();
        let help_submenu_open = help_submenu_open.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(window) = web_sys::window() {
                let _ = window.alert_with_message(
                    "About BrickCreator\n\nBrickCreator is a website built by and for Catrobat Pocket Code users, especially educators who teach Pocket Code.\n\nIt allows users to create PNG, SVG, and JSON files that can be directly used in presentations, teaching materials, and tutorials.\n\nCatrobat is an open-source platform with contributors from all over the world. For more information, visit: https://catrobat.org/about",
                );
            }
            menu_open.set(false);
            help_submenu_open.set(false);
        })
    };

    let tutorial_bricks = (*tutorial).get_brick_state_list();
    let selected_count = (*export_selection)
        .iter()
        .filter(|selected| **selected)
        .count();
    let can_undo = *history_index > 0;
    let can_redo = *history_index + 1 < (*history).len();
    html! {
        <div class={style::APP_ROOT}>
            <div class={style::APP_TOOLBAR}>
            <div class={style::APP_TOOLBAR_GROUP}>
                <div class={style::TOOLBAR_MENU}>
                    <IconButton
                        icon={Html::from_html_unchecked(AttrValue::from(ICON_MENU))}
                        title="Menu"
                        label="Menu"
                        onclick={on_menu.clone()}
                    />
                    if *menu_open {
                        <div class={style::TOOLBAR_MENU_DROPDOWN} onmouseleave={on_menu_mouse_leave}>
                            <button
                                class={style::TOOLBAR_MENU_ITEM}
                                type="button"
                                onclick={on_undo.clone()}
                                disabled={!can_undo}
                            >
                                <span class={style::TOOLBAR_MENU_ICON} aria-hidden="true">
                                    {Html::from_html_unchecked(themed_menu_icon(ICON_UNDO))}
                                </span>
                                <span>{"Undo"}</span>
                            </button>
                            <button
                                class={style::TOOLBAR_MENU_ITEM}
                                type="button"
                                onclick={on_redo.clone()}
                                disabled={!can_redo}
                            >
                                <span class={style::TOOLBAR_MENU_ICON} aria-hidden="true">
                                    {Html::from_html_unchecked(themed_menu_icon(ICON_REDO))}
                                </span>
                                <span>{"Redo"}</span>
                            </button>
                            <div class="toolbar-menu__submenu-wrap">
                                <button
                                    class={classes!("toolbar-menu__item", "toolbar-menu__item--submenu-toggle", (*help_submenu_open).then_some("toolbar-menu__item--active"))}
                                    type="button"
                                    onclick={on_help.clone()}
                                >
                                    <span class="toolbar-menu__icon" aria-hidden="true">
                                        {Html::from_html_unchecked(AttrValue::from(ICON_HELP))}
                                    </span>
                                    <span>{"Help"}</span>
                                    <span class="toolbar-menu__caret" aria-hidden="true">{"›"}</span>
                                </button>
                                if *help_submenu_open {
                                    <div class="toolbar-submenu">
                                        <button
                                            class="toolbar-submenu__item"
                                            type="button"
                                            onclick={on_open_explanation.clone()}
                                        >
                                            {"Icon Explanation"}
                                        </button>
                                        <a
                                            class="toolbar-submenu__item"
                                            href={HELP_CONTACT_URL}
                                            target="_blank"
                                            rel="noopener noreferrer"
                                            onclick={on_help_link_click.clone()}
                                        >
                                            {"Contact information"}
                                        </a>
                                        <a
                                            class="toolbar-submenu__item"
                                            href={HELP_DOCS_URL}
                                            target="_blank"
                                            rel="noopener noreferrer"
                                            onclick={on_help_link_click.clone()}
                                        >
                                            {"Brick Documentation"}
                                        </a>
                                    </div>
                                }
                            </div>
                            <button
                                class={style::TOOLBAR_MENU_ITEM}
                                type="button"
                                onclick={on_about.clone()}
                            >
                                <span class={style::TOOLBAR_MENU_ICON} aria-hidden="true">
                                    {Html::from_html_unchecked(AttrValue::from(ICON_HELP))}
                                </span>
                                <span>{"About"}</span>
                            </button>
                        </div>
                    }
                </div>
                <IconButton
                    icon={Html::from_html_unchecked(AttrValue::from(if *light { MOON_ICON } else { SUN_ICON }))}
                    title={if *light { "Switch to dark mode" } else { "Switch to light mode" }}
                    label="Theme"
                    onclick={toggle_theme.clone()}
                    />
                </div>
            <div class={style::APP_TOOLBAR_GROUP}>
                <IconButton
                    icon={Html::from_html_unchecked(AttrValue::from(ICON_UPLOAD))}
                    title="Import JSON"
                    label="Import"
                    onclick={on_import.clone()}
                />
                <IconButton
                    icon={Html::from_html_unchecked(AttrValue::from(ICON_DOWNLOAD))}
                    title="Export"
                    label="Export"
                    onclick={on_enter_export_mode.clone()}
                />
                <IconButton
                    icon={Html::from_html_unchecked(AttrValue::from(if *all_bricks_rendering { ICON_LOADING } else { ICON_EXPORT_ALL_BRICKS }))}
                    title={if *all_bricks_rendering { "Rendering All Bricks ZIP..." } else { "Render All Bricks ZIP" }}
                    label="All ZIP"
                    onclick={export_all_bricks_zip.clone()}
                    disabled={*all_bricks_rendering}
                />
                <IconButton
                    icon={Html::from_html_unchecked(AttrValue::from(if *ninepatch_rendering { ICON_LOADING } else { ICON_EXPORT_NINEPATCH }))}
                    title={if *ninepatch_rendering { "Rendering 9-patch ZIP..." } else { "Render 9-patch ZIP" }}
                    label="9-patch ZIP"
                    onclick={export_ninepatch_zip.clone()}
                    disabled={*ninepatch_rendering}
                />
            </div>
        </div>
            <div class={style::APP_MAIN}>
                <div class={style::APP_EDITOR_WRAP}>
                    <BrickEditor
                        brick={(*brick).clone()}
                        dispatcher={brick_dispatcher.clone()}
                        tutorial_dispatcher={tutorial_dispatcher.clone()}
                    />
                </div>
                <Sidebar>
                    <TutorialEditor
                        brick={(*brick).clone()}
                        brick_dispatcher={brick_dispatcher.clone()}
                        tutorial={(*tutorial).clone()}
                        tutorial_dispatcher={tutorial_dispatcher.clone()}
                        export_selection={(*export_selection).clone()}
                        on_toggle_export={on_toggle_export.clone()}
                        export_mode={*export_mode}
                        selected_count={selected_count}
                        total_bricks={tutorial_bricks.len()}
                        on_export_select_all={on_export_select_all.clone()}
                        on_export_clear={on_export_clear.clone()}
                        on_export_json={on_export_selected_json.clone()}
                        on_export_png={on_export_selected_png.clone()}
                        on_export_svg={on_export_selected_svg.clone()}
                        on_exit_export={on_exit_export_mode.clone()}
                    />
                </Sidebar>
            </div>
            if *explanation_modal_open {
                <Modal
                    title="Icon Explanation"
                    hint="What each toolbar icon does"
                    class={classes!("explanation-modal")}
                    on_close={on_close_explanation.clone()}
                >
                    <div class="explanation-modal__list">
                        <div class="explanation-modal__row">
                            <div class="explanation-modal__icon">
                                {Html::from_html_unchecked(AttrValue::from(ICON_MENU))}
                            </div>
                            <div class="explanation-modal__text">
                                {"Open the main menu, where you can access undo, redo, Help, and About."}
                            </div>
                        </div>
                        <div class="explanation-modal__row">
                            <div class="explanation-modal__icon">
                                {Html::from_html_unchecked(AttrValue::from(SUN_ICON))}
                            </div>
                            <div class="explanation-modal__text">
                                {"Switch to light mode."}
                            </div>
                        </div>
                        <div class="explanation-modal__row">
                            <div class="explanation-modal__icon">
                                {Html::from_html_unchecked(AttrValue::from(MOON_ICON))}
                            </div>
                            <div class="explanation-modal__text">
                                {"Switch to dark mode."}
                            </div>
                        </div>
                        <div class="explanation-modal__row">
                            <div class="explanation-modal__icon">
                                {Html::from_html_unchecked(themed_menu_icon(ICON_ADD_BRICK))}
                            </div>
                            <div class="explanation-modal__text">
                                {"Add the current brick from the brick editor to the tutorial editor."}
                            </div>
                        </div>
                        <div class="explanation-modal__row">
                            <div class="explanation-modal__icon">
                                {Html::from_html_unchecked(themed_menu_icon(ICON_DELETE))}
                            </div>
                            <div class="explanation-modal__text">
                                {"Delete the selected brick from the tutorial editor."}
                            </div>
                        </div>
                        <div class="explanation-modal__row">
                            <div class="explanation-modal__icon">
                                {Html::from_html_unchecked(themed_menu_icon(ICON_EDIT))}
                            </div>
                            <div class="explanation-modal__text">
                                {"Apply the current changes from the brick editor to the selected brick in the tutorial editor."}
                            </div>
                        </div>
                        <div class="explanation-modal__row">
                            <div class="explanation-modal__icon">
                                {Html::from_html_unchecked(AttrValue::from(ICON_PREVIEW))}
                            </div>
                            <div class="explanation-modal__text">
                                {"Show a preview of the rendered bricks in the tutorial editor."}
                            </div>
                        </div>
                        <div class="explanation-modal__row">
                            <div class="explanation-modal__icon">
                                {Html::from_html_unchecked(themed_menu_icon(ICON_EDIT_SQUARE))}
                            </div>
                            <div class="explanation-modal__text">
                                {"Return from preview mode to the tutorial editor."}
                            </div>
                        </div>
                        <div class="explanation-modal__row">
                            <div class="explanation-modal__icon">
                                {Html::from_html_unchecked(AttrValue::from(ICON_BRICK_CATALOG))}
                            </div>
                            <div class="explanation-modal__text">
                                {"Open the brick catalog. Double-click a prebuilt brick to add it to the tutorial editor."}
                            </div>
                        </div>
                        <div class="explanation-modal__row">
                            <div class="explanation-modal__icon explanation-modal__icon--text">
                                {"↑ ↓"}
                            </div>
                            <div class="explanation-modal__text">
                                {"Move the selected brick up or down in the tutorial editor."}
                            </div>
                        </div>
                        <div class="explanation-modal__row">
                            <div class="explanation-modal__icon">
                                {Html::from_html_unchecked(AttrValue::from(ICON_UPLOAD))}
                            </div>
                            <div class="explanation-modal__text">
                                {"Import a JSON file containing either a single brick or a full tutorial."}
                            </div>
                        </div>
                        <div class="explanation-modal__row">
                            <div class="explanation-modal__icon">
                                {Html::from_html_unchecked(AttrValue::from(ICON_DOWNLOAD))}
                            </div>
                            <div class="explanation-modal__text">
                                {"Choose which bricks to export and select the output format. Use Select All, Clear Selection, and Done to finish. The checkboxes to the left of the tutorial editor show which bricks are selected."}
                            </div>
                        </div>
                        <div class="explanation-modal__row">
                            <div class="explanation-modal__icon">
                                {Html::from_html_unchecked(themed_menu_icon(ICON_EXPORT_ALL_BRICKS))}
                            </div>
                            <div class="explanation-modal__text">
                                {"Download a ZIP file containing all bricks from the catalog."}
                            </div>
                        </div>
                        <div class="explanation-modal__row">
                            <div class="explanation-modal__icon explanation-modal__icon--text">
                                {"9"}
                            </div>
                            <div class="explanation-modal__text">
                                {"Download the 9-patch ZIP file."}
                            </div>
                        </div>
                    </div>
                </Modal>
            }
        </div>
    }
}

#[cfg(target_arch = "wasm32")]
pub fn mount_app() {
    yew::Renderer::<App>::new().render();
}
