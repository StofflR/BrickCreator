#[cfg(target_arch = "wasm32")]
use crate::components::editor_group::EditorGroup;
#[cfg(target_arch = "wasm32")]
use crate::components::icon_button::IconButton;
#[cfg(target_arch = "wasm32")]
use yew::prelude::*;

#[cfg(target_arch = "wasm32")]
const ICON_ADD: &str = include_str!("../../../res/addBrick.svg");
#[cfg(target_arch = "wasm32")]
#[derive(Properties, PartialEq)]
pub struct ContentGroupProps {
    pub content: String,
    pub on_content_input: Callback<InputEvent>,
    pub on_add_to_tutorial: Callback<MouseEvent>,
}

#[cfg(target_arch = "wasm32")]
#[function_component(ContentGroup)]
pub fn content_group(props: &ContentGroupProps) -> Html {
    html! {
        <EditorGroup
            title="Content"
            class="flex-none overflow-hidden"
            content_class="flex-none overflow-hidden max-[900px]:items-start"
            header={html! {
                <IconButton
                    icon={Html::from_html_unchecked(AttrValue::from(ICON_ADD))}
                    title="Add to tutorial"
                    onclick={props.on_add_to_tutorial.clone()}
                />
            }}
        >
            <textarea
                class="min-h-[8lh] flex-1 resize-none rounded-[var(--app-radius)] border border-app-border bg-app-surface-raised px-2.5 py-1.5 text-[14px] text-app-text outline-none focus:border-app-accent max-[900px]:max-h-[6lh] max-[900px]:min-h-[4lh]"
                placeholder="Content…"
                value={props.content.clone()}
                oninput={props.on_content_input.clone()}
            />
        </EditorGroup>
    }
}
