#[cfg(target_arch = "wasm32")]
use crate::style;
#[cfg(target_arch = "wasm32")]
use crate::app::views::brick::BrickView;
#[cfg(target_arch = "wasm32")]
use crate::components::card::Card;
#[cfg(target_arch = "wasm32")]
use crate::interfaces::brick::BrickState;
#[cfg(target_arch = "wasm32")]
use crate::interfaces::brick_type::BrickTypeEntry;
#[cfg(target_arch = "wasm32")]
use shared::color::ColorScheme;
#[cfg(target_arch = "wasm32")]
use shared::types::BrickType;
#[cfg(target_arch = "wasm32")]
use yew::prelude::*;

#[cfg(target_arch = "wasm32")]
#[derive(Properties, PartialEq)]
pub struct BrickTypeCardProps {
    pub brick_type: BrickTypeEntry,
    pub color_scheme: ColorScheme,
    pub selected: bool,
    pub on_select: Callback<BrickType>,
}

#[cfg(target_arch = "wasm32")]
#[function_component(BrickTypeCard)]
pub fn brick_type_card(props: &BrickTypeCardProps) -> Html {
    let on_click = {
        let on_select = props.on_select.clone();
        let selected = props.brick_type.kind;
        Callback::from(move |_| on_select.emit(selected))
    };
    let mut preview_brick = BrickState::default();
    preview_brick.change_type(props.brick_type.kind);
    preview_brick.as_mut_brick().color_scheme = props.color_scheme.clone();

    html! {
        <Card
            title={props.brick_type.label}
            class={style::BRICK_TYPE_CARD_CLASS}
            title_class={style::BRICK_TYPE_CARD_TITLE_CLASS}
            selectable={true}
            selected={props.selected}
            onclick={on_click}
        >
            <div class={style::BRICK_TYPE_CARD_PREVIEW_WRAP} aria-hidden="true">
                <BrickView
                    brick={preview_brick}
                    class={style::BRICK_TYPE_CARD_PREVIEW_IMAGE}
                />
            </div>
        </Card>
    }
}
