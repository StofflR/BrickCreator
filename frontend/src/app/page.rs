#[cfg(target_arch = "wasm32")]
use crate::app::editors::brick::BrickEditor;
#[cfg(target_arch = "wasm32")]
use crate::app::editors::tutorial::TutorialEditor;
#[cfg(target_arch = "wasm32")]
use crate::components::sidebar::Sidebar;
#[cfg(target_arch = "wasm32")]
use crate::interfaces::brick::BrickState;
#[cfg(target_arch = "wasm32")]
use crate::interfaces::tutorial::TutorialViewState;
#[cfg(target_arch = "wasm32")]
use yew::prelude::*;

#[cfg(target_arch = "wasm32")]
#[function_component(App)]
fn app() -> Html {
    let brick = use_reducer(BrickState::default);
    let tutorial = use_reducer(TutorialViewState::default);

    html! {
        <div class="page">
            <div class="page__main">
                <BrickEditor
                    brick={(*brick).clone()}
                    dispatcher={brick.dispatcher()}
                    tutorial_dispatcher={tutorial.dispatcher()}
                />
            </div>
            <Sidebar>
                <TutorialEditor
                    brick={(*brick).clone()}
                    tutorial={(*tutorial).clone()}
                    tutorial_dispatcher={tutorial.dispatcher()}
                />
            </Sidebar>
        </div>
    }
}

#[cfg(target_arch = "wasm32")]
pub fn mount_app() {
    yew::Renderer::<App>::new().render();
}
