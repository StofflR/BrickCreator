#[cfg(target_arch = "wasm32")]
use crate::components::editor_group::EditorGroup;
#[cfg(target_arch = "wasm32")]
use crate::components::icon_button::IconButton;
#[cfg(target_arch = "wasm32")]
use yew::prelude::*;

#[cfg(target_arch = "wasm32")]
const ICON_ADD: &str = include_str!("../../../res/add.svg");
#[cfg(target_arch = "wasm32")]
const ICON_IMAGE: &str = include_str!("../../../res/image.svg");
#[cfg(target_arch = "wasm32")]
const ICON_FILE_PNG: &str = include_str!("../../../res/file_png.svg");

#[cfg(target_arch = "wasm32")]
#[derive(Properties, PartialEq)]
pub struct ContentGroupProps {
    pub content: String,
    pub on_content_input: Callback<InputEvent>,
    pub on_add_to_tutorial: Callback<MouseEvent>,
    pub on_save_svg: Callback<MouseEvent>,
    pub on_save_png: Callback<MouseEvent>,
}

#[cfg(target_arch = "wasm32")]
#[function_component(ContentGroup)]
pub fn content_group(props: &ContentGroupProps) -> Html {
    html! {
        <EditorGroup title="Content">
            <textarea
                class="brick-settings__content-input"
                placeholder="Content…"
                value={props.content.clone()}
                oninput={props.on_content_input.clone()}
            />
            <div class="brick-settings__buttons">
                <IconButton
                    icon={Html::from_html_unchecked(AttrValue::from(ICON_ADD))}
                    title="Add to tutorial"
                    onclick={props.on_add_to_tutorial.clone()}
                />
                <IconButton
                    icon={Html::from_html_unchecked(AttrValue::from(ICON_IMAGE))}
                    title="Save SVG"
                    onclick={props.on_save_svg.clone()}
                />
                <IconButton
                    icon={Html::from_html_unchecked(AttrValue::from(ICON_FILE_PNG))}
                    title="Save PNG"
                    onclick={props.on_save_png.clone()}
                />
            </div>
        </EditorGroup>
    }
}
