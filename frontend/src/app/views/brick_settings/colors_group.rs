#[cfg(target_arch = "wasm32")]
use crate::style;
#[cfg(target_arch = "wasm32")]
use crate::app::views::color::ColorView;
#[cfg(target_arch = "wasm32")]
use crate::components::modal::Modal;
#[cfg(target_arch = "wasm32")]
use crate::components::editor_group::EditorGroup;
#[cfg(target_arch = "wasm32")]
use crate::interfaces::color::{BrickColorModel, ColorModel};
#[cfg(target_arch = "wasm32")]
use shared::color::ColorScheme;
#[cfg(target_arch = "wasm32")]
use web_sys::HtmlInputElement;
#[cfg(target_arch = "wasm32")]
use yew::prelude::*;

#[cfg(target_arch = "wasm32")]
const CUSTOM_COLOR_STORAGE_KEY: &str = "brickcreator.custom_colors";

#[cfg(target_arch = "wasm32")]
fn load_saved_custom_colors() -> Vec<ColorScheme> {
    gloo::utils::window()
        .local_storage()
        .ok()
        .flatten()
        .and_then(|storage| storage.get_item(CUSTOM_COLOR_STORAGE_KEY).ok().flatten())
        .and_then(|value| serde_json::from_str::<Vec<ColorScheme>>(&value).ok())
        .unwrap_or_default()
}

#[cfg(target_arch = "wasm32")]
fn persist_saved_custom_colors(colors: &[ColorScheme]) {
    let Ok(Some(storage)) = gloo::utils::window().local_storage() else {
        return;
    };

    if colors.is_empty() {
        let _ = storage.remove_item(CUSTOM_COLOR_STORAGE_KEY);
        return;
    }

    if let Ok(value) = serde_json::to_string(colors) {
        let _ = storage.set_item(CUSTOM_COLOR_STORAGE_KEY, &value);
    }
}

#[cfg(target_arch = "wasm32")]
#[derive(Properties, PartialEq)]
pub struct ColorsGroupProps {
    pub selected_name: String,
    pub on_select: Callback<ColorScheme>,
}

#[cfg(target_arch = "wasm32")]
#[derive(Clone, PartialEq)]
struct CustomColorDraft {
    name: String,
    color: String,
    shade: String,
    border: String,
    text: String,
    save_for_later: bool,
}

#[cfg(target_arch = "wasm32")]
impl Default for CustomColorDraft {
    fn default() -> Self {
        Self {
            name: String::new(),
            color: "#408ac5".to_string(),
            shade: "#27567c".to_string(),
            border: "#383838".to_string(),
            text: "#ffffff".to_string(),
            save_for_later: false,
        }
    }
}

#[cfg(target_arch = "wasm32")]
impl CustomColorDraft {
    fn to_scheme(&self) -> ColorScheme {
        ColorScheme {
            name: self.name.trim().to_string(),
            color: self.color.clone(),
            shade: self.shade.clone(),
            border: self.border.clone(),
            text: self.text.clone(),
        }
    }
}

