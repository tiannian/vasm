use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "toRaw")]
    pub(crate) fn _to_raw(proxy: &JsValue) -> JsValue;
}
