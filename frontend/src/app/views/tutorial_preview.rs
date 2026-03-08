#[cfg(target_arch = "wasm32")]
use yew::prelude::*;

#[cfg(target_arch = "wasm32")]
#[derive(Properties, PartialEq)]
pub struct TutorialPreviewViewProps {
    #[prop_or_default]
    pub preview_data: Option<String>,
}

#[cfg(target_arch = "wasm32")]
#[function_component(TutorialPreviewView)]
pub fn tutorial_preview_view(props: &TutorialPreviewViewProps) -> Html {
    if let Some(ref data) = props.preview_data {
        html! {
            <div class="tutorial-view__preview">
                <img
                    class="tutorial-view__preview-img"
                    src={format!("data:image/png;base64,{}", data)}
                    alt="Tutorial preview"
                />
            </div>
        }
    } else {
        html! {
            <div class="tutorial-view__empty">
                { "No bricks to preview" }
            </div>
        }
    }
}
