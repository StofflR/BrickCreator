#[cfg(target_arch = "wasm32")]
use gloo::events::EventListener;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use yew::prelude::*;

#[cfg(target_arch = "wasm32")]
const CHEVRON_RIGHT: &str = include_str!("../res/chevron_right.svg");
#[cfg(target_arch = "wasm32")]
const CHEVRON_LEFT: &str = include_str!("../res/chevron_left.svg");

#[cfg(target_arch = "wasm32")]
#[derive(Properties, PartialEq)]
pub struct SidebarProps {
    pub children: Html,
}

#[cfg(target_arch = "wasm32")]
#[function_component(Sidebar)]
pub fn sidebar(props: &SidebarProps) -> Html {
    let open = use_state(|| true);
    let touch_start = use_mut_ref(|| None::<(f64, f64)>);

    {
        let open = open.clone();
        let touch_start = touch_start.clone();
        use_effect(move || {
            let window = gloo::utils::window();
            let resize_window = window.clone();
            let resize_open = open.clone();
            let resize_listener = EventListener::new(&window, "resize", move |_| {
                let width = resize_window
                    .inner_width()
                    .ok()
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.0);
                if width > 900.0 && !*resize_open {
                    resize_open.set(true);
                }
            });

            let start_window = window.clone();
            let _start_open = open.clone();
            let start_touch = touch_start.clone();
            let start_listener = EventListener::new(&window, "touchstart", move |event| {
                let Some(event) = event.dyn_ref::<web_sys::TouchEvent>() else {
                    return;
                };
                let width = start_window
                    .inner_width()
                    .ok()
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.0);
                if width > 900.0 {
                    return;
                }
                if let Some(touch) = event.touches().get(0) {
                    let start_x = touch.client_x() as f64;
                    let start_y = touch.client_y() as f64;
                    start_touch.borrow_mut().replace((start_x, start_y));
                }
            });

            let end_window = window.clone();
            let end_open = open.clone();
            let end_touch = touch_start.clone();
            let end_listener = EventListener::new(&window, "touchend", move |event| {
                let Some(event) = event.dyn_ref::<web_sys::TouchEvent>() else {
                    return;
                };
                let width = end_window
                    .inner_width()
                    .ok()
                    .and_then(|v| v.as_f64())
                    .unwrap_or(0.0);
                if width > 900.0 {
                    return;
                }
                let Some((start_x, start_y)) = end_touch.borrow_mut().take() else {
                    return;
                };
                let Some(touch) = event.changed_touches().get(0) else {
                    return;
                };
                let end_x = touch.client_x() as f64;
                let end_y = touch.client_y() as f64;
                let dx = end_x - start_x;
                let dy = end_y - start_y;
                if dx.abs() < 60.0 || dx.abs() < dy.abs() {
                    return;
                }
                if *end_open && dx > 0.0 {
                    end_open.set(false);
                } else if !*end_open && dx < 0.0 {
                    end_open.set(true);
                }
            });

            move || {
                drop(resize_listener);
                drop(start_listener);
                drop(end_listener);
            }
        });
    }

    let toggle = {
        let open = open.clone();
        Callback::from(move |_: MouseEvent| open.set(!*open))
    };

    let chevron_icon = if *open {
        Html::from_html_unchecked(AttrValue::from(CHEVRON_RIGHT))
    } else {
        Html::from_html_unchecked(AttrValue::from(CHEVRON_LEFT))
    };

    let sidebar_class = classes!(
        "flex",
        "min-w-0",
        "w-[var(--app-toggle-width)]",
        "shrink-0",
        "flex-row",
        "overflow-hidden",
        "border-l",
        "border-app-border",
        "bg-app-surface",
        "transition-[width]",
        "duration-200",
        "ease-in-out",
        "max-[900px]:fixed",
        "max-[900px]:right-0",
        "max-[900px]:top-0",
        "max-[900px]:bottom-0",
        "max-[900px]:z-20",
        "max-[900px]:h-dvh",
        "max-[900px]:w-[var(--app-toggle-width)]",
        "max-[900px]:shadow-[-8px_0_16px_rgba(0,0,0,0.25)]",
        (*open).then_some("w-[calc(var(--app-sidebar-width)+var(--app-toggle-width))]"),
        (*open).then_some("max-[900px]:w-screen"),
    );

    html! {
        <div class={sidebar_class}>
            <div class="flex h-full basis-[var(--app-toggle-width)] flex-col border-r border-app-border max-[900px]:absolute max-[900px]:left-0 max-[900px]:top-0 max-[900px]:bottom-0 max-[900px]:z-[2] max-[900px]:bg-app-surface">
                <button class="flex flex-1 items-center justify-center bg-transparent px-0 py-2 text-app-text transition-colors hover:bg-app-surface-raised" onclick={toggle} type="button">
                    { chevron_icon }
                </button>
            </div>
            if *open {
                <div
                    class="flex min-w-0 flex-1 flex-col overflow-auto w-[var(--app-sidebar-width)] p-app-gap max-[900px]:w-screen max-[900px]:pl-[calc(var(--app-gap)+var(--app-toggle-width))]"
                    aria-hidden="false"
                >
                    { props.children.clone() }
                </div>
            }
        </div>
    }
}
