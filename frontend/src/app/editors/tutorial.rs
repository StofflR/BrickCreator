#[cfg(target_arch = "wasm32")]
use crate::app::views::tutorial::TutorialEditView;
#[cfg(target_arch = "wasm32")]
use crate::app::views::tutorial_preview::TutorialPreviewView;
#[cfg(target_arch = "wasm32")]
use crate::app::views::tutorial_settings::TutorialSettingsView;
#[cfg(target_arch = "wasm32")]
use crate::components::editor_group::EditorGroup;
#[cfg(target_arch = "wasm32")]
use crate::interfaces::brick::BrickState;
#[cfg(target_arch = "wasm32")]
use crate::interfaces::tutorial::{TutorialAction, TutorialViewState};
#[cfg(target_arch = "wasm32")]
use crate::interfaces::utility;
#[cfg(target_arch = "wasm32")]
use yew::prelude::*;

#[cfg(target_arch = "wasm32")]
#[derive(Properties, PartialEq)]
pub struct TutorialEditorProps {
    pub brick: BrickState,
    pub tutorial: TutorialViewState,
    pub tutorial_dispatcher: UseReducerDispatcher<TutorialViewState>,
}

#[cfg(target_arch = "wasm32")]
#[function_component(TutorialEditor)]
pub fn tutorial_editor(props: &TutorialEditorProps) -> Html {
    let bricks = props.tutorial.get_brick_state_list();
    let preview_toggle = use_state(|| false);
    let preview = *preview_toggle.clone();
    let preview_data = if preview {
        props.tutorial.get_png(800).ok()
    } else {
        None
    };

    let on_add = {
        let dispatcher = props.tutorial_dispatcher.clone();
        let brick = props.brick.clone();
        Callback::from(move |_: MouseEvent| {
            dispatcher.dispatch(TutorialAction::AddBrick(brick.clone()));
        })
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

    let on_select = {
        let dispatcher = props.tutorial_dispatcher.clone();
        Callback::from(move |index: usize| {
            dispatcher.dispatch(TutorialAction::Select(index));
        })
    };

    let on_move = {
        let dispatcher = props.tutorial_dispatcher.clone();
        Callback::from(move |(from, to): (usize, usize)| {
            dispatcher.dispatch(TutorialAction::MoveEntry(from, to));
        })
    };

    let on_import_json = {
        let dispatcher = props.tutorial_dispatcher.clone();
        Callback::from(move |_: MouseEvent| {
            let dispatcher = dispatcher.clone();
            utility::upload_json(Callback::from(move |text: String| {
                dispatcher.dispatch(TutorialAction::LoadJson(text));
            }));
        })
    };

    let on_export_json = {
        let tutorial = props.tutorial.clone();
        Callback::from(move |_: MouseEvent| {
            let json = tutorial.to_json();
            if let Err(e) = utility::download_json(&json, "tutorial.json") {
                web_sys::console::error_1(&format!("Export error: {e}").into());
            }
        })
    };

    let on_save_png = {
        let tutorial = props.tutorial.clone();
        Callback::from(move |_: MouseEvent| match tutorial.get_png_bytes(1920) {
            Ok(data) => {
                if let Err(e) = utility::download_png(&data, "tutorial.png") {
                    web_sys::console::error_1(&format!("PNG error: {e}").into());
                }
            }
            Err(e) => web_sys::console::error_1(&format!("PNG render error: {e}").into()),
        })
    };

    let has_selection = props.tutorial.selected_index.is_some();
    let selected_index = props.tutorial.selected_index;

    html! {
        <EditorGroup title="Tutorial Editor">
            <div class="tutorial-view">
                <TutorialSettingsView
                    {on_add}
                    {on_remove}
                    {on_apply}
                    {on_toggle_preview}
                    {on_import_json}
                    {on_export_json}
                    {on_save_png}
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
