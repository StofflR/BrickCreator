#[cfg(target_arch = "wasm32")]
use yew::prelude::*;

#[cfg(target_arch = "wasm32")]
#[derive(Properties, PartialEq)]
pub struct CardProps {
    pub title: AttrValue,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub selectable: bool,
    #[prop_or_default]
    pub selected: bool,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub children: Children,
}

#[cfg(target_arch = "wasm32")]
#[function_component(Card)]
pub fn card(props: &CardProps) -> Html {
    let card_class = classes!(
        "card",
        props.class.clone(),
        props.selectable.then_some("card--selectable"),
        props.selected.then_some("card--selected"),
    );

    html! {
        <div class={card_class} onclick={props.onclick.clone()}>
            <h4 class="card-title">{props.title.clone()}</h4>
            <div class="card-content">
                { for props.children.iter() }
            </div>
        </div>
    }
}
