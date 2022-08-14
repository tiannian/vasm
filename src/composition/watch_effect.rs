use std::sync::Arc;

use js_sys::Function;
use wasm_bindgen::{prelude::*, JsCast};

use crate::{Error, Result};

use super::FlushOption;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "watchEffect")]
    pub(crate) fn _watch_effect(getter: &Closure<dyn Fn()>, flush: &JsValue) -> JsValue;
}

#[derive(Clone)]
pub struct WatchEffect {
    #[allow(dead_code)]
    pub(crate) closure: Arc<Closure<dyn Fn()>>,
    pub(crate) stopper: Function,
}

impl WatchEffect {
    pub fn new<F>(watcher: F) -> Result<Self>
    where
        F: Fn() + 'static,
    {
        Self::new_with_options(watcher, &FlushOption::Pre)
    }

    pub fn new_post<F>(watcher: F) -> Result<Self>
    where
        F: Fn() + 'static,
    {
        Self::new_with_options(watcher, &FlushOption::Post)
    }

    pub fn new_sync<F>(watcher: F) -> Result<Self>
    where
        F: Fn() + 'static,
    {
        Self::new_with_options(watcher, &FlushOption::Sync)
    }

    pub fn new_with_options<F>(watcher: F, opt: &FlushOption) -> Result<Self>
    where
        F: Fn() + 'static,
    {
        let closure = Arc::new(Closure::new(watcher));

        let stopper = _watch_effect(&closure, &opt.to_js_value());

        let stopper: &Function = stopper.dyn_ref().ok_or(Error::ConvertFunctionError)?;

        let stopper = stopper.clone();

        Ok(Self { closure, stopper })
    }

    pub fn stop(&self) -> Result<()> {
        self.stopper.call0(&self.stopper)?;
        Ok(())
    }
}
