#[cfg(target_arch = "wasm32")]
use crate::app::views::view::brick_view::BrickView;
#[cfg(target_arch = "wasm32")]
use crate::interfaces::brick::{BrickState, StateAction};
#[cfg(target_arch = "wasm32")]
use crate::style;
#[cfg(target_arch = "wasm32")]
use yew::prelude::*;

#[cfg(target_arch = "wasm32")]
#[derive(Properties, PartialEq)]
pub struct BrickPreviewViewProps {
    pub brick: BrickState,
    pub dispatcher: UseReducerDispatcher<BrickState>,
}

#[cfg(target_arch = "wasm32")]
#[function_component(BrickPreviewView)]
pub fn brick_preview_view(props: &BrickPreviewViewProps) -> Html {
    let on_x_slider = {
        let dispatcher = props.dispatcher.clone();
        let current_y = props.brick.as_brick().offset.1;
        Callback::from(move |e: InputEvent| {
            let value: f32 = e
                .target_dyn_into::<web_sys::HtmlInputElement>()
                .and_then(|el| el.value().parse().ok())
                .unwrap_or(0.0);
            dispatcher.dispatch(StateAction::ChangeOffset(value / 100.0, current_y));
        })
    };

    let on_y_slider = {
        let dispatcher = props.dispatcher.clone();
        let current_x = props.brick.as_brick().offset.0;
        Callback::from(move |e: InputEvent| {
            let value: f32 = e
                .target_dyn_into::<web_sys::HtmlInputElement>()
                .and_then(|el| el.value().parse().ok())
                .unwrap_or(0.0);
            dispatcher.dispatch(StateAction::ChangeOffset(current_x, value / 100.0));
        })
    };

    let current = props.brick.as_brick();
    let x_slider_val = format!("{}", (current.offset.0 * 100.0).round() as i32);
    let y_slider_val = format!("{}", (current.offset.1 * 100.0).round() as i32);
    let brick_type = props.brick.get_type();

    html! {
        <div class={style::BRICK_PREVIEW_GRID}>
            <BrickView
                brick={props.brick.clone()}
                class={style::BRICK_PREVIEW_IMAGE}
            />
            <input
                class={style::BRICK_PREVIEW_Y_SLIDER}
                type="range"
                key={format!("y-slider-{:?}", brick_type)}
                min="0" max="100" value={y_slider_val}
                oninput={on_y_slider}
            />
            <input
                class={style::BRICK_PREVIEW_X_SLIDER}
                type="range"
                min="0" max="100" value={x_slider_val}
                oninput={on_x_slider}
            />
        </div>
    }
}
