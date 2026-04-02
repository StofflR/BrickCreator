#[cfg(target_arch = "wasm32")]
use crate::app::editors::brick::BrickEditor;
#[cfg(target_arch = "wasm32")]
use crate::app::editors::tutorial::TutorialEditor;
#[cfg(target_arch = "wasm32")]
use crate::components::sidebar::Sidebar;
#[cfg(target_arch = "wasm32")]
use crate::components::icon_button::IconButton;
use crate::components::modal::Modal;
#[cfg(target_arch = "wasm32")]
use crate::interfaces::brick::{BrickState, StateAction};
#[cfg(target_arch = "wasm32")]
use crate::interfaces::catalog;
#[cfg(target_arch = "wasm32")]
use crate::interfaces::ninepatch;
#[cfg(target_arch = "wasm32")]
use crate::interfaces::tutorial::{TutorialAction, TutorialViewState};
#[cfg(target_arch = "wasm32")]
use crate::interfaces::utility;
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
const SUN_ICON: &str = include_str!("../res/sun.svg");
#[cfg(target_arch = "wasm32")]
const MOON_ICON: &str = include_str!("../res/moon.svg");
#[cfg(target_arch = "wasm32")]
const MENU_ICON: &str = include_str!("../res/menu_dots.svg");

#[cfg(target_arch = "wasm32")]
#[derive(Clone, Copy, PartialEq)]
enum ExportTarget {
    BrickJson,
    TutorialJson,
    TutorialPng,
    BrickPng,
    BricksZip,
}

#[cfg(target_arch = "wasm32")]
#[derive(Clone, Copy, PartialEq)]
enum BricksZipTarget {
    All,
    Ninepatch,
}

