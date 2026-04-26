#[cfg(target_arch = "wasm32")]
use crate::components::color_card::ColorCard;
#[cfg(target_arch = "wasm32")]
use crate::interfaces::color::{BrickColorModel, ColorModel};
#[cfg(target_arch = "wasm32")]
use shared::color::ColorScheme;
#[cfg(target_arch = "wasm32")]
use yew::prelude::*;

#[cfg(target_arch = "wasm32")]
#[derive(Properties, PartialEq)]
pub struct ColorViewProps {
    pub selected_name: String,
    pub on_select: Callback<ColorScheme>,
}

#[cfg(target_arch = "wasm32")]
#[function_component(ColorView)]
pub fn color_view(props: &ColorViewProps) -> Html {
    let model = use_state(BrickColorModel::new);
    let colors = (*model).all_colors();

    html! {
        <div class="flex flex-wrap gap-1.5 max-[900px]:h-auto max-[900px]:items-center max-[900px]:gap-2 max-[900px]:overflow-visible">
            { for colors.into_iter().map(|color| {
                let key = color.name.clone();
                let selected = color.name == props.selected_name;
                html! {
                    <ColorCard
                        key={key}
                        color={color}
                        selected={selected}
                        on_select={props.on_select.clone()}
                    />
                }
            }) }
        </div>
    }
}