#[cfg(target_arch = "wasm32")]
#[function_component(ColorsGroup)]
pub fn colors_group(props: &ColorsGroupProps) -> Html {
    let model = use_state(|| BrickColorModel::with_custom_colors(load_saved_custom_colors()));
    let saved_custom_colors = use_state(load_saved_custom_colors);
    let modal_open = use_state(|| false);
    let draft = use_state(CustomColorDraft::default);
    let error_message = use_state(|| Option::<String>::None);
    let context_menu = use_state(|| Option::<(String, i32, i32)>::None);

    let colors = (*model).all_colors();
    let custom_color_names = colors
        .iter()
        .map(|color| color.name.clone())
        .filter(|name| !model.default_colors().iter().any(|entry| entry.name == *name))
        .collect::<Vec<_>>();

    let open_modal = {
        let modal_open = modal_open.clone();
        let draft = draft.clone();
        let error_message = error_message.clone();
        let context_menu = context_menu.clone();
        Callback::from(move |_| {
            draft.set(CustomColorDraft::default());
            error_message.set(None);
            context_menu.set(None);
            modal_open.set(true);
        })
    };

    let close_modal = {
        let modal_open = modal_open.clone();
        let error_message = error_message.clone();
        let context_menu = context_menu.clone();
        Callback::from(move |_| {
            error_message.set(None);
            context_menu.set(None);
            modal_open.set(false);
        })
    };

    let close_context_menu = {
        let context_menu = context_menu.clone();
        Callback::from(move |_e: MouseEvent| context_menu.set(None))
    };

    let update_name = {
        let draft = draft.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                let mut next = (*draft).clone();
                next.name = input.value();
                draft.set(next);
            }
        })
    };

    let update_hex = |setter: fn(&mut CustomColorDraft) -> &mut String| {
        let draft = draft.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                let mut next = (*draft).clone();
                *setter(&mut next) = input.value();
                draft.set(next);
            }
        })
    };

    let update_save_for_later = {
        let draft = draft.clone();
        Callback::from(move |e: Event| {
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                let mut next = (*draft).clone();
                next.save_for_later = input.checked();
                draft.set(next);
            }
        })
    };

    let on_save = {
        let model = model.clone();
        let saved_custom_colors = saved_custom_colors.clone();
        let draft = draft.clone();
        let error_message = error_message.clone();
        let modal_open = modal_open.clone();
        let context_menu = context_menu.clone();
        let on_select = props.on_select.clone();
        Callback::from(move |_| {
            let new_color = (*draft).to_scheme();
            if new_color.name.is_empty() {
                error_message.set(Some("Give the custom color a name before saving.".to_string()));
                return;
            }

            let mut next_model = (*model).clone();
            if !next_model.add_custom_color(new_color.clone()) {
                error_message.set(Some(
                    "That color name already exists. Pick a unique name.".to_string(),
                ));
                return;
            }

            on_select.emit(new_color.clone());
            if draft.save_for_later {
                let mut next_saved = (*saved_custom_colors).clone();
                next_saved.push(new_color.clone());
                persist_saved_custom_colors(&next_saved);
                saved_custom_colors.set(next_saved);
            }
            model.set(next_model);
            error_message.set(None);
            context_menu.set(None);
            modal_open.set(false);
        })
    };

    let on_custom_context_menu = {
        let context_menu = context_menu.clone();
        Callback::from(move |(name, e): (String, MouseEvent)| {
            e.prevent_default();
            context_menu.set(Some((name, e.client_x(), e.client_y())));
        })
    };

    let on_delete_custom = {
        let model = model.clone();
        let saved_custom_colors = saved_custom_colors.clone();
        let on_select = props.on_select.clone();
        let selected_name = props.selected_name.clone();
        let context_menu = context_menu.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let Some((name, _, _)) = (*context_menu).clone() else {
                return;
            };
            let mut next_model = (*model).clone();
            if !next_model.remove_custom_color(&name) {
                return;
            }

            if selected_name == name {
                if let Some(fallback) = next_model.default_colors().first().cloned() {
                    on_select.emit(fallback);
                }
            }

            let mut next_saved = (*saved_custom_colors).clone();
            let saved_len = next_saved.len();
            next_saved.retain(|color| color.name != name);
            if next_saved.len() != saved_len {
                persist_saved_custom_colors(&next_saved);
                saved_custom_colors.set(next_saved);
            }

            model.set(next_model);
            context_menu.set(None);
        })
    };

    let keep_menu_open = Callback::from(move |e: MouseEvent| {
        e.prevent_default();
        e.stop_propagation();
    });

    html! {
        <EditorGroup
            title="Colors"
            class={style::BRICK_SETTINGS_EDITOR_GROUP_CLASS}
            content_class={style::BRICK_SETTINGS_EDITOR_GROUP_CONTENT}
        >
            <div class={style::COLOR_VIEW_SHELL} onclick={close_context_menu}>
                <ColorView
                    colors={colors}
                    custom_color_names={custom_color_names}
                    selected_name={props.selected_name.clone()}
                    on_select={props.on_select.clone()}
                    on_add_custom={open_modal}
                    on_custom_context_menu={on_custom_context_menu}
                />
                if let Some((_, x, y)) = &*context_menu {
                    <div
                        class={style::CONTENT_GROUP_CONTEXT_MENU}
                        style={format!("left:{}px; top:{}px;", x, y)}
                        onmousedown={keep_menu_open}
                    >
                        <button
                            class={style::CONTENT_GROUP_CONTEXT_ITEM}
                            type="button"
                            onmousedown={on_delete_custom}
                        >
                            {"Delete"}
                        </button>
                    </div>
                }
            </div>
            if *modal_open {
                <Modal
                    title="Create Custom Color"
                    hint="Add a reusable color scheme for this session or save it for later on this device."
                    class={style::COLOR_MODAL_ROOT}
                    body_class={style::COLOR_MODAL_BODY}
                    on_close={close_modal.clone()}
                >
                    <div class={style::COLOR_MODAL_GRID}>
                        <div class={style::COLOR_MODAL_FORM}>
                            <label class={style::COLOR_MODAL_FIELD}>
                                <span class={style::COLOR_MODAL_LABEL}>{"Name"}</span>
                                <input
                                    class={style::COLOR_MODAL_TEXT_INPUT}
                                    type="text"
                                    value={draft.name.clone()}
                                    placeholder="Salami Red"
                                    oninput={update_name}
                                />
                            </label>
                            <div class={style::COLOR_MODAL_FIELD_ROW}>
                                <label class={style::COLOR_MODAL_FIELD}>
                                    <span class={style::COLOR_MODAL_LABEL}>{"Main color"}</span>
                                    <span class={style::COLOR_MODAL_COLOR_INPUT_WRAP}>
                                        <input
                                            class={style::COLOR_MODAL_COLOR_INPUT}
                                            type="color"
                                            value={draft.color.clone()}
                                            oninput={update_hex(|draft| &mut draft.color)}
                                        />
                                        <span class={style::COLOR_MODAL_COLOR_VALUE}>{draft.color.clone()}</span>
                                    </span>
                                </label>
                                <label class={style::COLOR_MODAL_FIELD}>
                                    <span class={style::COLOR_MODAL_LABEL}>{"Shade"}</span>
                                    <span class={style::COLOR_MODAL_COLOR_INPUT_WRAP}>
                                        <input
                                            class={style::COLOR_MODAL_COLOR_INPUT}
                                            type="color"
                                            value={draft.shade.clone()}
                                            oninput={update_hex(|draft| &mut draft.shade)}
                                        />
                                        <span class={style::COLOR_MODAL_COLOR_VALUE}>{draft.shade.clone()}</span>
                                    </span>
                                </label>
                            </div>
                            <div class={style::COLOR_MODAL_FIELD_ROW}>
                                <label class={style::COLOR_MODAL_FIELD}>
                                    <span class={style::COLOR_MODAL_LABEL}>{"Border"}</span>
                                    <span class={style::COLOR_MODAL_COLOR_INPUT_WRAP}>
                                        <input
                                            class={style::COLOR_MODAL_COLOR_INPUT}
                                            type="color"
                                            value={draft.border.clone()}
                                            oninput={update_hex(|draft| &mut draft.border)}
                                        />
                                        <span class={style::COLOR_MODAL_COLOR_VALUE}>{draft.border.clone()}</span>
                                    </span>
                                </label>
                                <label class={style::COLOR_MODAL_FIELD}>
                                    <span class={style::COLOR_MODAL_LABEL}>{"Text"}</span>
                                    <span class={style::COLOR_MODAL_COLOR_INPUT_WRAP}>
                                        <input
                                            class={style::COLOR_MODAL_COLOR_INPUT}
                                            type="color"
                                            value={draft.text.clone()}
                                            oninput={update_hex(|draft| &mut draft.text)}
                                        />
                                        <span class={style::COLOR_MODAL_COLOR_VALUE}>{draft.text.clone()}</span>
                                    </span>
                                </label>
                            </div>
                            <label class={style::COLOR_MODAL_CHECKBOX_ROW}>
                                <input
                                    class={style::COLOR_MODAL_CHECKBOX}
                                    type="checkbox"
                                    checked={draft.save_for_later}
                                    onchange={update_save_for_later}
                                />
                                <span class={style::COLOR_MODAL_CHECKBOX_TEXT}>
                                    <span class={style::COLOR_MODAL_CHECKBOX_TITLE}>{"Save for later on this device"}</span>
                                    <span class={style::COLOR_MODAL_CHECKBOX_HINT}>{"Stored in browser storage so it shows up again next time."}</span>
                                </span>
                            </label>
                            if let Some(message) = &*error_message {
                                <div class={style::COLOR_MODAL_ERROR}>{message.clone()}</div>
                            }
                            <div class={style::COLOR_MODAL_ACTIONS}>
                                <button
                                    class={style::COLOR_MODAL_BUTTON}
                                    type="button"
                                    onclick={close_modal}
                                >
                                    {"Cancel"}
                                </button>
                                <button
                                    class={classes!(
                                        style::COLOR_MODAL_BUTTON,
                                        style::COLOR_MODAL_BUTTON_PRIMARY,
                                    )}
                                    type="button"
                                    onclick={on_save}
                                >
                                    {"Save color"}
                                </button>
                            </div>
                        </div>
                        <div class={style::COLOR_MODAL_PREVIEW}>
                            <div class={style::COLOR_MODAL_PREVIEW_TITLE}>{"Preview"}</div>
                            <div class={style::COLOR_MODAL_PREVIEW_CARD}>
                                <div class={style::COLOR_MODAL_PREVIEW_NAME}>
                                    {
                                        if draft.name.trim().is_empty() {
                                            "Custom color".to_string()
                                        } else {
                                            draft.name.trim().to_string()
                                        }
                                    }
                                </div>
                                <div class={style::COLOR_MODAL_PREVIEW_SWATCH_ROW}>
                                    <div
                                        class={style::COLOR_MODAL_PREVIEW_SWATCH}
                                        style={format!(
                                            "background:{};color:{};border:3px solid {};",
                                            draft.color, draft.text, draft.border
                                        )}
                                    >
                                        {"abc"}
                                    </div>
                                    <div
                                        class={style::COLOR_MODAL_PREVIEW_SWATCH}
                                        style={format!(
                                            "background:{};color:{};border:3px solid {};",
                                            draft.shade, draft.text, draft.border
                                        )}
                                    >
                                        {"abc"}
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </Modal>
            }
        </EditorGroup>
    }
}
