#[cfg(target_arch = "wasm32")]
fn call_global(name: &str) {
    use wasm_bindgen::{JsCast, JsValue};

    let global = js_sys::global();
    let Ok(value) = js_sys::Reflect::get(&global, &JsValue::from_str(name)) else {
        return;
    };
    let Some(function) = value.dyn_ref::<js_sys::Function>() else {
        return;
    };

    let _ = function.call0(&global);
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn show_mobile_keyboard() {
    call_global("__icedMaterialShowMobileKeyboard");
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn show_mobile_keyboard() {}

#[cfg(target_arch = "wasm32")]
pub(crate) fn hide_mobile_keyboard() {
    call_global("__icedMaterialHideMobileKeyboard");
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn hide_mobile_keyboard() {}
