use iced::Task;
use js_sys::{Function, Promise, Reflect, Uint8Array};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;

use super::{WebFontError, is_supported_web_font};

pub(super) fn load(url: String) -> Task<Result<(), WebFontError>> {
    Task::future(fetch(url)).then(|result| match result {
        Ok(bytes) => iced::font::load(bytes).map(|result| result.map_err(WebFontError::FontLoad)),
        Err(error) => Task::done(Err(error)),
    })
}

async fn fetch(url: String) -> Result<Vec<u8>, WebFontError> {
    let global = js_sys::global();
    let fetch = function(&global, "fetch")?;
    let response = fetch
        .call1(&global, &JsValue::from_str(&url))
        .map_err(|_| WebFontError::RequestFailed)?;
    let response = response
        .dyn_into::<Promise>()
        .map_err(|_| WebFontError::MissingBrowserApi("fetch Promise"))?;
    let response = JsFuture::from(response)
        .await
        .map_err(|_| WebFontError::RequestFailed)?;

    let ok = Reflect::get(&response, &JsValue::from_str("ok"))
        .ok()
        .and_then(|value| value.as_bool())
        .ok_or(WebFontError::MissingBrowserApi("Response.ok"))?;

    if !ok {
        let status = Reflect::get(&response, &JsValue::from_str("status"))
            .ok()
            .and_then(|value| value.as_f64())
            .unwrap_or_default() as u16;

        return Err(WebFontError::HttpStatus(status));
    }

    let array_buffer = function(&response, "arrayBuffer")?
        .call0(&response)
        .map_err(|_| WebFontError::ReadFailed)?;
    let array_buffer = array_buffer
        .dyn_into::<Promise>()
        .map_err(|_| WebFontError::MissingBrowserApi("Response.arrayBuffer Promise"))?;
    let array_buffer = JsFuture::from(array_buffer)
        .await
        .map_err(|_| WebFontError::ReadFailed)?;
    let bytes = Uint8Array::new(&array_buffer).to_vec();

    if !is_supported_web_font(&bytes) {
        return Err(WebFontError::UnsupportedFormat);
    }

    Ok(bytes)
}

fn function(target: &JsValue, name: &'static str) -> Result<Function, WebFontError> {
    Reflect::get(target, &JsValue::from_str(name))
        .ok()
        .and_then(|value| value.dyn_into::<Function>().ok())
        .ok_or(WebFontError::MissingBrowserApi(name))
}
