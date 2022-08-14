use std::marker::PhantomData;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::{JsValueWrapper, Result};

use super::misc;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "ref")]
    pub(crate) fn _ref(val: &JsValue) -> JsValue;
}

pub struct Ref<T> {
    pub(crate) value: JsValueWrapper,
    marker: PhantomData<T>,
}

impl<T> Clone for Ref<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            marker: PhantomData,
        }
    }
}

impl<T> Ref<T> {
    pub fn new(t: &T) -> Result<Self>
    where
        T: Serialize,
    {
        let v = JsValue::from_serde(t)?;

        let value = JsValueWrapper::from(_ref(&v));

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

    pub fn get(&self, key: &str) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        Ok(self.value.get_value(key)?)
    }

    pub fn set(&self, key: &str, value: &T) -> Result<()>
    where
        T: Serialize,
    {
        self.value.set_value(key, value)
    }

    //     pub fn del(&self, key: &str) {
    // // self.value.delete(key);
    //     }
}
