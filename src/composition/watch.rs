use std::{marker::PhantomData, sync::Arc};

use js_sys::Function;
use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::*, throw_val, JsCast};

use crate::{Error, Result};

use super::{FlushOption, Ref};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "watch")]
    pub(crate) fn _watch(val: &JsValue, watcher: &JsValue, options: &JsValue) -> JsValue;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WatchOption {
    pub immediate: bool,
    pub deep: bool,
    pub flush: FlushOption,
}

impl WatchOption {
    pub fn to_js_value(&self) -> Result<JsValue> {
        Ok(JsValue::from_serde(self)?)
    }
}

impl Default for WatchOption {
    fn default() -> Self {
        Self {
            immediate: false,
            deep: false,
            flush: FlushOption::Pre,
        }
    }
}

pub struct Watch<T> {
    pub(crate) stoper: Function,
    #[allow(dead_code)]
    pub(crate) watcher: Arc<Closure<dyn Fn(JsValue, JsValue)>>,
    marker: PhantomData<T>,
}

impl<T> Watch<T>
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    fn new_from_jsvalue<F>(val: JsValue, f: F, opt: Option<WatchOption>) -> Result<Self>
    where
        F: Fn(T, T) + 'static,
    {
        let opt = if let Some(opt) = opt {
            opt
        } else {
            WatchOption::default()
        };

        let watcher = move |a: JsValue, b: JsValue| -> std::result::Result<(), JsError> {
            let a: T = a.into_serde()?;
            let b: T = b.into_serde()?;

            Ok(f(a, b))
        };

        let watcher = move |a, b| {
            if let Err(e) = watcher(a, b) {
                throw_val(e.into())
            }
        };

        let watcher = Arc::new(Closure::new(watcher));

        let stoper = _watch(&val, watcher.as_ref().as_ref(), &opt.to_js_value()?);
        let stoper = stoper
            .dyn_ref::<Function>()
            .ok_or(Error::ConvertFunctionError)?
            .clone();

        Ok(Self {
            watcher,
            stoper,
            marker: PhantomData,
        })
    }

    pub fn new<F>(t: &T, f: F, opt: Option<WatchOption>) -> Result<Self>
    where
        F: Fn(T, T) + 'static,
    {
        let js_value = JsValue::from_serde(t)?;

        Self::new_from_jsvalue(js_value, f, opt)
    }

    pub fn from_ref<F>(t: &Ref<T>, f: F, opt: Option<WatchOption>) -> Result<Self>
    where
        F: Fn(T, T) + 'static,
    {
        let js_value = t.value.clone();

        Self::new_from_jsvalue(js_value.0, f, opt)
    }

    pub fn stop(&self) -> Result<()> {
        self.stoper.call0(&self.stoper)?;
        Ok(())
    }
}
