#[cfg(target_arch = "wasm32")]
use crate::style;
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
            style::EDITOR_GROUP_ROOT,
            props.class.clone(),
        )}>
            <div class={classes!(
                style::EDITOR_GROUP_HEADER,
                props.header_class.clone(),
            )}>
                <span class="text-[12px] font-semibold tracking-[0.06em] text-app-text-muted uppercase">{props.title.clone()}</span>
                <div class="ml-auto flex items-center gap-1.5">
                    { props.header.clone() }
                </div>
            </div>
            <div class={classes!(
                style::EDITOR_GROUP_CONTENT,
                props.content_class.clone(),
            )}>
                { for props.children.iter() }
            </div>
        </div>
    }
}
