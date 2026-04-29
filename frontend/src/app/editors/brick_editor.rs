#[cfg(target_arch = "wasm32")]
use crate::app::views::brick_settings::BrickSettingsView;
#[cfg(target_arch = "wasm32")]
use crate::app::views::view::brick_preview_view::BrickPreviewView;
#[cfg(target_arch = "wasm32")]
use crate::components::editor_group::EditorGroup;
#[cfg(target_arch = "wasm32")]
use crate::interfaces::brick::BrickState;
#[cfg(target_arch = "wasm32")]
use crate::interfaces::tutorial::{TutorialAction, TutorialViewState};
#[cfg(target_arch = "wasm32")]
use yew::prelude::*;

#[cfg(target_arch = "wasm32")]
#[derive(Properties, PartialEq)]
pub struct BrickEditorProps {
    pub brick: BrickState,
    pub dispatcher: UseReducerDispatcher<BrickState>,
    pub tutorial_dispatcher: UseReducerDispatcher<TutorialViewState>,
}

#[cfg(target_arch = "wasm32")]
#[function_component(BrickEditor)]
pub fn brick_editor(props: &BrickEditorProps) -> Html {
    let on_add_to_tutorial = {
        let dispatcher = props.tutorial_dispatcher.clone();
        let brick = props.brick.clone();
        Callback::from(move |_: MouseEvent| {
            dispatcher.dispatch(TutorialAction::AddBrick(brick.clone()));
        })
    };

    html! {
        <EditorGroup title="Brick Editor">
            <BrickPreviewView
                brick={props.brick.clone()}
                dispatcher={props.dispatcher.clone()}
            />
            <BrickSettingsView
                brick={props.brick.clone()}
                dispatcher={props.dispatcher.clone()}
                {on_add_to_tutorial}
            />
        </EditorGroup>
    }
}
