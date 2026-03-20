#[cfg(target_arch = "wasm32")]
use crate::{
    app::views::brick::BrickView,
    components::drag_drop_list::{DragDropItem, DragDropList},
    interfaces::brick::BrickState,
};

#[cfg(target_arch = "wasm32")]
use yew::prelude::*;

#[cfg(target_arch = "wasm32")]
use web_sys::KeyboardEvent;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{JsCast, closure::Closure};

#[cfg(target_arch = "wasm32")]
#[derive(Properties, PartialEq)]
pub struct TutorialEditViewProps {
    pub bricks: Vec<BrickState>,
    #[prop_or_default]
    pub selected_index: Option<usize>,
    pub on_select: Callback<usize>,
    pub on_move: Callback<(usize, usize)>,
}

#[cfg(target_arch = "wasm32")]
#[function_component(TutorialEditView)]
pub fn tutorial_edit_view(props: &TutorialEditViewProps) -> Html {
    // ── Tastatursteuerung ───────────────────────────────────────
    {
        let on_move = props.on_move.clone();
        let selected_index = props.selected_index;
        let len = props.bricks.len();

        use_effect_with((selected_index, len), move |(selected_index, len)| {
            let on_move = on_move.clone();
            let selected_index = *selected_index;
            let len = *len;

            let handler = Closure::<dyn FnMut(_)>::new(move |event: KeyboardEvent| {
                if let Some(i) = selected_index {
                    match event.key().as_str() {
                        "ArrowUp" => {
                            if i > 0 {
                                on_move.emit((i, i - 1));
                            }
                        }
                        "ArrowDown" => {
                            if i + 1 < len {
                                on_move.emit((i, i + 1));
                            }
                        }
                        _ => {}
                    }
                }
            });

            let window = web_sys::window().unwrap();

            window
                .add_event_listener_with_callback("keydown", handler.as_ref().unchecked_ref())
                .unwrap();

            move || {
                window
                    .remove_event_listener_with_callback(
                        "keydown",
                        handler.as_ref().unchecked_ref(),
                    )
                    .unwrap();
            }
        });
    }

    // ── Render ─────────────────────────────────────────────────
    if props.bricks.is_empty() {
        html! {
            <div class="tutorial-view__empty">
                { "Add bricks to build your tutorial" }
            </div>
        }
    } else {
        html! {
            <DragDropList
                item_count={props.bricks.len()}
                selected_index={props.selected_index}
                on_select={props.on_select.clone()}
                on_move={props.on_move.clone()}
                class="tutorial-view__brick-list"
                item_class="tutorial-view__brick-item"
                item_selected_class="tutorial-view__brick-item--selected"
            >
                {
                    for props.bricks.iter().enumerate().map(|(idx, state)| {
                        let is_selected = Some(idx) == props.selected_index;

                        html_nested! {
                            <DragDropItem>
                                <div class="brick-row">

                                    // ── Pfeile links ───────────────────────
                                    <div class="brick-controls">
                                        {
                                            if is_selected {
                                                let len = props.bricks.len();
                                                let on_move = props.on_move.clone();

                                                html! {
                                                    <>
                                                        <button
                                                            onclick={{
                                                                let on_move = on_move.clone();
                                                                move |event: MouseEvent| {
                                                                    event.stop_propagation();
                                                                    if idx > 0 {
                                                                        on_move.emit((idx, idx - 1));
                                                                    }
                                                                }
                                                            }}
                                                            disabled={idx == 0}
                                                        >
                                                            { "↑" }
                                                        </button>

                                                        <button
                                                            onclick={{
                                                                let on_move = on_move.clone();
                                                                move |event: MouseEvent| {
                                                                    event.stop_propagation();
                                                                    if idx + 1 < len {
                                                                        on_move.emit((idx, idx + 1));
                                                                    }
                                                                }
                                                            }}
                                                            disabled={idx + 1 >= len}
                                                        >
                                                            { "↓" }
                                                        </button>
                                                    </>
                                                }
                                            } else {
                                                html! {}
                                            }
                                        }
                                    </div>

                                    // ── Brick ─────────────────────────────
                                    <BrickView brick={state.clone()} />

                                </div>
                            </DragDropItem>
                        }
                    })
                }
            </DragDropList>
        }
    }
}
