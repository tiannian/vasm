use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "onMounted")]
    pub(crate) fn _on_mounted(getter: &Closure<dyn Fn()>);

    #[wasm_bindgen(js_name = "onUpdated")]
    pub(crate) fn _on_update(getter: &Closure<dyn Fn()>);

    #[wasm_bindgen(js_name = "onUnmounted")]
    pub(crate) fn _on_unmounted(getter: &Closure<dyn Fn()>);

    #[wasm_bindgen(js_name = "onBeforeMount")]
    pub(crate) fn _on_before_mount(getter: &Closure<dyn Fn()>);

    #[wasm_bindgen(js_name = "onBeforeUpdate")]
    pub(crate) fn _on_before_update(getter: &Closure<dyn Fn()>);

    #[wasm_bindgen(js_name = "onBeforeUnmount")]
    pub(crate) fn _on_before_unmount(getter: &Closure<dyn Fn()>);

    #[wasm_bindgen(js_name = "onErrorCaptured")]
    pub(crate) fn _on_error_capture(getter: &Closure<dyn Fn()>);

    #[wasm_bindgen(js_name = "onActivated")]
    pub(crate) fn _on_activated(getter: &Closure<dyn Fn()>);

    #[wasm_bindgen(js_name = "onDeactivated")]
    pub(crate) fn _on_deactivated(getter: &Closure<dyn Fn()>);
}
