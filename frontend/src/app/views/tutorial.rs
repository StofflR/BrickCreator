#[cfg(target_arch = "wasm32")]
use crate::{
    app::views::brick::BrickView,
    components::drag_drop_list::{DragDropItem, DragDropList},
    interfaces::brick::BrickState,
};
#[cfg(target_arch = "wasm32")]
use yew::prelude::*;

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
                { for props.bricks.iter().map(|state| html_nested! {
                    <DragDropItem>
                        <BrickView brick={state.clone()} />
                    </DragDropItem>
                }) }
            </DragDropList>
        }
    }
}
