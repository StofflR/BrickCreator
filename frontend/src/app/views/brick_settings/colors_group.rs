#[cfg(target_arch = "wasm32")]
use crate::app::views::color::ColorView;
#[cfg(target_arch = "wasm32")]
use crate::components::editor_group::EditorGroup;
#[cfg(target_arch = "wasm32")]
use shared::color::ColorScheme;
#[cfg(target_arch = "wasm32")]
use yew::prelude::*;

#[cfg(target_arch = "wasm32")]
#[derive(Properties, PartialEq)]
pub struct ColorsGroupProps {
    pub selected_name: String,
    pub on_select: Callback<ColorScheme>,
}

#[cfg(target_arch = "wasm32")]
#[function_component(ColorsGroup)]
pub fn colors_group(props: &ColorsGroupProps) -> Html {
    html! {
        <EditorGroup title="Colors">
            <ColorView
                selected_name={props.selected_name.clone()}
                on_select={props.on_select.clone()}
            />
        </EditorGroup>
    }
}
