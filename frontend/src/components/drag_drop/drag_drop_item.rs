#[cfg(target_arch = "wasm32")]
use yew::prelude::*;

#[cfg(target_arch = "wasm32")]
#[derive(Properties, PartialEq, Clone)]
pub struct DragDropItemProps {
    pub children: Html,
}

#[cfg(target_arch = "wasm32")]
#[function_component(DragDropItem)]
pub fn drag_drop_item(props: &DragDropItemProps) -> Html {
    props.children.clone()
}
