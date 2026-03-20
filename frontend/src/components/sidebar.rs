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
const SUN_ICON: &str = include_str!("../res/sun.svg");
#[cfg(target_arch = "wasm32")]
const MOON_ICON: &str = include_str!("../res/moon.svg");

#[cfg(target_arch = "wasm32")]
fn get_document() -> web_sys::Document {
    gloo::utils::document()
}

#[cfg(target_arch = "wasm32")]
fn load_saved_theme() -> bool {
    gloo::utils::window()
        .local_storage()
        .ok()
        .flatten()
        .and_then(|s| s.get_item("theme").ok().flatten())
        .map(|v| v == "light")
        .unwrap_or(false)
}

#[cfg(target_arch = "wasm32")]
fn apply_theme(light: bool) {
    if let Some(el) = get_document().document_element() {
        if light {
            let _ = el.set_attribute("data-theme", "light");
        } else {
            let _ = el.remove_attribute("data-theme");
        }
    }
    if let Ok(Some(storage)) = gloo::utils::window().local_storage() {
        let _ = storage.set_item("theme", if light { "light" } else { "dark" });
    }
}

#[cfg(target_arch = "wasm32")]
#[derive(Properties, PartialEq)]
pub struct SidebarProps {
    pub children: Html,
}

#[cfg(target_arch = "wasm32")]
#[function_component(Sidebar)]
pub fn sidebar(props: &SidebarProps) -> Html {
    let open = use_state(|| {
        gloo::utils::window()
            .inner_width()
            .ok()
            .and_then(|v| v.as_f64())
            .map(|w| w > 900.0)
            .unwrap_or(false)
    });
    let light = use_state(|| {
        let saved = load_saved_theme();
        apply_theme(saved);
        saved
    });
    let touch_start = use_mut_ref(|| None::<(f64, f64)>);

    {
        let open = open.clone();
        let touch_start = touch_start.clone();
        use_effect(move || {
            let window = gloo::utils::window();
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
                drop(start_listener);
                drop(end_listener);
            }
        });
    }

    let toggle = {
        let open = open.clone();
        Callback::from(move |_: MouseEvent| open.set(!*open))
    };

    let toggle_theme = {
        let light = light.clone();
        Callback::from(move |_: MouseEvent| {
            let new_val = !*light;
            apply_theme(new_val);
            light.set(new_val);
        })
    };

    let chevron_icon = if *open {
        Html::from_html_unchecked(AttrValue::from(CHEVRON_RIGHT))
    } else {
        Html::from_html_unchecked(AttrValue::from(CHEVRON_LEFT))
    };

    let theme_icon = if *light {
        Html::from_html_unchecked(AttrValue::from(MOON_ICON))
    } else {
        Html::from_html_unchecked(AttrValue::from(SUN_ICON))
    };

    let sidebar_class = classes!("page__sidebar", (*open).then_some("page__sidebar--open"),);

    html! {
        <div class={sidebar_class}>
            <div class="sidebar-strip">
                <button class="sidebar-toggle" onclick={toggle} type="button">
                    { chevron_icon }
                </button>
                <button
                    class="sidebar-toggle sidebar-theme-toggle"
                    onclick={toggle_theme}
                    type="button"
                    title={if *light { "Switch to dark mode" } else { "Switch to light mode" }}
                >
                    { theme_icon }
                </button>
            </div>
            if *open {
                <div class="sidebar-content">
                    { props.children.clone() }
                </div>
            }
        </div>
    }
}
