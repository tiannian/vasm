use std::rc::Rc;

use js_sys::{Object, Array};
use wasm_bindgen::prelude::*;

use crate::{Result, JsValueWrapper};

pub struct Component {
    pub templete: String,

    pub setup: Closure<dyn Fn(JsValue, JsValue) -> JsValue>,

    pub props: Vec<String>,

    pub events: Vec<String>,

    pub lifetime_hooks: Vec<Rc<Closure<dyn Fn()>>>,

    pub components: Vec<JsValue>,
}

impl Component {
    pub fn to_jsvalue(&self) -> Result<JsValue> {
        let object = JsValueWrapper(Object::new().into());

        object.set("templete", &JsValue::from_str(&self.templete))?;

        object.set("setup", self.setup.as_ref())?;

        let props = Array::new();
        for prop in &self.props {
            let val = JsValue::from_str(&prop);
            props.push(&val);
        }
        object.set("props",&props)?;

        let events = Array::new();
        for event in &self.events {
            let val = JsValue::from_str(&event);
            props.push(&val);
        }
        object.set("events",&events)?;

        let components = Array::new();
        for component in &self.components {
            components.push(component);
        }
        object.set("components", &components)?;

        Ok(object.0)
    }
}
