#[cfg(target_arch = "wasm32")]
use crate::style;
#[cfg(target_arch = "wasm32")]
use crate::components::color_card::ColorCard;
#[cfg(target_arch = "wasm32")]
use shared::color::ColorScheme;
#[cfg(target_arch = "wasm32")]
use yew::prelude::*;

#[cfg(target_arch = "wasm32")]
#[derive(Properties, PartialEq)]
pub struct ColorViewProps {
    pub colors: Vec<ColorScheme>,
    pub custom_color_names: Vec<String>,
    pub selected_name: String,
    pub on_select: Callback<ColorScheme>,
    pub on_add_custom: Callback<MouseEvent>,
    pub on_custom_context_menu: Callback<(String, MouseEvent)>,
}

#[cfg(target_arch = "wasm32")]
#[function_component(ColorView)]
pub fn color_view(props: &ColorViewProps) -> Html {
    html! {
        <div class={style::COLOR_VIEW_GRID}>
            { for props.colors.iter().cloned().map(|color| {
                let key = color.name.clone();
                let selected = color.name == props.selected_name;
                let is_custom = props.custom_color_names.iter().any(|name| name == &color.name);
                html! {
                    <ColorCard
                        key={key}
                        color={color}
                        selected={selected}
                        is_custom={is_custom}
                        on_select={props.on_select.clone()}
                        on_context_menu={props.on_custom_context_menu.clone()}
                    />
                }
            }) }
            <button
                class={style::COLOR_ADD_BUTTON}
                type="button"
                onclick={props.on_add_custom.clone()}
                aria-label="Add custom color"
                title="Add custom color"
            >
                <span class={style::COLOR_ADD_BUTTON_ICON} aria-hidden="true">{"+"}</span>
                <span class={style::COLOR_ADD_BUTTON_LABEL}>{"Custom color"}</span>
            </button>
        </div>
    }
}
