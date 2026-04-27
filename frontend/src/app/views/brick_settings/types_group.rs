#[cfg(target_arch = "wasm32")]
use crate::style;
#[cfg(target_arch = "wasm32")]
use crate::app::views::brick_type::BrickTypeView;
#[cfg(target_arch = "wasm32")]
use crate::components::editor_group::EditorGroup;
#[cfg(target_arch = "wasm32")]
use shared::color::ColorScheme;
#[cfg(target_arch = "wasm32")]
use shared::types::BrickType;
#[cfg(target_arch = "wasm32")]
use yew::prelude::*;

#[cfg(target_arch = "wasm32")]
#[derive(Properties, PartialEq)]
pub struct TypesGroupProps {
    pub selected: BrickType,
    pub color_scheme: ColorScheme,
    pub on_select: Callback<BrickType>,
}

#[cfg(target_arch = "wasm32")]
#[function_component(TypesGroup)]
pub fn types_group(props: &TypesGroupProps) -> Html {
    html! {
        <EditorGroup
            title="Brick Types"
            class={style::BRICK_SETTINGS_EDITOR_GROUP_CLASS}
            content_class={style::BRICK_SETTINGS_EDITOR_GROUP_CONTENT}
        >
            <BrickTypeView
                selected={props.selected}
                color_scheme={props.color_scheme.clone()}
                on_select={props.on_select.clone()}
            />
        </EditorGroup>
    }
}
