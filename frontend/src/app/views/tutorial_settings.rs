#[cfg(target_arch = "wasm32")]
use crate::components::icon_button::IconButton;
#[cfg(target_arch = "wasm32")]
use yew::prelude::*;

#[cfg(target_arch = "wasm32")]
const ICON_ADD: &str = include_str!("../../res/add.svg");
#[cfg(target_arch = "wasm32")]
const ICON_DELETE: &str = include_str!("../../res/delete.svg");
#[cfg(target_arch = "wasm32")]
const ICON_CHECK: &str = include_str!("../../res/check.svg");
#[cfg(target_arch = "wasm32")]
const ICON_PREVIEW: &str = include_str!("../../res/preview.svg");
#[cfg(target_arch = "wasm32")]
const ICON_EDIT: &str = include_str!("../../res/edit.svg");
#[cfg(target_arch = "wasm32")]
const ICON_UPLOAD: &str = include_str!("../../res/upload.svg");
#[cfg(target_arch = "wasm32")]
const ICON_FILE_JSON: &str = include_str!("../../res/file_json.svg");
#[cfg(target_arch = "wasm32")]
const ICON_FILE_PNG: &str = include_str!("../../res/file_png.svg");

#[cfg(target_arch = "wasm32")]
#[derive(Properties, PartialEq)]
pub struct TutorialSettingsViewProps {
    pub on_add: Callback<MouseEvent>,
    pub on_remove: Callback<MouseEvent>,
    pub on_apply: Callback<MouseEvent>,
    pub on_toggle_preview: Callback<MouseEvent>,
    pub on_import_json: Callback<MouseEvent>,
    pub on_export_json: Callback<MouseEvent>,
    pub on_save_png: Callback<MouseEvent>,
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
        <div class="tutorial-view__toolbar">
            <IconButton
                icon={Html::from_html_unchecked(AttrValue::from(ICON_ADD))}
                title="Add brick"
                onclick={props.on_add.clone()}
            />
            <IconButton
                icon={Html::from_html_unchecked(AttrValue::from(ICON_DELETE))}
                title="Remove brick"
                onclick={props.on_remove.clone()}
                disabled={!props.has_selection}
            />
            <IconButton
                icon={Html::from_html_unchecked(AttrValue::from(ICON_CHECK))}
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
                icon={Html::from_html_unchecked(AttrValue::from(ICON_UPLOAD))}
                title="Import JSON"
                onclick={props.on_import_json.clone()}
            />
            <IconButton
                icon={Html::from_html_unchecked(AttrValue::from(ICON_FILE_JSON))}
                title="Export JSON"
                onclick={props.on_export_json.clone()}
            />
            <IconButton
                icon={Html::from_html_unchecked(AttrValue::from(ICON_FILE_PNG))}
                title="Save PNG"
                onclick={props.on_save_png.clone()}
            />
        </div>
    }
}
