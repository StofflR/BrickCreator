#[cfg(target_arch = "wasm32")]
use crate::components::icon_button::IconButton;
#[cfg(target_arch = "wasm32")]
use crate::style;
#[cfg(target_arch = "wasm32")]
use yew::prelude::*;

#[cfg(target_arch = "wasm32")]
const ICON_DELETE: &str = include_str!("../../res/delete.svg");
#[cfg(target_arch = "wasm32")]
const ICON_PREVIEW: &str = include_str!("../../res/preview.svg");
#[cfg(target_arch = "wasm32")]
const ICON_EDIT: &str = include_str!("../../res/edit.svg");
#[cfg(target_arch = "wasm32")]
const ICON_BRICK_CATALOG: &str = include_str!("../../res/brickcatalog.svg");

#[cfg(target_arch = "wasm32")]
#[derive(Properties, PartialEq)]
pub struct TutorialSettingsViewProps {
    pub on_remove: Callback<MouseEvent>,
    pub on_apply: Callback<MouseEvent>,
    pub on_toggle_preview: Callback<MouseEvent>,
    pub on_open_catalog: Callback<MouseEvent>,
    pub has_selection: bool,
    pub show_preview: bool,
}

#[cfg(target_arch = "wasm32")]
#[function_component(TutorialSettingsView)]
pub fn tutorial_settings_view(props: &TutorialSettingsViewProps) -> Html {
    let preview_icon = if props.show_preview {
        Html::from_html_unchecked(AttrValue::from(ICON_EDIT))
    } else {
        Html::from_html_unchecked(AttrValue::from(ICON_PREVIEW))
    };

    let preview_title = if props.show_preview {
        "Edit"
    } else {
        "Preview"
    };

    html! {
        <div class={style::TUTORIAL_SETTINGS_ACTIONS}>
            <IconButton
                icon={Html::from_html_unchecked(AttrValue::from(ICON_DELETE))}
                title="Remove brick"
                onclick={props.on_remove.clone()}
                disabled={!props.has_selection}
            />
            <IconButton
                icon={Html::from_html_unchecked(AttrValue::from(ICON_EDIT))}
                title="Apply changes"
                onclick={props.on_apply.clone()}
                disabled={!props.has_selection}
            />
            <IconButton
                icon={preview_icon}
                title={preview_title}
                onclick={props.on_toggle_preview.clone()}
            />
            <IconButton
                icon={Html::from_html_unchecked(AttrValue::from(ICON_BRICK_CATALOG))}
                title="Open brick catalog"
                onclick={props.on_open_catalog.clone()}
            />
        </div>
    }
}
