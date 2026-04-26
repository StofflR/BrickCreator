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
        "flex",
        "h-16",
        "w-[90px]",
        "shrink-0",
        "flex-col",
        "overflow-hidden",
        "rounded-[var(--app-radius)]",
        "border",
        "border-app-border",
        "bg-app-surface-raised",
        "px-[10px]",
        "py-2",
        "max-[900px]:h-auto",
        "max-[900px]:min-h-16",
        "max-[900px]:min-w-[70px]",
        "max-[900px]:max-w-24",
        "max-[900px]:w-auto",
        "max-[900px]:px-2",
        "max-[900px]:py-1.5",
        props.class.clone(),
        props.selectable.then_some("cursor-pointer"),
        props.selectable.then_some("select-none"),
        props.selectable.then_some("hover:border-app-accent"),
        props.selected.then_some("border-app-accent"),
        props
            .selected
            .then_some("bg-[color-mix(in_srgb,var(--app-accent)_15%,var(--app-surface-raised))]"),
        props.mobile_circle.then_some("max-[900px]:h-[30px]"),
        props.mobile_circle.then_some("max-[900px]:w-[30px]"),
        props.mobile_circle.then_some("max-[900px]:min-h-[30px]"),
        props.mobile_circle.then_some("max-[900px]:min-w-[30px]"),
        props.mobile_circle.then_some("max-[900px]:max-w-[30px]"),
        props.mobile_circle.then_some("max-[900px]:items-center"),
        props.mobile_circle.then_some("max-[900px]:justify-center"),
        props.mobile_circle.then_some("max-[900px]:rounded-full"),
        props.mobile_circle.then_some("max-[900px]:border-2"),
        props.mobile_circle.then_some("max-[900px]:bg-transparent"),
        props.mobile_circle.then_some("max-[900px]:p-0"),
        (props.mobile_circle && props.selected).then_some(
            "max-[900px]:bg-[color-mix(in_srgb,var(--app-accent)_18%,var(--app-surface-raised))]"
        ),
        (props.mobile_circle && props.selected).then_some(
            "max-[900px]:shadow-[0_0_0_2px_color-mix(in_srgb,var(--app-accent)_30%,transparent)]"
        ),
    );

    html! {
        <div
            class={card_class}
            onclick={props.onclick.clone()}
            ondblclick={props.ondblclick.clone()}
        >
            <h4 class={classes!(
                "mb-1",
                "overflow-hidden",
                "text-ellipsis",
                "whitespace-nowrap",
                "text-[12px]",
                "font-semibold",
                "text-app-text",
                "max-[900px]:mb-0.5",
                "max-[900px]:text-[11px]",
                props.hide_title_on_mobile.then_some("max-[900px]:hidden"),
                props.title_class.clone(),
            )}>{props.title.clone()}</h4>
            <div class={classes!(
                "flex",
                "flex-col",
                "gap-1",
                props.hide_content_on_mobile.then_some("max-[900px]:hidden"),
                props.content_class.clone(),
            )}>
                { for props.children.iter() }
            </div>
        </div>
    }
}
