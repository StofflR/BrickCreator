#[cfg(target_arch = "wasm32")]
use crate::app::editors::brick::BrickEditor;
#[cfg(target_arch = "wasm32")]
use crate::app::editors::tutorial::TutorialEditor;
#[cfg(target_arch = "wasm32")]
use crate::components::sidebar::Sidebar;
#[cfg(target_arch = "wasm32")]
use crate::components::icon_button::IconButton;
#[cfg(target_arch = "wasm32")]
use crate::interfaces::brick::BrickState;
use crate::interfaces::catalog;
use crate::interfaces::ninepatch;
#[cfg(target_arch = "wasm32")]
#[cfg(target_arch = "wasm32")]
#[cfg(target_arch = "wasm32")]
use crate::interfaces::tutorial::{tutorial_from_states, tutorial_png_bytes, TutorialAction, TutorialViewState};
#[cfg(target_arch = "wasm32")]
use crate::interfaces::utility;
#[cfg(target_arch = "wasm32")]
use shared::tutorial::Tutorial;
use wasm_bindgen::JsCast;
use web_sys::{Blob, BlobPropertyBag, HtmlAnchorElement, Url};
#[cfg(target_arch = "wasm32")]
#[cfg(target_arch = "wasm32")]
#[cfg(target_arch = "wasm32")]
use yew::prelude::*;

#[cfg(target_arch = "wasm32")]
const ICON_UPLOAD: &str = include_str!("../res/upload.svg");
#[cfg(target_arch = "wasm32")]
const ICON_DOWNLOAD: &str = include_str!("../res/download.svg");
#[cfg(target_arch = "wasm32")]
const ICON_EXPORT_ALL_BRICKS: &str = "<span class='icon-text'>B</span>";
#[cfg(target_arch = "wasm32")]
const ICON_EXPORT_NINEPATCH: &str = "<span class='icon-text'>9</span>";

#[cfg(target_arch = "wasm32")]
const SUN_ICON: &str = include_str!("../res/sun.svg");
#[cfg(target_arch = "wasm32")]
const MOON_ICON: &str = include_str!("../res/moon.svg");

#[cfg(target_arch = "wasm32")]
fn get_document() -> web_sys::Document {
    gloo::utils::document()
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
#[function_component(App)]
fn app() -> Html {
    let brick = use_reducer(BrickState::default);
    let tutorial = use_reducer(TutorialViewState::default);

    let export_mode = use_state(|| false);
    let all_bricks_url = use_state(|| Option::<String>::None);
    let all_bricks_rendering = use_state(|| false);
    let ninepatch_url = use_state(|| Option::<String>::None);
    let ninepatch_rendering = use_state(|| false);
    let light = use_state(|| {
        let saved = load_saved_theme();
        apply_theme(saved);
        saved
    });

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

    let export_all_bricks_zip = {
        let all_bricks_url = all_bricks_url.clone();
        let all_bricks_rendering = all_bricks_rendering.clone();
        Callback::from(move |_: MouseEvent| {
            if *all_bricks_rendering {
                return;
            }
            if let Some(url) = (*all_bricks_url).clone() {
                if let Some(window) = web_sys::window() {
                    if let Some(document) = window.document() {
                        if let Some(anchor) = document.create_element("a").ok().and_then(|e| e.dyn_into::<HtmlAnchorElement>().ok()) {
                            anchor.set_href(&url);
                            anchor.set_download("all_bricks.zip");
                            let _ = anchor.click();
                            return;
                        }
                    }
                }
            }
            all_bricks_rendering.set(true);
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
                                if let Some(window) = web_sys::window() {
                                    if let Some(document) = window.document() {
                                        if let Some(anchor) = document.create_element("a").ok().and_then(|e| e.dyn_into::<HtmlAnchorElement>().ok()) {
                                            anchor.set_href(&url);
                                            anchor.set_download("all_bricks.zip");
                                            let _ = anchor.click();
                                        }
                                    }
                                }
                            }
                            Err(e) => web_sys::console::error_1(&format!("All bricks URL error: {e:?}").into()),
                        },
                        Err(e) => web_sys::console::error_1(&format!("All bricks blob error: {e:?}").into()),
                    }
                }
                Err(e) => web_sys::console::error_1(&format!("All bricks render error: {e}").into()),
            }
            all_bricks_rendering.set(false);
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
                if let Some(window) = web_sys::window() {
                    if let Some(document) = window.document() {
                        if let Some(anchor) = document.create_element("a").ok().and_then(|e| e.dyn_into::<HtmlAnchorElement>().ok()) {
                            anchor.set_href(&url);
                            anchor.set_download("ninepatch_bricks.zip");
                            let _ = anchor.click();
                            return;
                        }
                    }
                }
            }
            ninepatch_rendering.set(true);
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
                                if let Some(window) = web_sys::window() {
                                    if let Some(document) = window.document() {
                                        if let Some(anchor) = document.create_element("a").ok().and_then(|e| e.dyn_into::<HtmlAnchorElement>().ok()) {
                                            anchor.set_href(&url);
                                            anchor.set_download("ninepatch_bricks.zip");
                                            let _ = anchor.click();
                                        }
                                    }
                                }
                            }
                            Err(e) => web_sys::console::error_1(&format!("Ninepatch URL error: {e:?}").into()),
                        },
                        Err(e) => web_sys::console::error_1(&format!("Ninepatch blob error: {e:?}").into()),
                    }
                }
                Err(e) => web_sys::console::error_1(&format!("Ninepatch render error: {e}").into()),
            }
            ninepatch_rendering.set(false);
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

    let tutorial_bricks = (*tutorial).get_brick_state_list();
    let selected_count = (*export_selection).iter().filter(|selected| **selected).count();
    html! {
        <div class="page">
            <div class="transfer-toolbar">
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
                    icon={Html::from_html_unchecked(AttrValue::from(ICON_EXPORT_ALL_BRICKS))}
                    title="Render All Bricks ZIP"
                    label="All ZIP"
                    onclick={export_all_bricks_zip.clone()}
                />
                <IconButton
                    icon={Html::from_html_unchecked(AttrValue::from(ICON_EXPORT_NINEPATCH))}
                    title="Render 9-patch ZIP"
                    label="9-patch ZIP"
                    onclick={export_ninepatch_zip.clone()}
                />
                <IconButton
                    icon={Html::from_html_unchecked(AttrValue::from(if *light { MOON_ICON } else { SUN_ICON }))}
                    title={if *light { "Switch to dark mode" } else { "Switch to light mode" }}
                    label="Theme"
                    onclick={toggle_theme.clone()}
                />
            </div>
            <div class="page__content">
                <div class="page__main">
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
                        on_exit_export={on_exit_export_mode.clone()}
                    />
                </Sidebar>
            </div>
        </div>
    }
}

#[cfg(target_arch = "wasm32")]
pub fn mount_app() {
    yew::Renderer::<App>::new().render();
}
