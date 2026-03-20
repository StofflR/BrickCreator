#[cfg(target_arch = "wasm32")]
use std::collections::BTreeMap;

#[cfg(target_arch = "wasm32")]
use yew::prelude::*;

#[cfg(target_arch = "wasm32")]
use crate::app::views::brick::BrickView;
#[cfg(target_arch = "wasm32")]
use crate::components::card::Card;
#[cfg(target_arch = "wasm32")]
use crate::components::editor_group::EditorGroup;
#[cfg(target_arch = "wasm32")]
use crate::components::modal::Modal;
#[cfg(target_arch = "wasm32")]
use crate::interfaces::brick::BrickState;

#[cfg(target_arch = "wasm32")]
fn group_key_from_path(path: &str) -> &str {
    path.split('/').next().unwrap_or("(root)")
}

#[cfg(target_arch = "wasm32")]
fn label_from_path(path: &str) -> &str {
    let file = path.rsplit_once('/').map(|(_, f)| f).unwrap_or(path);
    file.strip_suffix(".json").unwrap_or(file)
}

#[cfg(target_arch = "wasm32")]
#[derive(Properties, PartialEq)]
pub struct BrickCatalogModalProps {
    pub on_close: Callback<MouseEvent>,
    pub on_add_brick: Callback<BrickState>,
}

#[cfg(target_arch = "wasm32")]
#[function_component(BrickCatalogModal)]
pub fn brick_catalog_modal(props: &BrickCatalogModalProps) -> Html {
    let mut groups: BTreeMap<String, Vec<(&'static str, Option<BrickState>)>> = BTreeMap::new();
    for (path, json) in crate::generated::brick_catalog::BRICKS {
        let key = group_key_from_path(path).to_string();
        let brick = match BrickState::from_json(json) {
            Ok(brick) => Some(brick),
            Err(e) => {
                web_sys::console::error_1(&format!("{path}: {e}").into());
                None
            }
        };
        groups.entry(key).or_default().push((*path, brick));
    }

    for entries in groups.values_mut() {
        entries.sort_by(|(a, _), (b, _)| a.cmp(b));
    }

    html! {
        <Modal
            title="Brick Catalog"
            hint="Double-click to add"
            on_close={props.on_close.clone()}
        >
            <div class="brick-catalog">
                { for groups.into_iter().map(|(group, entries)| {
                    html! {
                        <div class="brick-catalog__group" key={group.clone()}>
                            <EditorGroup title={group.clone()}>
                                <div class="brick-catalog__grid">
                                    { for entries.into_iter().map(|(path, brick)| {
                                        let on_add_brick = props.on_add_brick.clone();
                                        let on_dblclick = if let Some(brick) = brick.clone() {
                                            Callback::from(move |_| on_add_brick.emit(brick.clone()))
                                        } else {
                                            Callback::default()
                                        };

                                        html! {
                                            <Card
                                                key={path}
                                                title={label_from_path(path)}
                                                selectable={true}
                                                ondblclick={on_dblclick}
                                            >
                                                <div class="brick-catalog__thumb" title={path}>
                                                    if let Some(brick) = brick {
                                                        <BrickView brick={brick} />
                                                    } else {
                                                        <div class="card-label">{ "Invalid brick" }</div>
                                                    }
                                                </div>
                                            </Card>
                                        }
                                    }) }
                                </div>
                            </EditorGroup>
                        </div>
                    }
                }) }
            </div>
        </Modal>
    }
}
