#[cfg(target_arch = "wasm32")]
mod colors_group;
#[cfg(target_arch = "wasm32")]
mod content_group;
#[cfg(target_arch = "wasm32")]
mod types_group;

#[cfg(target_arch = "wasm32")]
use crate::interfaces::brick::{BrickState, StateAction};
#[cfg(target_arch = "wasm32")]
use crate::interfaces::utility;
#[cfg(target_arch = "wasm32")]
use colors_group::ColorsGroup;
#[cfg(target_arch = "wasm32")]
use content_group::ContentGroup;
#[cfg(target_arch = "wasm32")]
use shared::color::ColorScheme;
#[cfg(target_arch = "wasm32")]
use shared::types::BrickType;
#[cfg(target_arch = "wasm32")]
use types_group::TypesGroup;
#[cfg(target_arch = "wasm32")]
use yew::prelude::*;

#[cfg(target_arch = "wasm32")]
#[derive(Properties, PartialEq)]
pub struct BrickSettingsViewProps {
    pub brick: BrickState,
    pub dispatcher: UseReducerDispatcher<BrickState>,
    pub on_add_to_tutorial: Callback<MouseEvent>,
}

#[cfg(target_arch = "wasm32")]
#[function_component(BrickSettingsView)]
pub fn brick_settings_view(props: &BrickSettingsViewProps) -> Html {
    let on_content_input = {
        let dispatcher = props.dispatcher.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlTextAreaElement>() {
                dispatcher.dispatch(StateAction::ChangeContent(input.value()));
            }
        })
    };

    let on_brick_type_select = {
        let dispatcher = props.dispatcher.clone();
        Callback::from(move |brick_type: BrickType| {
            dispatcher.dispatch(StateAction::ChangeType(brick_type));
        })
    };

    let on_color_select = {
        let dispatcher = props.dispatcher.clone();
        Callback::from(move |color: ColorScheme| {
            dispatcher.dispatch(StateAction::ChangeColor(color));
        })
    };

    let current = props.brick.as_brick();
    let content_val = current.content.clone();
    let brick_type = props.brick.get_type();
    let color_name = current.color_scheme.name.clone();
    let selected_color = current.color_scheme.clone();



    let on_save_svg = {
        let brick = props.brick.clone();
        Callback::from(move |_| {
            let svg = brick.clone().get_svg();
            if let Err(e) = utility::download_svg(&svg, "brick.svg") {
                web_sys::console::error_1(&format!("SVG error: {e}").into());
            }
        })
    };

    let on_save_png = {
        let brick = props.brick.clone();
        Callback::from(move |_| match brick.clone().get_png(1920) {
            Ok(data) => {
                if let Err(e) = utility::download_png(&data, "brick.png") {
                    web_sys::console::error_1(&format!("PNG error: {e}").into());
                }
            }
            Err(e) => web_sys::console::error_1(&format!("PNG render error: {e}").into()),
        })
    };

    html! {
        <div class="brick-settings-wrap">
            <ContentGroup
                content={content_val}
                on_content_input={on_content_input}
                on_add_to_tutorial={props.on_add_to_tutorial.clone()}
                on_save_svg={on_save_svg}
                on_save_png={on_save_png}
            />
            <div class="brick-settings__bottom">
                <ColorsGroup
                    selected_name={color_name}
                    on_select={on_color_select}
                />
                <TypesGroup
                    selected={brick_type}
                    color_scheme={selected_color}
                    on_select={on_brick_type_select}
                />
            </div>
        </div>
    }
}
