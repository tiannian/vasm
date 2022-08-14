use std::marker::PhantomData;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::{JsValueWrapper, Result};

use super::{misc, Ref};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "readonly")]
    pub(crate) fn _readonly(val: &JsValue) -> JsValue;
}

pub struct Readonly<T> {
    value: JsValueWrapper,
    marker: PhantomData<T>,
}

impl<T> Clone for Readonly<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            marker: PhantomData,
        }
    }
}

impl<T> Readonly<T> {
    pub fn new(t: &T) -> Result<Self>
    where
        T: Serialize,
    {
        let v = JsValue::from_serde(t)?;

        let value = JsValueWrapper::from(_readonly(&v));

        Ok(Self {
            value,
            marker: PhantomData,
        })
    }

    pub fn to_jsvalue(&self) -> &JsValue {
        &self.value.0
    }

    pub fn from_jsvalue(&self, raw: JsValue) -> Self {
        Self {
            value: JsValueWrapper(raw),
            marker: PhantomData,
        }
    }

    pub fn to_raw(&self) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let val = misc::_to_raw(&self.value.0);

        Ok(val.into_serde()?)
    }

    pub fn from_ref(t: &Ref<T>) -> Result<Self> {
        let value = JsValueWrapper::from(_readonly(t.value.as_ref()));

        Ok(Self {
            value,
            marker: PhantomData,
        })
    }

    pub fn get(&self, key: &str) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        self.value.get_value(key)
    }
}
