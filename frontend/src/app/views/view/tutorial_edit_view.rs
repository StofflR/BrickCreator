#[cfg(target_arch = "wasm32")]
use crate::{
    app::views::view::brick_view::BrickView,
    components::drag_drop::drag_drop_list::{DragDropItem, DragDropList},
    interfaces::brick::BrickState,
    style,
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
    pub export_selection: Vec<bool>,
    pub on_toggle_export: Callback<usize>,
    pub export_mode: bool,
}

#[cfg(target_arch = "wasm32")]
#[function_component(TutorialEditView)]
pub fn tutorial_edit_view(props: &TutorialEditViewProps) -> Html {
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

    if props.bricks.is_empty() {
        html! {
            <div class={style::TUTORIAL_EMPTY_STATE}>
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
                class={style::TUTORIAL_LIST}
                item_class={style::TUTORIAL_LIST_ITEM}
                item_selected_class={style::TUTORIAL_LIST_ITEM_SELECTED}
            >
                {
                    for props.bricks.iter().enumerate().map(|(idx, state)| {
                        let is_selected = Some(idx) == props.selected_index;

                        html_nested! {
                            <DragDropItem>
                                <div class={style::TUTORIAL_ROW}>

                                    if props.export_mode {
                                        <label class={style::TUTORIAL_EXPORT_LABEL} onclick={{
                                            let on_toggle_export = props.on_toggle_export.clone();
                                            move |event: MouseEvent| {
                                                event.stop_propagation();
                                                on_toggle_export.emit(idx);
                                            }
                                        }}>
                                            <input
                                                class={style::TUTORIAL_EXPORT_CHECKBOX}
                                                type="checkbox"
                                                checked={props.export_selection.get(idx).copied().unwrap_or(false)}
                                            />
                                        </label>
                                    }

                                    <div class={style::TUTORIAL_MOVE_BUTTONS}>
                                        {
                                            if is_selected {
                                                let len = props.bricks.len();
                                                let on_move = props.on_move.clone();

                                                html! {
                                                    <>
                                                        <button
                                                            class={style::TUTORIAL_MOVE_BUTTON}
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
                                                            class={style::TUTORIAL_MOVE_BUTTON}
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

                                    <BrickView brick={state.clone()} class={style::TUTORIAL_BRICK_VIEW} />

                                </div>
                            </DragDropItem>
                        }
                    })
                }
            </DragDropList>
        }
    }
}
