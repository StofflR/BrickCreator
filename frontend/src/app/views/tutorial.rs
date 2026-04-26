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
            <div class="py-6 text-center text-[12px] text-app-text-muted">
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
                class="flex min-h-0 flex-1 flex-col overflow-y-auto p-[2px]"
                item_class="relative mx-[-4px] my-[2%] cursor-pointer rounded-[var(--app-radius)] border-2 border-transparent px-1 py-0 transition-colors select-none hover:border-app-border [&+&]:mt-[-2.5%] [&[draggable='true']]:cursor-grab active:[&[draggable='true']]:cursor-grabbing"
                item_selected_class="border-app-accent"
            >
                {
                    for props.bricks.iter().enumerate().map(|(idx, state)| {
                        let is_selected = Some(idx) == props.selected_index;

                        html_nested! {
                            <DragDropItem>
                                <div class="flex items-center gap-1.5">

                                    if props.export_mode {
                                        <label class="flex items-center justify-center px-1" onclick={{
                                            let on_toggle_export = props.on_toggle_export.clone();
                                            move |event: MouseEvent| {
                                                event.stop_propagation();
                                                on_toggle_export.emit(idx);
                                            }
                                        }}>
                                            <input
                                                class="h-4 w-4 cursor-pointer"
                                                type="checkbox"
                                                checked={props.export_selection.get(idx).copied().unwrap_or(false)}
                                            />
                                        </label>
                                    }

                                    <div class="flex flex-col justify-center gap-0.5">
                                        {
                                            if is_selected {
                                                let len = props.bricks.len();
                                                let on_move = props.on_move.clone();

                                                html! {
                                                    <>
                                                        <button
                                                            class="block rounded-[var(--app-radius)] border border-app-border bg-app-bg px-1.5 py-0.5 text-[14px] transition-colors hover:border-app-accent disabled:cursor-not-allowed disabled:opacity-40"
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
                                                            class="block rounded-[var(--app-radius)] border border-app-border bg-app-bg px-1.5 py-0.5 text-[14px] transition-colors hover:border-app-accent disabled:cursor-not-allowed disabled:opacity-40"
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

                                    <BrickView brick={state.clone()} class="flex-1" />

                                </div>
                            </DragDropItem>
                        }
                    })
                }
            </DragDropList>
        }
    }
}
