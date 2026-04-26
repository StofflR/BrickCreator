#[cfg(target_arch = "wasm32")]
use yew::prelude::*;

#[cfg(target_arch = "wasm32")]
#[derive(Properties, PartialEq)]
pub struct EditorGroupProps {
    pub title: AttrValue,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub header_class: Classes,
    #[prop_or_default]
    pub content_class: Classes,
    #[prop_or_default]
    pub header: Html,
    #[prop_or_default]
    pub children: Children,
}

#[cfg(target_arch = "wasm32")]
#[function_component(EditorGroup)]
pub fn editor_group(props: &EditorGroupProps) -> Html {
    html! {
        <div class={classes!(
            "flex",
            "min-h-0",
            "flex-1",
            "flex-col",
            "overflow-hidden",
            "rounded-[var(--app-radius)]",
            "border",
            "border-app-border",
            "bg-app-surface",
            props.class.clone(),
        )}>
            <div class={classes!(
                "flex",
                "shrink-0",
                "items-center",
                "justify-between",
                "gap-2",
                "border-b",
                "border-app-border",
                "bg-app-surface-raised",
                "px-3",
                "py-2",
                props.header_class.clone(),
            )}>
                <span class="text-[12px] font-semibold tracking-[0.06em] text-app-text-muted uppercase">{props.title.clone()}</span>
                <div class="ml-auto flex items-center gap-1.5">
                    { props.header.clone() }
                </div>
            </div>
            <div class={classes!(
                "flex",
                "min-h-0",
                "flex-1",
                "flex-col",
                "overflow-hidden",
                "p-app-gap",
                props.content_class.clone(),
            )}>
                { for props.children.iter() }
            </div>
        </div>
    }
}
