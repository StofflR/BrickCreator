#[cfg(target_arch = "wasm32")]
use crate::style;
#[cfg(target_arch = "wasm32")]
use yew::prelude::*;

#[cfg(target_arch = "wasm32")]
#[derive(Properties, PartialEq)]
pub struct CardProps {
    pub title: AttrValue,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub title_class: Classes,
    #[prop_or_default]
    pub content_class: Classes,
    #[prop_or_default]
    pub selectable: bool,
    #[prop_or_default]
    pub selected: bool,
    #[prop_or_default]
    pub mobile_circle: bool,
    #[prop_or_default]
    pub hide_title_on_mobile: bool,
    #[prop_or_default]
    pub hide_content_on_mobile: bool,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub ondblclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub children: Children,
}

#[cfg(target_arch = "wasm32")]
#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
    let card_class = classes!(
        style::CARD_ROOT,
        props.class.clone(),
        props.selectable.then_some(style::CARD_SELECTABLE),
        props.selected.then_some(style::CARD_SELECTED),
        props.mobile_circle.then_some(style::CARD_MOBILE_CIRCLE),
        (props.mobile_circle && props.selected).then_some(style::CARD_MOBILE_CIRCLE_SELECTED),
    );

    html! {
        <div
            class={card_class}
            title={props.title.clone()}
            onclick={props.onclick.clone()}
            ondblclick={props.ondblclick.clone()}
        >
            <h4 class={classes!(
                style::CARD_TITLE,
                props.hide_title_on_mobile.then_some("max-[900px]:hidden"),
                props.title_class.clone(),
            )}>{props.title.clone()}</h4>
            <div class={classes!(
                style::CARD_CONTENT,
                props.hide_content_on_mobile.then_some("max-[900px]:hidden"),
                props.content_class.clone(),
            )}>
                { for props.children.iter() }
            </div>
        </div>
    }
}
