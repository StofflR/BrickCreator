#[cfg(target_arch = "wasm32")]
use crate::style;
#[cfg(target_arch = "wasm32")]
use yew::prelude::*;

#[cfg(target_arch = "wasm32")]
#[derive(Properties, PartialEq)]
pub struct ModalProps {
    pub title: AttrValue,
    #[prop_or_default]
    pub hint: AttrValue,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub body_class: Classes,
    pub on_close: Callback<MouseEvent>,
    #[prop_or_default]
    pub children: Children,
}

#[cfg(target_arch = "wasm32")]
#[function_component(Modal)]
pub fn modal(props: &ModalProps) -> Html {
    let on_overlay_click = {
        let on_close = props.on_close.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            e.stop_propagation();
            on_close.emit(e);
        })
    };

    let on_modal_click = Callback::from(|e: MouseEvent| {
        e.stop_propagation();
    });

    html! {
        <div class={style::MODAL_OVERLAY} onclick={on_overlay_click.clone()}>
            <div
                class={classes!(
                    style::MODAL_ROOT,
                    props.class.clone(),
                )}
                onclick={on_modal_click}
            >
                <div class={style::MODAL_HEADER}>
                    <div class={style::MODAL_TITLE_ROW}>
                        <div class={style::MODAL_TITLE}>{props.title.clone()}</div>
                        if !props.hint.is_empty() {
                            <div class={style::MODAL_HINT}>{props.hint.clone()}</div>
                        }
                    </div>
                    <button
                        class={style::MODAL_CLOSE_BUTTON}
                        type="button"
                        onclick={props.on_close.clone()}
                        aria-label="Close"
                    >
                        {"×"}
                    </button>
                </div>
                <div class={classes!(
                    style::MODAL_BODY,
                    props.body_class.clone(),
                )}>
                    { for props.children.iter() }
                </div>
            </div>
        </div>
    }
}
