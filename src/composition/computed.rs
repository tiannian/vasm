use std::{marker::PhantomData, sync::Arc};

use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::*, throw_val};

use crate::{JsValueWrapper, Result};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "computed")]
    pub(crate) fn _computed(getter: &Closure<dyn Fn() -> JsValue>) -> JsValue;
}

pub struct Computed<T> {
    value: JsValueWrapper,
    closure: Arc<Closure<dyn Fn() -> JsValue>>,
    marker: PhantomData<T>,
}

impl<T> Clone for Computed<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            closure: self.closure.clone(),
            marker: PhantomData,
        }
    }
}

impl<T> Computed<T> {
    pub fn new<F>(getter: F) -> Result<Self>
    where
        T: Serialize,
        F: Fn() -> T + 'static,
    {
        let js_getter = move || -> JsValue {
            let val = getter();

            match JsValue::from_serde(&val) {
                Ok(value) => value,
                Err(e) => throw_val(JsError::from(e).into()),
            }
        };

        let closure = Arc::new(Closure::new(js_getter));

        let value = _computed(&closure);

        let value = JsValueWrapper::from(value);

        Ok(Self {
            closure,
            value,
            marker: PhantomData,
        })
    }

    pub fn to_jsvalue(&self) -> &JsValue {
        &self.value.0
    }

    pub fn get(&self, key: &str) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        self.value.get_value(key)
    }
}