#[cfg(target_arch = "wasm32")]
#[derive(Clone, Copy, PartialEq)]
enum ImportTarget {
    BrickJson,
    TutorialJson,
}

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

    let all_bricks_url = use_state(|| Option::<String>::None);
    let all_bricks_rendering = use_state(|| false);
    let ninepatch_url = use_state(|| Option::<String>::None);
    let ninepatch_rendering = use_state(|| false);

    let export_modal_open = use_state(|| false);
    let import_modal_open = use_state(|| false);
    let help_modal_open = use_state(|| false);
    let export_target = use_state(|| ExportTarget::BrickJson);
    let export_bricks_target = use_state(|| BricksZipTarget::All);
    let import_target = use_state(|| ImportTarget::BrickJson);
    let menu_open = use_state(|| false);

    let light = use_state(|| {
        let saved = load_saved_theme();
        apply_theme(saved);
        saved
    });

    let brick_dispatcher = brick.dispatcher();
    let tutorial_dispatcher = tutorial.dispatcher();

    let import_brick_json = {
        let dispatcher = brick_dispatcher.clone();
        Callback::from(move |_: ()| {
            let dispatcher = dispatcher.clone();
            utility::upload_json(Callback::from(move |text: String| {
                dispatcher.dispatch(StateAction::LoadJson(text));
            }));
        })
    };

    let export_brick_json = {
        let brick_state = (*brick).clone();
        Callback::from(move |_: ()| {
            let json = brick_state.to_string();
            if let Err(e) = utility::download_json(&json, "brick.json") {
                web_sys::console::error_1(&format!("Export error: {e}").into());
            }
        })
    };

    let import_tutorial_json = {
        let dispatcher = tutorial_dispatcher.clone();
        Callback::from(move |_: ()| {
            let dispatcher = dispatcher.clone();
            utility::upload_json(Callback::from(move |text: String| {
                dispatcher.dispatch(TutorialAction::LoadJson(text));
            }));
        })
    };

    let export_tutorial_json = {
        let tutorial_state = (*tutorial).clone();
        Callback::from(move |_: ()| {
            let json = tutorial_state.to_json();
            if let Err(e) = utility::download_json(&json, "tutorial.json") {
                web_sys::console::error_1(&format!("Export error: {e}").into());
            }
        })
    };

    let save_tutorial_png = {
        let tutorial_state = (*tutorial).clone();
        Callback::from(move |_: ()| match tutorial_state.get_png_bytes(1920) {
            Ok(data) => {
                if let Err(e) = utility::download_png(&data, "tutorial.png") {
                    web_sys::console::error_1(&format!("PNG error: {e}").into());
                }
            }
            Err(e) => web_sys::console::error_1(&format!("PNG render error: {e}").into()),
        })
    };

    let save_brick_png = {
        let brick_state = (*brick).clone();
        Callback::from(move |_: ()| match brick_state.clone().get_png(1920) {
            Ok(data) => {
                if let Err(e) = utility::download_png(&data, "brick.png") {
                    web_sys::console::error_1(&format!("PNG error: {e}").into());
                }
            }
            Err(e) => web_sys::console::error_1(&format!("PNG render error: {e}").into()),
        })
    };

    let export_all_bricks_png = {
        let all_bricks_url = all_bricks_url.clone();
        let all_bricks_rendering = all_bricks_rendering.clone();
        Callback::from(move |_: ()| {
            if *all_bricks_rendering {
                return;
            }

            if let Some(url) = (*all_bricks_url).clone() {
                let Some(window) = web_sys::window() else {
                    web_sys::console::error_1(&"All bricks download error: no window".into());
                    return;
                };
                let Some(document) = window.document() else {
                    web_sys::console::error_1(&"All bricks download error: no document".into());
                    return;
                };

                let anchor: HtmlAnchorElement = match document
                    .create_element("a")
                    .ok()
                    .and_then(|e| e.dyn_into().ok())
                {
                    Some(a) => a,
                    None => {
                        web_sys::console::error_1(
                            &"All bricks download error: failed to create anchor".into(),
                        );
                        return;
                    }
                };

                anchor.set_href(&url);
                anchor.set_download("all_bricks.zip");
                anchor.set_attribute("style", "display:none").ok();

                if let Some(body) = document.body() {
                    let _ = body.append_child(&anchor);
                    anchor.click();
                    let _ = body.remove_child(&anchor);
                } else {
                    anchor.click();
                }

                return;
            }

            all_bricks_rendering.set(true);
            match catalog::render_all_bricks_zip_bytes(1920) {
                Ok(data) => {
                    let uint8 = js_sys::Uint8Array::from(data.as_slice());
                    let parts = js_sys::Array::new();
                    parts.push(&uint8);

                    let mut opts = BlobPropertyBag::new();
                    #[allow(deprecated)]
                    opts.type_("application/zip");
                    let blob =
                        match Blob::new_with_buffer_source_sequence_and_options(&parts, &opts) {
                            Ok(b) => b,
                            Err(e) => {
                                web_sys::console::error_1(
                                    &format!("All bricks blob error: {e:?}").into(),
                                );
                                all_bricks_rendering.set(false);
                                return;
                            }
                        };

                    match Url::create_object_url_with_blob(&blob) {
                        Ok(url) => {
                            let url_for_state = url.clone();
                            all_bricks_url.set(Some(url_for_state));
                            if let Some(window) = web_sys::window()
                                && let Some(document) = window.document()
                            {
                                let anchor: Option<HtmlAnchorElement> = document
                                    .create_element("a")
                                    .ok()
                                    .and_then(|e| e.dyn_into().ok());
                                if let Some(anchor) = anchor {
                                    anchor.set_href(&url);
                                    anchor.set_download("all_bricks.zip");
                                    anchor.set_attribute("style", "display:none").ok();
                                    if let Some(body) = document.body() {
                                        let _ = body.append_child(&anchor);
                                        anchor.click();
                                        let _ = body.remove_child(&anchor);
                                    } else {
                                        anchor.click();
                                    }
                                    web_sys::console::log_1(
                                        &"ALL bricks ZIP download attempted. If nothing happened, click again.".into(),
                                    );
                                } else {
                                    web_sys::console::log_1(
                                        &"ALL bricks ZIP ready — click again to download.".into(),
                                    );
                                }
                            } else {
                                web_sys::console::log_1(
                                    &"ALL bricks ZIP ready — click again to download.".into(),
                                );
                            }
                        }
                        Err(e) => web_sys::console::error_1(
                            &format!("All bricks URL error: {e:?}").into(),
                        ),
                    }
                }
                Err(e) => {
                    web_sys::console::error_1(&format!("All bricks render error: {e}").into())
                }
            }
            all_bricks_rendering.set(false);
        })
    };

    let export_ninepatch_zip = {
        let ninepatch_url = ninepatch_url.clone();
        let ninepatch_rendering = ninepatch_rendering.clone();
        Callback::from(move |_: ()| {
            if *ninepatch_rendering {
                return;
            }

            if let Some(url) = (*ninepatch_url).clone() {
                let Some(window) = web_sys::window() else {
                    web_sys::console::error_1(&"9-patch download error: no window".into());
                    return;
                };
                let Some(document) = window.document() else {
                    web_sys::console::error_1(&"9-patch download error: no document".into());
                    return;
                };

                let anchor: HtmlAnchorElement = match document
                    .create_element("a")
                    .ok()
                    .and_then(|e| e.dyn_into().ok())
                {
                    Some(a) => a,
                    None => {
                        web_sys::console::error_1(
                            &"9-patch download error: failed to create anchor".into(),
                        );
                        return;
                    }
                };

                anchor.set_href(&url);
                anchor.set_download("ninepatch_bricks.zip");
                anchor.set_attribute("style", "display:none").ok();

                if let Some(body) = document.body() {
                    let _ = body.append_child(&anchor);
                    anchor.click();
                    let _ = body.remove_child(&anchor);
                } else {
                    anchor.click();
                }

                return;
            }

            ninepatch_rendering.set(true);
            match ninepatch::render_ninepatch_zip_bytes() {
                Ok(data) => {
                    let uint8 = js_sys::Uint8Array::from(data.as_slice());
                    let parts = js_sys::Array::new();
                    parts.push(&uint8);

                    let mut opts = BlobPropertyBag::new();
                    #[allow(deprecated)]
                    opts.type_("application/zip");
                    let blob =
                        match Blob::new_with_buffer_source_sequence_and_options(&parts, &opts) {
                            Ok(b) => b,
                            Err(e) => {
                                web_sys::console::error_1(
                                    &format!("9-patch blob error: {e:?}").into(),
                                );
                                ninepatch_rendering.set(false);
                                return;
                            }
                        };

                    match Url::create_object_url_with_blob(&blob) {
                        Ok(url) => {
                            let url_for_state = url.clone();
                            ninepatch_url.set(Some(url_for_state));
                            if let Some(window) = web_sys::window()
                                && let Some(document) = window.document()
                            {
                                let anchor: Option<HtmlAnchorElement> = document
                                    .create_element("a")
                                    .ok()
                                    .and_then(|e| e.dyn_into().ok());
                                if let Some(anchor) = anchor {
                                    anchor.set_href(&url);
                                    anchor.set_download("ninepatch_bricks.zip");
                                    anchor.set_attribute("style", "display:none").ok();
                                    if let Some(body) = document.body() {
                                        let _ = body.append_child(&anchor);
                                        anchor.click();
                                        let _ = body.remove_child(&anchor);
                                    } else {
                                        anchor.click();
                                    }
                                    web_sys::console::log_1(
                                        &"9-patch ZIP download attempted. If nothing happened, click again.".into(),
                                    );
                                } else {
                                    web_sys::console::log_1(
                                        &"9-patch ZIP ready — click again to download.".into(),
                                    );
                                }
                            } else {
                                web_sys::console::log_1(
                                    &"9-patch ZIP ready — click again to download.".into(),
                                );
                            }
                        }
                        Err(e) => {
                            web_sys::console::error_1(&format!("9-patch URL error: {e:?}").into())
                        }
                    }
                }
                Err(e) => web_sys::console::error_1(&format!("9-patch render error: {e}").into()),
            }
            ninepatch_rendering.set(false);
        })
    };

    let all_bricks_ready = (*all_bricks_url).is_some();
    let ninepatch_ready = (*ninepatch_url).is_some();

    let all_bricks_title = if *all_bricks_rendering {
        "Rendering ALL bricks…"
    } else if all_bricks_ready {
        "Download ALL bricks ZIP (click again if needed)"
    } else {
        "Render ALL bricks ZIP"
    };

    let ninepatch_title = if *ninepatch_rendering {
        "Rendering 9-patch ZIP…"
    } else if ninepatch_ready {
        "Download 9-patch ZIP (click again if needed)"
    } else {
        "Render 9-patch ZIP"
    };

    let export_action_label = match *export_target {
        ExportTarget::BrickJson => "Export Brick JSON",
        ExportTarget::TutorialJson => "Export Tutorial JSON",
        ExportTarget::TutorialPng => "Export Tutorial PNG",
        ExportTarget::BrickPng => "Export Brick PNG",
        ExportTarget::BricksZip => "Render Bricks ZIP",
    };

    let export_action_hint = match *export_target {
        ExportTarget::BrickJson => "Exports the current brick as JSON.",
        ExportTarget::TutorialJson => "Exports the current tutorial as JSON.",
        ExportTarget::TutorialPng => "Exports the tutorial PNG at 1920px.",
        ExportTarget::BrickPng => "Exports the current brick PNG at 1920px.",
        ExportTarget::BricksZip => match *export_bricks_target {
            BricksZipTarget::All => all_bricks_title,
            BricksZipTarget::Ninepatch => ninepatch_title,
        },
    };

    let export_busy = matches!(*export_target, ExportTarget::BricksZip)
        && match *export_bricks_target {
            BricksZipTarget::All => *all_bricks_rendering,
            BricksZipTarget::Ninepatch => *ninepatch_rendering,
        };

    let import_action_label = match *import_target {
        ImportTarget::BrickJson => "Import Brick JSON",
        ImportTarget::TutorialJson => "Import Tutorial JSON",
    };

    let on_close_export_modal = {
        let export_modal_open = export_modal_open.clone();
        Callback::from(move |_: MouseEvent| export_modal_open.set(false))
    };

    let on_close_import_modal = {
        let import_modal_open = import_modal_open.clone();
        Callback::from(move |_: MouseEvent| import_modal_open.set(false))
    };

    let on_page_click = {
        let menu_open = menu_open.clone();
        Callback::from(move |_: MouseEvent| {
            if *menu_open {
                menu_open.set(false);
            }
        })
    };

    let on_menu_container_click = Callback::from(|e: MouseEvent| {
        e.stop_propagation();
    });

    let on_toggle_menu = {
        let menu_open = menu_open.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            e.stop_propagation();
            let is_open = *menu_open;
            menu_open.set(!is_open);
        })
    };

    let on_menu_open_export = {
        let export_modal_open = export_modal_open.clone();
        let menu_open = menu_open.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            e.stop_propagation();
            export_modal_open.set(true);
            menu_open.set(false);
        })
    };

    let on_menu_open_import = {
        let import_modal_open = import_modal_open.clone();
        let menu_open = menu_open.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            e.stop_propagation();
            import_modal_open.set(true);
            menu_open.set(false);
        })
    };

    let on_menu_toggle_theme = {
        let light = light.clone();
        let menu_open = menu_open.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            e.stop_propagation();
            let new_val = !*light;
            apply_theme(new_val);
            light.set(new_val);
            menu_open.set(false);
        })
    };

    let on_open_help_modal = {
        let help_modal_open = help_modal_open.clone();
        let menu_open = menu_open.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            e.stop_propagation();
            help_modal_open.set(true);
            menu_open.set(false);
        })
    };

    let on_close_help_modal = {
        let help_modal_open = help_modal_open.clone();
        Callback::from(move |_: MouseEvent| help_modal_open.set(false))
    };

    let on_export_target_brick_json = {
        let export_target = export_target.clone();
        Callback::from(move |_: MouseEvent| export_target.set(ExportTarget::BrickJson))
    };

    let on_export_target_tutorial_json = {
        let export_target = export_target.clone();
        Callback::from(move |_: MouseEvent| export_target.set(ExportTarget::TutorialJson))
    };

    let on_export_target_tutorial_png = {
        let export_target = export_target.clone();
        Callback::from(move |_: MouseEvent| export_target.set(ExportTarget::TutorialPng))
    };

    let on_export_target_brick_png = {
        let export_target = export_target.clone();
        Callback::from(move |_: MouseEvent| export_target.set(ExportTarget::BrickPng))
    };

    let on_export_target_bricks_zip = {
        let export_target = export_target.clone();
        Callback::from(move |_: MouseEvent| export_target.set(ExportTarget::BricksZip))
    };

    let on_export_bricks_target_all = {
        let export_bricks_target = export_bricks_target.clone();
        Callback::from(move |_: MouseEvent| export_bricks_target.set(BricksZipTarget::All))
    };

    let on_export_bricks_target_ninepatch = {
        let export_bricks_target = export_bricks_target.clone();
        Callback::from(move |_: MouseEvent| export_bricks_target.set(BricksZipTarget::Ninepatch))
    };

    let on_import_target_brick_json = {
        let import_target = import_target.clone();
        Callback::from(move |_: MouseEvent| import_target.set(ImportTarget::BrickJson))
    };

    let on_import_target_tutorial_json = {
        let import_target = import_target.clone();
        Callback::from(move |_: MouseEvent| import_target.set(ImportTarget::TutorialJson))
    };

    let on_confirm_export = {
        let export_target = export_target.clone();
        let export_bricks_target = export_bricks_target.clone();
        let export_modal_open = export_modal_open.clone();
        let export_brick_json = export_brick_json.clone();
        let export_tutorial_json = export_tutorial_json.clone();
        let save_tutorial_png = save_tutorial_png.clone();
        let save_brick_png = save_brick_png.clone();
        let export_all_bricks_png = export_all_bricks_png.clone();
        let export_ninepatch_zip = export_ninepatch_zip.clone();
        Callback::from(move |_: MouseEvent| {
            match *export_target {
                ExportTarget::BrickJson => export_brick_json.emit(()),
                ExportTarget::TutorialJson => export_tutorial_json.emit(()),
                ExportTarget::TutorialPng => save_tutorial_png.emit(()),
                ExportTarget::BrickPng => save_brick_png.emit(()),
                ExportTarget::BricksZip => match *export_bricks_target {
                    BricksZipTarget::All => export_all_bricks_png.emit(()),
                    BricksZipTarget::Ninepatch => export_ninepatch_zip.emit(()),
                },
            }
            export_modal_open.set(false);
        })
    };

    let on_confirm_import = {
        let import_target = import_target.clone();
        let import_modal_open = import_modal_open.clone();
        let import_brick_json = import_brick_json.clone();
        let import_tutorial_json = import_tutorial_json.clone();
        Callback::from(move |_: MouseEvent| {
            match *import_target {
                ImportTarget::BrickJson => import_brick_json.emit(()),
                ImportTarget::TutorialJson => import_tutorial_json.emit(()),
            }
            import_modal_open.set(false);
        })
    };

    html! {
        <div class="page" onclick={on_page_click}>
            <div class="transfer-toolbar">
                <div class="transfer-toolbar__menu" onclick={on_menu_container_click.clone()}>
                    <IconButton
                        icon={Html::from_html_unchecked(AttrValue::from(MENU_ICON))}
                        title="Menu"
                        label="Menu"
                        onclick={on_toggle_menu.clone()}
                    />
                    if *menu_open {
                        <div class="menu-dropdown">
                            <button class="menu-dropdown__item" type="button" onclick={on_menu_open_import.clone()}>
                                {"Import"}
                            </button>
                            <button class="menu-dropdown__item" type="button" onclick={on_menu_open_export.clone()}>
                                {"Export"}
                            </button>
                            <button class="menu-dropdown__item" type="button" onclick={on_menu_toggle_theme.clone()}>
                                {if *light { "Dark mode" } else { "Light mode" }}
                            </button>
                            <button class="menu-dropdown__item" type="button" onclick={on_open_help_modal.clone()}>
                                {"Help"}
                            </button>
                        </div>
                    }
                </div>
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
                    />
                </Sidebar>
            </div>
            <div class="menu-toolbar">
                <div class="menu-toolbar__menu" onclick={on_menu_container_click.clone()}>
                    <IconButton
                        icon={Html::from_html_unchecked(AttrValue::from(MENU_ICON))}
                        title="Menu"
                        label="Menu"
                        onclick={on_toggle_menu.clone()}
                    />
                    if *menu_open {
                        <div class="menu-dropdown">
                            <button class="menu-dropdown__item" type="button" onclick={on_menu_open_import.clone()}>
                                {"Import"}
                            </button>
                            <button class="menu-dropdown__item" type="button" onclick={on_menu_open_export.clone()}>
                                {"Export"}
                            </button>
                            <button class="menu-dropdown__item" type="button" onclick={on_menu_toggle_theme.clone()}>
                                {if *light { "Dark mode" } else { "Light mode" }}
                            </button>
                            <button class="menu-dropdown__item" type="button" onclick={on_open_help_modal.clone()}>
                                {"Help"}
                            </button>
                        </div>
                    }
                </div>
            </div>
            if *import_modal_open {
                <div data-testid="import-modal">
                <Modal
                    title="Import"
                    hint="Choose what to import"
                    on_close={on_close_import_modal.clone()}
                    class={classes!("modal--compact")}
                >
                    <div class="transfer-modal">
                        <div class="transfer-modal__group">
                            <div class="transfer-modal__label">{"Target"}</div>
                            <label class="transfer-modal__option" onclick={on_import_target_brick_json.clone()}>
                                <input
                                    type="radio"
                                    id="import-brick-json" data-testid="import-brick-json"
                                    name="import-target"
                                    checked={*import_target == ImportTarget::BrickJson}
                                />
                                <span>{"Brick JSON"}</span>
                            </label>
                            <label class="transfer-modal__option" onclick={on_import_target_tutorial_json.clone()}>
                                <input
                                    type="radio"
                                    id="import-tutorial-json" data-testid="import-tutorial-json"
                                    name="import-target"
                                    checked={*import_target == ImportTarget::TutorialJson}
                                />
                                <span>{"Tutorial JSON"}</span>
                            </label>
                        </div>
                        <button
                            class="transfer-modal__action"
                            type="button"
                            onclick={on_confirm_import.clone()}
                        >
                            {import_action_label}
                        </button>
                    </div>
                </Modal>
                </div>
            }
            if *export_modal_open {
                <div data-testid="export-modal">
                <Modal
                    title="Export"
                    hint="Choose a format"
                    on_close={on_close_export_modal.clone()}
                    class={classes!("modal--compact")}
                >
                    <div class="transfer-modal">
                        <div class="transfer-modal__group">
                            <div class="transfer-modal__label">{"Format"}</div>
                            <label class="transfer-modal__option" onclick={on_export_target_brick_json.clone()}>
                                <input
                                    type="radio"
                                    id="export-brick-json" data-testid="export-brick-json"
                                    name="export-target"
                                    checked={*export_target == ExportTarget::BrickJson}
                                />
                                <span>{"Brick JSON"}</span>
                            </label>
                            <label class="transfer-modal__option" onclick={on_export_target_tutorial_json.clone()}>
                                <input
                                    type="radio"
                                    id="export-tutorial-json" data-testid="export-tutorial-json"
                                    name="export-target"
                                    checked={*export_target == ExportTarget::TutorialJson}
                                />
                                <span>{"Tutorial JSON"}</span>
                            </label>
                            <label class="transfer-modal__option" onclick={on_export_target_tutorial_png.clone()}>
                                <input
                                    type="radio"
                                    id="export-tutorial-png" data-testid="export-tutorial-png"
                                    name="export-target"
                                    checked={*export_target == ExportTarget::TutorialPng}
                                />
                                <span>{"Tutorial PNG"}</span>
                            </label>
                            <label class="transfer-modal__option" onclick={on_export_target_brick_png.clone()}>
                                <input
                                    type="radio"
                                    id="export-brick-png" data-testid="export-brick-png"
                                    name="export-target"
                                    checked={*export_target == ExportTarget::BrickPng}
                                />
                                <span>{"Brick PNG"}</span>
                            </label>
                            <label class="transfer-modal__option" onclick={on_export_target_bricks_zip.clone()}>
                                <input
                                    type="radio"
                                    id="export-bricks-zip" data-testid="export-bricks-zip"
                                    name="export-target"
                                    checked={*export_target == ExportTarget::BricksZip}
                                />
                                <span>{"Bricks ZIP"}</span>
                            </label>
                        </div>
                        <div class="transfer-modal__group">
                            <div class="transfer-modal__label">{"Brick set"}</div>
                            <label class="transfer-modal__option" onclick={on_export_bricks_target_all.clone()}>
                                <input
                                    type="radio"
                                    id="export-bricks-all" data-testid="export-bricks-all"
                                    name="brick-set"
                                    checked={*export_bricks_target == BricksZipTarget::All}
                                />
                                <span>{"All bricks"}</span>
                            </label>
                            <label class="transfer-modal__option" onclick={on_export_bricks_target_ninepatch.clone()}>
                                <input
                                    type="radio"
                                    id="export-bricks-ninepatch" data-testid="export-bricks-ninepatch"
                                    name="brick-set"
                                    checked={*export_bricks_target == BricksZipTarget::Ninepatch}
                                />
                                <span>{"9-patch bricks"}</span>
                            </label>
                        </div>
                        if !export_action_hint.is_empty() {
                            <div class="transfer-modal__hint">{export_action_hint}</div>
                        }
                        <button
                            class="transfer-modal__action"
                            type="button"
                            onclick={on_confirm_export.clone()}
                            disabled={export_busy}
                        >
                            {export_action_label}
                        </button>
                    </div>
                </Modal>
                </div>
            }
            if *help_modal_open {
                <div data-testid="help-modal">
                <Modal
                    title="Help"
                    hint="Quick tips"
                    on_close={on_close_help_modal.clone()}
                    class={classes!("modal--compact")}
                >
                    <div class="help-modal">
                        <div class="help-modal__item">
                            <div class="help-modal__title">{"Import / Export"}</div>
                            <div class="help-modal__text">
                                {"Use JSON for edits and PNG for previews. Bricks ZIP renders a full set."}
                            </div>
                        </div>
                        <div class="help-modal__item">
                            <div class="help-modal__title">{"Theme"}</div>
                            <div class="help-modal__text">
                                {"Switch between light and dark mode from the menu or toolbar."}
                            </div>
                        </div>
                    </div>
                </Modal>
                </div>
            }
        </div>
    }
}

#[cfg(target_arch = "wasm32")]
pub fn mount_app() {
    yew::Renderer::<App>::new().render();
}
