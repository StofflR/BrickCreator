#[cfg(target_arch = "wasm32")]
use yew::prelude::*;

#[cfg(target_arch = "wasm32")]
#[derive(Properties, PartialEq)]
pub struct EditorGroupProps {
    pub title: AttrValue,
    #[prop_or_default]
    pub header: Html,
    #[prop_or_default]
    pub children: Children,
}

#[cfg(target_arch = "wasm32")]
#[function_component(EditorGroup)]
pub fn editor_group(props: &EditorGroupProps) -> Html {
    html! {
        <div class="editor-group">
            <div class="editor-group__header">
                <span class="editor-group__title">{props.title.clone()}</span>
                <div class="editor-group__header-actions">
                    { props.header.clone() }
                </div>
            </div>
            <div class="editor-group__content">
                { for props.children.iter() }
            </div>
        </div>
    }
}
