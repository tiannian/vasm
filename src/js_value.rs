use js_sys::Reflect;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

use crate::Result;

#[derive(Clone)]
pub struct JsValueWrapper(pub JsValue);

impl JsValueWrapper {
    pub fn get(&self, key: &str) -> Result<JsValue> {
        let key = JsValue::from(key);

        let v = Reflect::get(&self.0, &key)?;

        Ok(v)
    }

    pub fn get_value<T>(&self, key: &str) -> Result<T>
    where
        T: for<'de> Deserialize<'de>,
    {
        let v = self.get(key)?;

        let t = v.into_serde()?;

        Ok(t)
    }

    pub fn set(&self, key: &str, val: &JsValue) -> Result<()> {
        let key = JsValue::from(key);

        Reflect::set(&self.0, &key, val)?;

        Ok(())
    }

    pub fn set_value<T>(&self, key: &str, val: &T) -> Result<()>
    where
        T: Serialize,
    {
        let key = JsValue::from(key);

        let val = JsValue::from_serde(val)?;

        Reflect::set(&self.0, &key, &val)?;

        Ok(())
    }

    //     pub fn delete(&self, key: &str) -> Result<()> {
    // let key = JsValue::from(key);
    //
    // Reflect::delete_property(self.0, &key);
    //
    // Ok(())
    //     }
}

impl AsRef<JsValue> for JsValueWrapper {
    fn as_ref(&self) -> &JsValue {
        &self.0
    }
}

impl AsMut<JsValue> for JsValueWrapper {
    fn as_mut(&mut self) -> &mut JsValue {
        &mut self.0
    }
}

impl From<JsValue> for JsValueWrapper {
    fn from(e: JsValue) -> Self {
        Self(e)
    }
}

impl Into<JsValue> for JsValueWrapper {
    fn into(self) -> JsValue {
        self.0
    }
}
