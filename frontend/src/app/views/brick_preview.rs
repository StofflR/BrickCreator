#[cfg(target_arch = "wasm32")]
use crate::app::views::brick::BrickView;
#[cfg(target_arch = "wasm32")]
use crate::interfaces::brick::{BrickState, StateAction};
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
        <div class="grid flex-none max-[900px]:max-h-[45vh]">
            <BrickView
                brick={props.brick.clone()}
                class="col-start-1 row-start-1 h-full w-full overflow-hidden"
            />
            <input
                class="col-start-2 row-start-1 min-h-0 min-w-0 flex-1 [direction:ltr] [writing-mode:vertical-lr]"
                type="range"
                key={format!("y-slider-{:?}", brick_type)}
                min="0" max="100" value={y_slider_val}
                oninput={on_y_slider}
            />
            <input
                class="col-start-1 row-start-2 min-h-0 min-w-0 flex-1"
                type="range"
                min="0" max="100" value={x_slider_val}
                oninput={on_x_slider}
            />
        </div>
    }
}
