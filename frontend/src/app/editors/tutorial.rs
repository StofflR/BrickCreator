#[cfg(target_arch = "wasm32")]
use crate::app::views::brick_catalog_modal::BrickCatalogModal;
#[cfg(target_arch = "wasm32")]
use crate::app::views::tutorial::TutorialEditView;
#[cfg(target_arch = "wasm32")]
use crate::app::views::tutorial_preview::TutorialPreviewView;
#[cfg(target_arch = "wasm32")]
use crate::app::views::tutorial_settings::TutorialSettingsView;
#[cfg(target_arch = "wasm32")]
use crate::components::editor_group::EditorGroup;
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
#[derive(Properties, PartialEq)]
pub struct TutorialEditorProps {
    pub brick: BrickState,
    pub brick_dispatcher: UseReducerDispatcher<BrickState>,
    pub tutorial: TutorialViewState,
    pub tutorial_dispatcher: UseReducerDispatcher<TutorialViewState>,
}

#[cfg(target_arch = "wasm32")]
#[function_component(TutorialEditor)]
pub fn tutorial_editor(props: &TutorialEditorProps) -> Html {
    let bricks = props.tutorial.get_brick_state_list();
    let preview_toggle = use_state(|| false);
    let preview = *preview_toggle.clone();
    let catalog_open = use_state(|| false);
    let all_bricks_url = use_state(|| Option::<String>::None);
    let all_bricks_rendering = use_state(|| false);
    let ninepatch_url = use_state(|| Option::<String>::None);
    let ninepatch_rendering = use_state(|| false);
    let preview_data = if preview {
        props.tutorial.get_png(800).ok()
    } else {
        None
    };

    let on_remove = {
        let dispatcher = props.tutorial_dispatcher.clone();
        Callback::from(move |_: MouseEvent| {
            dispatcher.dispatch(TutorialAction::RemoveSelected);
        })
    };

    let on_apply = {
        let dispatcher = props.tutorial_dispatcher.clone();
        let brick = props.brick.clone();
        Callback::from(move |_: MouseEvent| {
            dispatcher.dispatch(TutorialAction::ApplyChanges(brick.clone()));
        })
    };

    let on_toggle_preview = {
        Callback::from(move |_: MouseEvent| {
            preview_toggle.clone().set(!*preview_toggle);
        })
    };

    let on_open_catalog = {
        let catalog_open = catalog_open.clone();
        Callback::from(move |_: MouseEvent| {
            catalog_open.set(true);
        })
    };

    let on_select = {
        let dispatcher = props.tutorial_dispatcher.clone();
        let brick_dispatcher = props.brick_dispatcher.clone();
        let bricks = bricks.clone();
        Callback::from(move |index: usize| {
            dispatcher.dispatch(TutorialAction::Select(index));
            if let Some(brick) = bricks.get(index) {
                brick_dispatcher.dispatch(StateAction::Set(brick.clone()));
            }
        })
    };

    let on_move = {
        let dispatcher = props.tutorial_dispatcher.clone();
        Callback::from(move |(from, to): (usize, usize)| {
            dispatcher.dispatch(TutorialAction::MoveEntry(from, to));
        })
    };

    let on_import_json = {
        let dispatcher = props.tutorial_dispatcher.clone();
        Callback::from(move |_: MouseEvent| {
            let dispatcher = dispatcher.clone();
            utility::upload_json(Callback::from(move |text: String| {
                dispatcher.dispatch(TutorialAction::LoadJson(text));
            }));
        })
    };

    let on_export_json = {
        let tutorial = props.tutorial.clone();
        Callback::from(move |_: MouseEvent| {
            let json = tutorial.to_json();
            if let Err(e) = utility::download_json(&json, "tutorial.json") {
                web_sys::console::error_1(&format!("Export error: {e}").into());
            }
        })
    };

    let on_save_png = {
        let tutorial = props.tutorial.clone();
        Callback::from(move |_: MouseEvent| match tutorial.get_png_bytes(1920) {
            Ok(data) => {
                if let Err(e) = utility::download_png(&data, "tutorial.png") {
                    web_sys::console::error_1(&format!("PNG error: {e}").into());
                }
            }
            Err(e) => web_sys::console::error_1(&format!("PNG render error: {e}").into()),
        })
    };

    let on_export_all_bricks_png = {
        let all_bricks_url = all_bricks_url.clone();
        let all_bricks_rendering = all_bricks_rendering.clone();
        Callback::from(move |_: MouseEvent| {
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

    let on_export_ninepatch_zip = {
        let ninepatch_url = ninepatch_url.clone();
        let ninepatch_rendering = ninepatch_rendering.clone();
        Callback::from(move |_: MouseEvent| {
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

    let has_selection = props.tutorial.selected_index.is_some();
    let selected_index = props.tutorial.selected_index;
    let all_bricks_ready = (*all_bricks_url).is_some();
    let ninepatch_ready = (*ninepatch_url).is_some();

    html! {
        <EditorGroup title="Tutorial Editor">
            <div class="tutorial-view">
                if *catalog_open {
                    <BrickCatalogModal
                        on_close={{
                            let catalog_open = catalog_open.clone();
                            Callback::from(move |_: MouseEvent| catalog_open.set(false))
                        }}
                        on_add_brick={{
                            let dispatcher = props.tutorial_dispatcher.clone();
                            let catalog_open = catalog_open.clone();
                            Callback::from(move |brick: BrickState| {
                                dispatcher.dispatch(TutorialAction::AddBrick(brick));
                                catalog_open.set(false);
                            })
                        }}
                    />
                }
                <TutorialSettingsView
                    {on_remove}
                    {on_apply}
                    {on_toggle_preview}
                    {on_import_json}
                    {on_export_json}
                    {on_save_png}
                    {on_export_all_bricks_png}
                    {on_export_ninepatch_zip}
                    {on_open_catalog}
                    {all_bricks_ready}
                    all_bricks_rendering={*all_bricks_rendering}
                    {ninepatch_ready}
                    ninepatch_rendering={*ninepatch_rendering}
                    {has_selection}
                    show_preview={preview}
                />
                if preview {
                    <TutorialPreviewView preview_data={preview_data} />
                } else {
                    <TutorialEditView
                        bricks={bricks}
                        selected_index={selected_index}
                        on_select={on_select}
                        on_move={on_move}
                    />
                }
            </div>
        </EditorGroup>
    }
}
