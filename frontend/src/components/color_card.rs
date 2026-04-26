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
    let swatch_style = format!(
        "background:linear-gradient(135deg, {} 0 50%, {} 50% 100%);",
        props.color.color, props.color.shade
    );

    html! {
        <Card
            title={props.color.name.clone()}
            class="max-[900px]:border-2"
            content_class="mt-auto"
            selectable={true}
            selected={props.selected}
            mobile_circle={true}
            hide_title_on_mobile={true}
            onclick={on_click}
        >
            <div
                class="hidden h-5 w-5 rounded-full max-[900px]:block"
                style={swatch_style}
                aria-hidden="true"
            ></div>
            <div class="flex w-full items-stretch gap-1 max-[900px]:hidden">
                <div class="flex min-w-0 flex-1 items-center justify-center overflow-hidden rounded-[3px] px-1 py-px text-center text-[10px] leading-none max-[900px]:px-[3px] max-[900px]:py-px max-[900px]:text-[10px]" style={color_style}>{"abc"}</div>
                <div class="flex min-w-0 flex-1 items-center justify-center overflow-hidden rounded-[3px] px-1 py-px text-center text-[10px] leading-none max-[900px]:px-[3px] max-[900px]:py-px max-[900px]:text-[10px]" style={shade_style}>{"abc"}</div>
            </div>
        </Card>
    }
}
