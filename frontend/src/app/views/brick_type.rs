#[cfg(target_arch = "wasm32")]
use crate::components::brick_type_card::BrickTypeCard;
#[cfg(target_arch = "wasm32")]
use crate::interfaces::brick_type::{BrickTypeModel, StaticBrickTypeModel};
#[cfg(target_arch = "wasm32")]
use shared::types::BrickType;
#[cfg(target_arch = "wasm32")]
use yew::prelude::*;

#[cfg(target_arch = "wasm32")]
#[derive(Properties, PartialEq)]
pub struct BrickTypeViewProps {
    pub selected: BrickType,
    pub on_select: Callback<BrickType>,
}

#[cfg(target_arch = "wasm32")]
#[function_component(BrickTypeView)]
pub fn brick_type_view(props: &BrickTypeViewProps) -> Html {
    let model = StaticBrickTypeModel;

    html! {
        <div class="brick-type-view">
            { for model.all_types().iter().map(|entry| {
                html! {
                    <BrickTypeCard
                        key={entry.label}
                        brick_type={*entry}
                        selected={entry.kind == props.selected}
                        on_select={props.on_select.clone()}
                    />
                }
            }) }
        </div>
    }
}
