#[cfg(target_arch = "wasm32")]
use crate::components::card::Card;
#[cfg(target_arch = "wasm32")]
use crate::style;
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
    let swatch_style = format!(
        "background:linear-gradient(135deg, {} 0 50%, {} 50% 100%);",
        props.color.color, props.color.shade
    );

    html! {
        <Card
            title={props.color.name.clone()}
            class={style::COLOR_CARD_CLASS}
            content_class={style::COLOR_CARD_CONTENT_CLASS}
            selectable={true}
            selected={props.selected}
            mobile_circle={true}
            hide_title_on_mobile={true}
            onclick={on_click}
        >
            <div
                class={style::COLOR_CARD_SWATCH}
                style={swatch_style}
                aria-hidden="true"
            ></div>
            <div class={style::COLOR_CARD_SWATCH_ROW}>
                <div class={style::COLOR_CARD_SWATCH_VALUE} style={color_style}>{"abc"}</div>
                <div class={style::COLOR_CARD_SWATCH_VALUE} style={shade_style}>{"abc"}</div>
            </div>
        </Card>
    }
}
