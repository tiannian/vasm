use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub(crate) type App;

    #[wasm_bindgen(js_name = "createApp")]
    pub(crate) fn _create_app(rc: &JsValue, rp: &JsValue) -> App;

    #[wasm_bindgen(method)]
    pub(crate) fn mount(this: &App, rc: &JsValue);
}
