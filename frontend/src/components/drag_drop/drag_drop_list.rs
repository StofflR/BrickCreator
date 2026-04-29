#[cfg(target_arch = "wasm32")]
pub use crate::components::drag_drop::drag_drop_item::{DragDropItem, DragDropItemProps};
#[cfg(target_arch = "wasm32")]
use crate::style;
#[cfg(target_arch = "wasm32")]
use yew::prelude::*;

#[cfg(target_arch = "wasm32")]
#[derive(Properties, PartialEq)]
pub struct DragDropListProps {
    pub item_count: usize,
    pub children: ChildrenWithProps<DragDropItem>,
    #[prop_or_default]
    pub selected_index: Option<usize>,
    pub on_select: Callback<usize>,
    pub on_move: Callback<(usize, usize)>,
    #[prop_or(AttrValue::Static(style::DRAG_DROP_LIST))]
    pub class: AttrValue,
    #[prop_or(AttrValue::Static(style::DRAG_DROP_LIST_ITEM))]
    pub item_class: AttrValue,
    #[prop_or(AttrValue::Static(style::DRAG_DROP_LIST_ITEM_SELECTED))]
    pub item_selected_class: AttrValue,
}

#[cfg(target_arch = "wasm32")]
#[function_component(DragDropList)]
pub fn drag_drop_list(props: &DragDropListProps) -> Html {
    let drag_index = use_state(|| None::<usize>);

    let items = props
        .children
        .iter()
        .enumerate()
        .map(|(i, child)| {
            let selected = props.selected_index == Some(i);
            let on_select = props.on_select.clone();
            let on_move = props.on_move.clone();
            let drag_index = drag_index.clone();

            let class = classes!(
                props.item_class.clone(),
                selected.then_some(props.item_selected_class.clone()),
            );

            let onclick = {
                let on_select = on_select.clone();
                Callback::from(move |_: MouseEvent| {
                    on_select.emit(i);
                })
            };

            let ondragstart = {
                let drag_index = drag_index.clone();
                Callback::from(move |e: DragEvent| {
                    drag_index.set(Some(i));
                    if let Some(dt) = e.data_transfer() {
                        let _ = dt.set_data("text/plain", &i.to_string());
                    }
                })
            };

            let ondragover = Callback::from(|e: DragEvent| {
                e.prevent_default();
            });

            let ondrop = {
                let drag_index = drag_index.clone();
                let on_move = on_move.clone();
                Callback::from(move |e: DragEvent| {
                    e.prevent_default();
                    if let Some(from) = *drag_index
                        && from != i
                    {
                        on_move.emit((from, i));
                    }
                    drag_index.set(None);
                })
            };

            let ondragend = {
                let drag_index = drag_index.clone();
                Callback::from(move |_: DragEvent| {
                    drag_index.set(None);
                })
            };

            html! {
                <div
                    {class}
                    draggable="true"
                    {onclick}
                    {ondragstart}
                    {ondragover}
                    {ondrop}
                    {ondragend}
                >
                    { child }
                </div>
            }
        })
        .collect::<Html>();

    html! {
        <div class={props.class.clone()}>
            { items }
        </div>
    }
}
