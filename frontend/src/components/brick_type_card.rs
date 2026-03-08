#[cfg(target_arch = "wasm32")]
use crate::components::card::Card;
#[cfg(target_arch = "wasm32")]
use crate::interfaces::brick_type::BrickTypeEntry;
#[cfg(target_arch = "wasm32")]
use shared::types::BrickType;
#[cfg(target_arch = "wasm32")]
use yew::prelude::*;

#[cfg(target_arch = "wasm32")]
#[derive(Properties, PartialEq)]
pub struct BrickTypeCardProps {
    pub brick_type: BrickTypeEntry,
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

    html! {
        <Card
            title={props.brick_type.label}
            selectable={true}
            selected={props.selected}
            onclick={on_click}
        >
            <p class="card-label">{"Brick Type"}</p>
        </Card>
    }
}
