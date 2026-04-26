#[cfg(target_arch = "wasm32")]
use crate::components::editor_group::EditorGroup;
#[cfg(target_arch = "wasm32")]
use crate::components::icon_button::IconButton;
#[cfg(target_arch = "wasm32")]
use shared::brick::base::{DROP_MARKER, VARIABLE_MARKER};
#[cfg(target_arch = "wasm32")]
use yew::prelude::*;

#[cfg(target_arch = "wasm32")]
const ICON_ADD: &str = include_str!("../../../res/addBrick.svg");
#[cfg(target_arch = "wasm32")]
#[derive(Properties, PartialEq)]
pub struct ContentGroupProps {
    pub content: String,
    pub on_content_input: Callback<String>,
    pub on_add_to_tutorial: Callback<MouseEvent>,
}

#[cfg(target_arch = "wasm32")]
#[function_component(ContentGroup)]
pub fn content_group(props: &ContentGroupProps) -> Html {
    let textarea_ref = use_node_ref();
    let selection = use_state(|| (0_u32, 0_u32));

    let sync_selection = {
        let textarea_ref = textarea_ref.clone();
        let selection = selection.clone();
        Callback::from(move |_| {
            if let Some(input) = textarea_ref.cast::<web_sys::HtmlTextAreaElement>() {
                let start = input.selection_start().ok().flatten().unwrap_or(0);
                let end = input.selection_end().ok().flatten().unwrap_or(start);
                selection.set((start, end));
            }
        })
    };

    let on_input = {
        let on_content_input = props.on_content_input.clone();
        let sync_selection = sync_selection.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlTextAreaElement>() {
                on_content_input.emit(input.value());
            }
            sync_selection.emit(());
        })
    };

    let on_select = {
        let sync_selection = sync_selection.clone();
        Callback::from(move |_e: Event| {
            sync_selection.emit(());
        })
    };

    let on_keyup = {
        let sync_selection = sync_selection.clone();
        Callback::from(move |_e: KeyboardEvent| {
            sync_selection.emit(());
        })
    };

    let on_mouseup = {
        let sync_selection = sync_selection.clone();
        Callback::from(move |_e: MouseEvent| {
            sync_selection.emit(());
        })
    };

    let wrap_selection = {
        let textarea_ref = textarea_ref.clone();
        let selection = selection.clone();
        let on_content_input = props.on_content_input.clone();
        let content = props.content.clone();
        move |marker: &'static str| {
            let (start, end) = *selection;
            if start >= end {
                return;
            }

            let start = start as usize;
            let end = end as usize;
            let Some(selected) = content.get(start..end) else {
                return;
            };

            let updated = format!(
                "{}{}{}{}{}",
                &content[..start],
                marker,
                selected,
                marker,
                &content[end..]
            );
            let caret_start = (start + marker.len()) as u32;
            let caret_end = (end + marker.len()) as u32;

            if let Some(input) = textarea_ref.cast::<web_sys::HtmlTextAreaElement>() {
                input.set_value(&updated);
                let _ = input.focus();
                let _ = input.set_selection_range(caret_start, caret_end);
            }

            selection.set((caret_start, caret_end));
            on_content_input.emit(updated);
        }
    };

    let on_variable = {
        let wrap_selection = wrap_selection.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            wrap_selection(VARIABLE_MARKER);
        })
    };

    let on_dropdown = {
        let wrap_selection = wrap_selection.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            wrap_selection(DROP_MARKER);
        })
    };

    let has_selection = {
        let (start, end) = *selection;
        start < end
    };

    html! {
        <EditorGroup
            title="Content"
            header={html! {
                <>
                    <IconButton
                        icon={html! { <span class="icon-text">{"*"}</span> }}
                        title="Variable"
                        label="Variable"
                        onclick={on_variable}
                        disabled={!has_selection}
                    />
                    <IconButton
                        icon={html! { <span class="icon-text">{"_"}</span> }}
                        title="Dropdown"
                        label="Dropdown"
                        onclick={on_dropdown}
                        disabled={!has_selection}
                    />
                    <IconButton
                        icon={Html::from_html_unchecked(AttrValue::from(ICON_ADD))}
                        title="Add to tutorial"
                        onclick={props.on_add_to_tutorial.clone()}
                    />
                </>
            }}
        >
            <textarea
                class="brick-settings__content-input"
                placeholder="Content…"
                ref={textarea_ref}
                value={props.content.clone()}
                oninput={on_input}
                onselect={on_select}
                onkeyup={on_keyup}
                onmouseup={on_mouseup}
            />
        </EditorGroup>
    }
}
