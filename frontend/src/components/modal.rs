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
        <div class="fixed inset-0 z-[1000] flex items-center justify-center bg-black/55 p-6" onclick={on_overlay_click.clone()}>
            <div
                class={classes!(
                    "flex",
                    "h-[min(720px,92vh)]",
                    "w-[min(980px,96vw)]",
                    "flex-col",
                    "overflow-hidden",
                    "rounded-[var(--app-radius)]",
                    "border",
                    "border-app-border",
                    "bg-app-surface",
                    props.class.clone(),
                )}
                onclick={on_modal_click}
            >
                <div class="flex shrink-0 items-center justify-between gap-app-gap border-b border-app-border bg-app-surface-raised px-3 py-2.5">
                    <div class="flex min-w-0 items-baseline gap-2.5">
                        <div class="font-bold">{props.title.clone()}</div>
                        if !props.hint.is_empty() {
                            <div class="whitespace-nowrap text-[12px] font-normal text-app-text-muted">{props.hint.clone()}</div>
                        }
                    </div>
                    <button
                        class="h-8 w-8 rounded-[var(--app-radius)] border border-app-border bg-app-surface text-app-text text-[18px] leading-none transition-colors hover:bg-app-border"
                        type="button"
                        onclick={props.on_close.clone()}
                        aria-label="Close"
                    >
                        {"×"}
                    </button>
                </div>
                <div class={classes!(
                    "flex",
                    "min-h-0",
                    "flex-1",
                    "flex-col",
                    "gap-app-gap",
                    "overflow-auto",
                    "p-app-gap",
                    props.body_class.clone(),
                )}>
                    { for props.children.iter() }
                </div>
            </div>
        </div>
    }
}
