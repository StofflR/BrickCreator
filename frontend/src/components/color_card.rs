#[cfg(target_arch = "wasm32")]
use crate::components::card::Card;
#[cfg(target_arch = "wasm32")]
use shared::color::ColorScheme;
#[cfg(target_arch = "wasm32")]
use yew::prelude::*;

#[cfg(target_arch = "wasm32")]
#[derive(Properties, PartialEq)]
pub struct ColorCardProps {
    pub color: ColorScheme,
    pub selected: bool,
    pub on_select: Callback<ColorScheme>,
}

#[cfg(target_arch = "wasm32")]
#[function_component(ColorCard)]
pub fn color_card(props: &ColorCardProps) -> Html {
    let on_click = {
        let on_select = props.on_select.clone();
        let selected = props.color.clone();
        Callback::from(move |_| on_select.emit(selected.clone()))
    };

    let color_style = format!(
        "background:{};color:{};border:3px solid {};",
        props.color.color, props.color.text, props.color.border
    );
    let shade_style = format!(
        "background:{};color:{};border:3px solid {};",
        props.color.shade, props.color.text, props.color.border
    );

    html! {
        <Card
            title={props.color.name.clone()}
            class="card--color"
            selectable={true}
            selected={props.selected}
            onclick={on_click}
        >
            <div class="color-card-content">
                <div class="color-card-sample" style={color_style}>{"abc"}</div>
                <div class="color-card-sample" style={shade_style}>{"abc"}</div>
            </div>
        </Card>
    }
}
