use shared::color::{ALL_COLOR_SCHEMES, ColorScheme};
use yew::{Callback, Html, Properties, function_component, html};

#[derive(Properties, PartialEq)]
pub struct ColorViewProps {
    pub on_select: Callback<ColorScheme>,
}

#[function_component(ColorView)]
pub fn color_view(props: &ColorViewProps) -> Html {
    html! {
        <div class="color-list">
            <h2>{ "Color Schemes" }</h2>
            <div class="color-grid">
                {ALL_COLOR_SCHEMES.iter().map(|scheme| {
                    let on_click = props.on_select.clone();
                    html! {
                        <div
                            class="color-item"
                            onclick={move |_| on_click.emit(*scheme)}
                            style="cursor: pointer;"
                        >
                            <div class="color-name">{ scheme.name }</div>
                            <div class="color-description-row">
                                <div class="color-description"> {"Background"} </div>
                                <div class="color-description"> {"Border"} </div>
                            </div>
                            <div class="color-preview">
                                <div class="color-sample" style={format!(
                                    "background: {}; border: 1px solid {}; color: {};",
                                    scheme.color, scheme.border, scheme.text
                                )}>{ "Abc" }</div>
                                <div class="color-sample-shade" style={format!(
                                    "background: {}; border: 1px solid {};",
                                    scheme.shade, scheme.border
                                )}></div>
                            </div>
                        </div>
                    }
                }).collect::<Html>()}
            </div>
        </div>
    }
}
