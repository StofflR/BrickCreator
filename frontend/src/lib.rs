pub mod app;
pub mod components;
pub mod interfaces;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start() {
    app::page::mount_app();
}
