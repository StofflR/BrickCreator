#[cfg(target_arch = "wasm32")]
use crate::interfaces::brick::BrickState;
#[cfg(target_arch = "wasm32")]
use yew::prelude::*;

#[cfg(target_arch = "wasm32")]
#[derive(Properties, PartialEq)]
pub struct BrickViewProps {
    pub brick: BrickState,
}

#[cfg(target_arch = "wasm32")]
#[function_component(BrickView)]
pub fn brick_view(props: &BrickViewProps) -> Html {
    let svg = props.brick.as_brick().to_svg();
    let data_uri = format!(
        "data:image/svg+xml;charset=utf-8,{}",
        js_sys::encode_uri_component(&svg)
    );
    html! {
        <img class="brick-view" src={data_uri} />
    }
}
