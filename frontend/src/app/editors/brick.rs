#[cfg(target_arch = "wasm32")]
use crate::app::views::brick_preview::BrickPreviewView;
#[cfg(target_arch = "wasm32")]
use crate::app::views::brick_settings::BrickSettingsView;
#[cfg(target_arch = "wasm32")]
use crate::components::editor_group::EditorGroup;
#[cfg(target_arch = "wasm32")]
use crate::interfaces::brick::BrickState;
#[cfg(target_arch = "wasm32")]
use yew::prelude::*;

#[cfg(target_arch = "wasm32")]
#[derive(Properties, PartialEq)]
pub struct BrickEditorProps {
    pub brick: BrickState,
    pub dispatcher: UseReducerDispatcher<BrickState>,
}

#[cfg(target_arch = "wasm32")]
#[function_component(BrickEditor)]
pub fn brick_editor(props: &BrickEditorProps) -> Html {
    html! {
        <EditorGroup title="Brick Editor">
            <BrickPreviewView
                    brick={props.brick.clone()}
                    dispatcher={props.dispatcher.clone()}
                />
                <BrickSettingsView
                brick={props.brick.clone()}
                dispatcher={props.dispatcher.clone()}
            />
        </EditorGroup>
    }
}
