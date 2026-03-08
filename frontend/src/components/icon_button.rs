#[cfg(target_arch = "wasm32")]
use yew::prelude::*;

#[cfg(target_arch = "wasm32")]
#[derive(Properties, PartialEq)]
pub struct IconButtonProps {
    pub icon: Html,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub label: AttrValue,
    #[prop_or_default]
    pub title: AttrValue,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub disabled: bool,
}

#[cfg(target_arch = "wasm32")]
#[function_component(IconButton)]
pub fn icon_button(props: &IconButtonProps) -> Html {
    let button_class = classes!(
        "icon-button",
        props.class.clone(),
        props.disabled.then_some("icon-button--disabled"),
    );

    let onclick = {
        let cb = props.onclick.clone();
        let disabled = props.disabled;
        Callback::from(move |e: MouseEvent| {
            if !disabled {
                cb.emit(e);
            }
        })
    };

    html! {
        <button
            class={button_class}
            onclick={onclick}
            disabled={props.disabled}
            aria-label={props.label.clone()}
            title={props.title.clone()}
            type="button"
        >
            { props.icon.clone() }
        </button>
    }
}
