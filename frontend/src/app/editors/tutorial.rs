#[cfg(target_arch = "wasm32")]
use crate::app::views::brick_catalog_modal::BrickCatalogModal;
#[cfg(target_arch = "wasm32")]
use crate::app::views::tutorial::TutorialEditView;
#[cfg(target_arch = "wasm32")]
use crate::app::views::tutorial_preview::TutorialPreviewView;
#[cfg(target_arch = "wasm32")]
use crate::app::views::tutorial_settings::TutorialSettingsView;
#[cfg(target_arch = "wasm32")]
use crate::components::editor_group::EditorGroup;
#[cfg(target_arch = "wasm32")]
use crate::interfaces::brick::{BrickState, StateAction};
#[cfg(target_arch = "wasm32")]
use crate::interfaces::tutorial::{TutorialAction, TutorialViewState};
#[cfg(target_arch = "wasm32")]
use yew::prelude::*;

#[cfg(target_arch = "wasm32")]
#[derive(Properties, PartialEq)]
pub struct TutorialEditorProps {
    pub brick: BrickState,
    pub brick_dispatcher: UseReducerDispatcher<BrickState>,
    pub tutorial: TutorialViewState,
    pub tutorial_dispatcher: UseReducerDispatcher<TutorialViewState>,
}

#[cfg(target_arch = "wasm32")]
#[function_component(TutorialEditor)]
pub fn tutorial_editor(props: &TutorialEditorProps) -> Html {
    let bricks = props.tutorial.get_brick_state_list();
    let preview_toggle = use_state(|| false);
    let preview = *preview_toggle.clone();
    let catalog_open = use_state(|| false);
    let preview_data = if preview {
        props.tutorial.get_png(800).ok()
    } else {
        None
    };

    let on_remove = {
        let dispatcher = props.tutorial_dispatcher.clone();
        Callback::from(move |_: MouseEvent| {
            dispatcher.dispatch(TutorialAction::RemoveSelected);
        })
    };

    let on_apply = {
        let dispatcher = props.tutorial_dispatcher.clone();
        let brick = props.brick.clone();
        Callback::from(move |_: MouseEvent| {
            dispatcher.dispatch(TutorialAction::ApplyChanges(brick.clone()));
        })
    };

    let on_toggle_preview = {
        Callback::from(move |_: MouseEvent| {
            preview_toggle.clone().set(!*preview_toggle);
        })
    };

    let on_open_catalog = {
        let catalog_open = catalog_open.clone();
        Callback::from(move |_: MouseEvent| {
            catalog_open.set(true);
        })
    };

    let on_select = {
        let dispatcher = props.tutorial_dispatcher.clone();
        let brick_dispatcher = props.brick_dispatcher.clone();
        let bricks = bricks.clone();
        Callback::from(move |index: usize| {
            dispatcher.dispatch(TutorialAction::Select(index));
            if let Some(brick) = bricks.get(index) {
                brick_dispatcher.dispatch(StateAction::Set(brick.clone()));
            }
        })
    };

    let on_move = {
        let dispatcher = props.tutorial_dispatcher.clone();
        Callback::from(move |(from, to): (usize, usize)| {
            dispatcher.dispatch(TutorialAction::MoveEntry(from, to));
        })
    };





    let has_selection = props.tutorial.selected_index.is_some();
    let selected_index = props.tutorial.selected_index;

    html! {
        <EditorGroup title="Tutorial Editor">
            <div class="tutorial-view">
                if *catalog_open {
                    <BrickCatalogModal
                        on_close={{
                            let catalog_open = catalog_open.clone();
                            Callback::from(move |_: MouseEvent| catalog_open.set(false))
                        }}
                        on_add_brick={{
                            let dispatcher = props.tutorial_dispatcher.clone();
                            let catalog_open = catalog_open.clone();
                            Callback::from(move |brick: BrickState| {
                                dispatcher.dispatch(TutorialAction::AddBrick(brick));
                                catalog_open.set(false);
                            })
                        }}
                    />
                }
                <TutorialSettingsView
                    {on_remove}
                    {on_apply}
                    {on_toggle_preview}
                    {on_open_catalog}
                    {has_selection}
                    show_preview={preview}
                />
                if preview {
                    <TutorialPreviewView preview_data={preview_data} />
                } else {
                    <TutorialEditView
                        bricks={bricks}
                        selected_index={selected_index}
                        on_select={on_select}
                        on_move={on_move}
                    />
                }
            </div>
        </EditorGroup>
    }
}
