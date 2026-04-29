#[cfg(target_arch = "wasm32")]
mod colors_group;
#[cfg(target_arch = "wasm32")]
mod content_group;
#[cfg(target_arch = "wasm32")]
mod types_group;

#[cfg(target_arch = "wasm32")]
pub mod brick_settings_view;

#[cfg(target_arch = "wasm32")]
pub use brick_settings_view::BrickSettingsView;
