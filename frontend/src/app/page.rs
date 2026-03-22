#[cfg(target_arch = "wasm32")]
use crate::app::editors::brick::BrickEditor;
#[cfg(target_arch = "wasm32")]
use crate::app::editors::tutorial::TutorialEditor;
#[cfg(target_arch = "wasm32")]
use crate::components::sidebar::Sidebar;
#[cfg(target_arch = "wasm32")]
use crate::components::icon_button::IconButton;
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
use std::rc::Rc;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use web_sys::{Blob, BlobPropertyBag, HtmlAnchorElement, HtmlSelectElement, Url};
#[cfg(target_arch = "wasm32")]
use yew::prelude::*;

#[cfg(target_arch = "wasm32")]
const ICON_UPLOAD: &str = include_str!("../res/upload.svg");
#[cfg(target_arch = "wasm32")]
const ICON_FILE_JSON: &str = include_str!("../res/file_json.svg");
#[cfg(target_arch = "wasm32")]
const ICON_FILE_PNG: &str = include_str!("../res/file_png.svg");
#[cfg(target_arch = "wasm32")]
const ICON_DOWNLOAD: &str = include_str!("../res/download.svg");
#[cfg(target_arch = "wasm32")]
const ICON_NINEPATCH: &str = include_str!("../res/ninepatch_9.svg");

#[cfg(target_arch = "wasm32")]
#[function_component(App)]
fn app() -> Html {
    let brick = use_reducer(BrickState::default);
    let tutorial = use_reducer(TutorialViewState::default);

    let all_bricks_url = use_state(|| Option::<String>::None);
    let all_bricks_rendering = use_state(|| false);
    let ninepatch_url = use_state(|| Option::<String>::None);
    let ninepatch_rendering = use_state(|| false);

    let selected_action = use_state(|| "brick".to_string());

    let brick_dispatcher = brick.dispatcher();
    let tutorial_dispatcher = tutorial.dispatcher();

    let on_action_change = {
        let selected_action = selected_action.clone();
        Callback::from(move |e: Event| {
            if let Some(select) = e.target_dyn_into::<HtmlSelectElement>() {
                selected_action.set(select.value());
            }
        })
    };

    let import_brick = {
        let dispatcher = brick_dispatcher.clone();
        Rc::new(move || {
            let dispatcher = dispatcher.clone();
            utility::upload_json(Callback::from(move |text: String| {
                dispatcher.dispatch(StateAction::LoadJson(text));
            }));
        })
    };

    let export_brick = {
        let brick_state = (*brick).clone();
        Rc::new(move || {
            let json = brick_state.to_string();
            if let Err(e) = utility::download_json(&json, "brick.json") {
                web_sys::console::error_1(&format!("Export error: {e}").into());
            }
        })
    };

    let import_tutorial = {
        let dispatcher = tutorial_dispatcher.clone();
        Rc::new(move || {
            let dispatcher = dispatcher.clone();
            utility::upload_json(Callback::from(move |text: String| {
                dispatcher.dispatch(TutorialAction::LoadJson(text));
            }));
        })
    };

    let export_tutorial = {
        let tutorial_state = (*tutorial).clone();
        Rc::new(move || {
            let json = tutorial_state.to_json();
            if let Err(e) = utility::download_json(&json, "tutorial.json") {
                web_sys::console::error_1(&format!("Export error: {e}").into());
            }
        })
    };

    let save_tutorial_png = {
        let tutorial_state = (*tutorial).clone();
        Rc::new(move || match tutorial_state.get_png_bytes(1920) {
            Ok(data) => {
                if let Err(e) = utility::download_png(&data, "tutorial.png") {
                    web_sys::console::error_1(&format!("PNG error: {e}").into());
                }
            }
            Err(e) => web_sys::console::error_1(&format!("PNG render error: {e}").into()),
        })
    };

    let export_all_bricks = {
        let all_bricks_url = all_bricks_url.clone();
        let all_bricks_rendering = all_bricks_rendering.clone();
        Rc::new(move || {
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
            match catalog::render_all_bricks_zip_bytes(300) {
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

    let export_ninepatch = {
        let ninepatch_url = ninepatch_url.clone();
        let ninepatch_rendering = ninepatch_rendering.clone();
        Rc::new(move || {
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

    let on_import_selected = {
        let selected_action = selected_action.clone();
        let import_brick = import_brick.clone();
        let import_tutorial = import_tutorial.clone();
        Callback::from(move |_: MouseEvent| {
            match selected_action.as_str() {
                "brick" => (import_brick)(),
                "tutorial" => (import_tutorial)(),
                _ => {}
            }
        })
    };

    let on_export_selected = {
        let selected_action = selected_action.clone();
        let export_brick = export_brick.clone();
        let export_tutorial = export_tutorial.clone();
        let save_tutorial_png = save_tutorial_png.clone();
        let export_all_bricks = export_all_bricks.clone();
        let export_ninepatch = export_ninepatch.clone();
        Callback::from(move |_: MouseEvent| {
            match selected_action.as_str() {
                "brick" => (export_brick)(),
                "tutorial" => (export_tutorial)(),
                "tutorial_png" => (save_tutorial_png)(),
                "all_bricks" => (export_all_bricks)(),
                "ninepatch" => (export_ninepatch)(),
                _ => {}
            }
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

    html! {
        <div class="page">
            <div class="page__toolbar">
                <div class="page__toolbar-group">
                    <span class="page__toolbar-label">{"Action"}</span>
                    <select class="page__toolbar-select" onchange={on_action_change}>
                        <option value="brick">{"Brick JSON"}</option>
                        <option value="tutorial">{"Tutorial JSON"}</option>
                        <option value="tutorial_png">{"Tutorial PNG"}</option>
                        <option value="all_bricks">{"All Bricks ZIP"}</option>
                        <option value="ninepatch">{"9-patch ZIP"}</option>
                    </select>
                </div>
                <IconButton
                    icon={Html::from_html_unchecked(AttrValue::from(ICON_UPLOAD))}
                    title={if selected_action.as_str() == "tutorial" { "Import Tutorial JSON" } else { "Import Brick JSON" }}
                    onclick={on_import_selected.clone()}
                    disabled={!(selected_action.as_str() == "brick" || selected_action.as_str() == "tutorial")}
                />
                <IconButton
                    icon={match selected_action.as_str() {
                        "tutorial_png" => Html::from_html_unchecked(AttrValue::from(ICON_FILE_PNG)),
                        "all_bricks" => Html::from_html_unchecked(AttrValue::from(ICON_DOWNLOAD)),
                        "ninepatch" => Html::from_html_unchecked(AttrValue::from(ICON_NINEPATCH)),
                        _ => Html::from_html_unchecked(AttrValue::from(ICON_FILE_JSON)),
                    }}
                    title={match selected_action.as_str() {
                        "brick" => "Export Brick JSON",
                        "tutorial" => "Export Tutorial JSON",
                        "tutorial_png" => "Save Tutorial PNG",
                        "all_bricks" => all_bricks_title,
                        "ninepatch" => ninepatch_title,
                        _ => "Export",
                    }}
                    onclick={on_export_selected.clone()}
                    disabled={match selected_action.as_str() {
                        "all_bricks" => *all_bricks_rendering,
                        "ninepatch" => *ninepatch_rendering,
                        _ => false,
                    }}
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
