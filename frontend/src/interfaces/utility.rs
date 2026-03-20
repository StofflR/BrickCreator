#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[allow(deprecated)]
#[cfg(target_arch = "wasm32")]
use web_sys::{Blob, BlobPropertyBag, HtmlAnchorElement, Url};

/// Trigger a file download in the browser.
#[cfg(target_arch = "wasm32")]
fn download_blob(blob: &Blob, filename: &str) -> Result<(), String> {
    let url = Url::create_object_url_with_blob(blob).map_err(|e| format!("{e:?}"))?;

    let document = web_sys::window()
        .and_then(|w| w.document())
        .ok_or("No document")?;
    let window = web_sys::window().ok_or("No window")?;

    let anchor: HtmlAnchorElement = document
        .create_element("a")
        .map_err(|e| format!("{e:?}"))?
        .dyn_into()
        .map_err(|e| format!("{e:?}"))?;

    anchor.set_href(&url);
    anchor.set_download(filename);
    anchor.set_attribute("style", "display:none").ok();

    let body = document.body().ok_or("No body")?;
    body.append_child(&anchor).map_err(|e| format!("{e:?}"))?;
    anchor.click();
    body.remove_child(&anchor).map_err(|e| format!("{e:?}"))?;

    // Some browsers cancel downloads if we revoke the URL too early.
    let url_to_revoke = url.clone();
    let revoke = wasm_bindgen::closure::Closure::once(move || {
        let _ = Url::revoke_object_url(&url_to_revoke);
    });
    window
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            revoke.as_ref().unchecked_ref(),
            60_000,
        )
        .map_err(|e| format!("{e:?}"))?;
    revoke.forget();
    Ok(())
}

/// Download raw PNG bytes as a `.png` file.
#[cfg(target_arch = "wasm32")]
pub fn download_png(data: &[u8], filename: &str) -> Result<(), String> {
    let uint8 = js_sys::Uint8Array::from(data);
    let parts = js_sys::Array::new();
    parts.push(&uint8.buffer());

    let mut opts = BlobPropertyBag::new();
    #[allow(deprecated)]
    opts.type_("image/png");

    let blob = Blob::new_with_buffer_source_sequence_and_options(&parts, &opts)
        .map_err(|e| format!("{e:?}"))?;

    download_blob(&blob, filename)
}

/// Download an SVG string as a `.svg` file.
#[cfg(target_arch = "wasm32")]
pub fn download_svg(svg: &str, filename: &str) -> Result<(), String> {
    let parts = js_sys::Array::new();
    parts.push(&JsValue::from_str(svg));

    let mut opts = BlobPropertyBag::new();
    #[allow(deprecated)]
    opts.type_("image/svg+xml");

    let blob =
        Blob::new_with_str_sequence_and_options(&parts, &opts).map_err(|e| format!("{e:?}"))?;

    download_blob(&blob, filename)
}

/// Download a JSON string as a `.json` file.
#[cfg(target_arch = "wasm32")]
pub fn download_json(json: &str, filename: &str) -> Result<(), String> {
    let parts = js_sys::Array::new();
    parts.push(&JsValue::from_str(json));

    let mut opts = BlobPropertyBag::new();
    #[allow(deprecated)]
    opts.type_("application/json");

    let blob =
        Blob::new_with_str_sequence_and_options(&parts, &opts).map_err(|e| format!("{e:?}"))?;

    download_blob(&blob, filename)
}

/// Upload a JSON file via a hidden `<input type="file">`.
///
/// Returns a `Callback<String>` wrapper: the provided `on_load` callback will
/// be invoked with the file contents once the user picks a file.
#[cfg(target_arch = "wasm32")]
pub fn upload_json(on_load: yew::Callback<String>) {
    use wasm_bindgen::closure::Closure;

    let document = match web_sys::window().and_then(|w| w.document()) {
        Some(d) => d,
        None => return,
    };

    let input: web_sys::HtmlInputElement = match document
        .create_element("input")
        .ok()
        .and_then(|e| e.dyn_into().ok())
    {
        Some(i) => i,
        None => return,
    };

    input.set_type("file");
    input.set_attribute("accept", ".json,application/json").ok();
    input.set_attribute("style", "display:none").ok();

    let body = match document.body() {
        Some(b) => b,
        None => return,
    };
    body.append_child(&input).ok();

    let input_clone = input.clone();
    let on_change = Closure::<dyn FnMut()>::new(move || {
        let files = match input_clone.files() {
            Some(f) => f,
            None => return,
        };
        let file = match files.get(0) {
            Some(f) => f,
            None => return,
        };

        let reader = match web_sys::FileReader::new().ok() {
            Some(r) => r,
            None => return,
        };

        let on_load = on_load.clone();
        let reader_clone = reader.clone();
        let onloadend = Closure::<dyn FnMut()>::new(move || {
            if let Ok(result) = reader_clone.result()
                && let Some(text) = result.as_string()
            {
                on_load.emit(text);
            }
        });

        reader.set_onloadend(Some(onloadend.as_ref().unchecked_ref()));
        onloadend.forget();

        reader.read_as_text(&file).ok();

        // Clean up the input element
        if let Some(parent) = input_clone.parent_node() {
            parent.remove_child(&input_clone).ok();
        }
    });

    input.set_onchange(Some(on_change.as_ref().unchecked_ref()));
    on_change.forget();

    input.click();
}
