#[cfg(target_arch = "wasm32")]
use yew::prelude::*;

#[cfg(target_arch = "wasm32")]
#[derive(Properties, PartialEq)]
pub struct ModalProps {
    pub title: AttrValue,
    #[prop_or_default]
    pub hint: AttrValue,
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
        <div class="modal-overlay" onclick={on_overlay_click.clone()}>
            <div class="modal" onclick={on_modal_click}>
                <div class="modal__header">
                    <div class="modal__title-row">
                        <div class="modal__title">{props.title.clone()}</div>
                        if !props.hint.is_empty() {
                            <div class="modal__hint">{props.hint.clone()}</div>
                        }
                    </div>
                    <button class="modal__close" type="button" onclick={props.on_close.clone()} aria-label="Close">
                        {"×"}
                    </button>
                </div>
                <div class="modal__body">
                    { for props.children.iter() }
                </div>
            </div>
        </div>
    }
}
